#!/usr/bin/env python3
"""
Matter-AI — agente de consola para workstreams Matter, sin Codex.

Solo Python estándar + HTTPS. Las llamadas a OpenAI usan Responses API.
Los procesos y archivos se limitan a workspaces locales verificados.
Las operaciones de Git y sistema con efectos relevantes requieren aprobación.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations

import argparse
import getpass
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import urllib.error
import urllib.request

HOME = Path.home()
CONFIG_DIR = HOME / ".config" / "matter-ai"
CONFIG_PATH = CONFIG_DIR / "config.json"
KEY_PATH = CONFIG_DIR / "api.key"
HANDOFF_PATH = HOME / ".local" / "share" / "matter-ai" / "MATTER_AI_HANDOFF.md"
ROOT = HOME / "Matter" / "workspaces"
API_URL = "https://api.openai.com/v1/responses"
MODEL_DEFAULT = "gpt-6.2-sol"
MAX_ACTIONS = 18
AUTO_LOCAL_EDITS = False

# Mapa público. Credenciales de GitHub y Vault NUNCA están en este archivo.
WORKSTREAMS = {
    "orchestration": ("Server-Matter", "workstreams/01-orchestration"),
    "base": ("Base-Matter", "workstreams/02-base"),
    "vault": ("Vault-Matter", "workstreams/03-vault"),
    "docs": ("Docs-Matter", "workstreams/04-docs"),
    "system": ("System-Matter", "workstreams/05-system"),
    "msel": ("System-Matter", "workstreams/06-msel"),
    "matter-rust": ("Base-Matter", "workstreams/07-matter-rust"),
    "register-ui": ("Base-Matter", "workstreams/08-register-ui"),
    "register-gateway": ("Vault-Matter", "workstreams/09-register-gateway"),
    "whatsapp": ("WhatsApp-Matter", "workstreams/10-whatsapp"),
    "whatsmeow-compat": ("WhatsApp-Matter", "workstreams/11-whatsmeow-compat"),
    "telegram": ("Telegram-Matter", "workstreams/12-telegram"),
    "discord": ("Discord-Matter", "workstreams/13-discord"),
    "api": ("API-Matter", "workstreams/14-api"),
    "mcp": ("MCP-Matter", "workstreams/15-mcp"),
    "ai": ("AI-Matter", "workstreams/16-ai"),
    "automation": ("Automation-Matter", "workstreams/17-automation"),
    "sdk": ("SDK-Matter", "workstreams/18-sdk"),
    "extensions": ("Extensions-Matter", "workstreams/19-extensions"),
    "interfaces": ("Interfaces-Matter", "workstreams/20-interfaces"),
    "web": ("Web-Matter", "workstreams/21-web"),
    "social": ("Social-Matter", "workstreams/22-social"),
    "marketplace": ("Marketplace-Matter", "workstreams/23-marketplace"),
    "app": ("App-Matter", "workstreams/24-app"),
    "cloud": ("Cloud-Matter", "workstreams/25-cloud"),
    "os": ("OS-Matter", "workstreams/26-os"),
    "bot": ("Bot-Matter", "workstreams/27-bot"),
}

SECRET_FILE_RE = re.compile(
    r"(^|/)(\.env($|\.)|\.ssh|\.gnupg|\.aws|\.git|api\.key|"
    r"credentials?($|\.)|secrets?($|\.)|id_ed25519|id_rsa|"
    r"session($|/)|sessions($|/)|vault/data($|/))",
    re.IGNORECASE,
)
SECRET_FRAGMENT_RE = re.compile(
    r"(OPENAI_API_KEY|GH_TOKEN|GITHUB_TOKEN|PRIVATE_KEY|"
    r"BEGIN [A-Z ]*PRIVATE KEY|whatsapp.*session)",
    re.IGNORECASE,
)
SAFE_READ = {"status", "diff", "log", "branch", "rev-parse"}
SAFE_RUN = {
    "cargo": {"check", "test", "fmt", "clippy", "metadata"},
    "python3": {"-m"},
    "python": {"-m"},
    "pytest": set(),
    "rustc": {"--version"},
}
DANGEROUS_COMMANDS = {
    "sudo", "su", "doas", "rm", "dd", "mkfs", "mount", "umount", "shutdown",
    "reboot", "systemctl", "service", "chown", "chmod", "curl", "wget", "bash",
    "sh", "pwsh", "powershell", "docker", "podman", "node", "npm", "npx",
    "python3 -c", "python -c",
}


def mkdir_private(path: Path) -> None:
    path.mkdir(parents=True, exist_ok=True)
    os.chmod(path, 0o700)


def atomic_private_file(path: Path, data: str) -> None:
    mkdir_private(path.parent)
    temp = path.parent / (path.name + ".tmp-" + str(os.getpid()))
    fd = os.open(temp, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
    try:
        with os.fdopen(fd, "w", encoding="utf-8") as file:
            file.write(data)
        os.replace(temp, path)
        os.chmod(path, 0o600)
    finally:
        if temp.exists():
            temp.unlink()


def config() -> dict:
    if CONFIG_PATH.exists():
        return json.loads(CONFIG_PATH.read_text(encoding="utf-8"))
    return {"model": MODEL_DEFAULT}


def api_key() -> str | None:
    key = os.environ.get("OPENAI_API_KEY", "").strip()
    if key:
        return key
    if KEY_PATH.exists():
        if (KEY_PATH.stat().st_mode & 0o077) != 0:
            raise RuntimeError("Clave insegura: permisos distintos de 0600.")
        return KEY_PATH.read_text(encoding="utf-8").strip()
    return None


def setup() -> int:
    print("Matter-AI · configuración local. API OpenAI: facturación separada de ChatGPT.")
    print("La clave se introduce sin mostrar caracteres; nunca se sube a GitHub.")
    key = getpass.getpass("OpenAI API key (deja vacío para configurar después): ").strip()
    if key:
        if not key.startswith("sk-") or len(key) < 20:
            print("Clave de API con formato no reconocido; no guardada.")
            return 2
        atomic_private_file(KEY_PATH, key + "\n")
        print("Clave guardada con permisos 0600.")
    cfg = config()
    model = input(f"Modelo [{cfg.get('model', MODEL_DEFAULT)}]: ").strip()
    if model:
        if not re.fullmatch(r"[a-zA-Z0-9._-]{3,80}", model):
            print("Nombre de modelo inválido.")
            return 2
        cfg["model"] = model
    atomic_private_file(CONFIG_PATH, json.dumps(cfg, indent=2) + "\n")
    mkdir_private(ROOT)
    print("Configuración lista. Ejecuta matter-ai o matter-ai sync matter-rust.")
    return 0


def project_path(project: str) -> Path:
    if project not in WORKSTREAMS:
        raise ValueError("workstream_desconocido")
    path = ROOT / project
    if path.is_symlink() or not path.is_dir():
        raise ValueError("workstream_no_descargado: matter-ai sync " + project)
    if not (path / ".git").exists():
        raise ValueError("no_es_repositorio_git")
    root = path.resolve(strict=True)
    if not root.is_relative_to(ROOT.resolve()):
        raise ValueError("ruta_fuera_de_workspace")
    expected = WORKSTREAMS[project][1]
    active = run(["git", "branch", "--show-current"], root, 15)
    if active.get("returncode") != 0 or active.get("output", "").strip() != expected:
        raise ValueError("rama_local_incorrecta; se bloquean operaciones hasta revisar Git")
    return root


def file_path(project: str, relative_path: str) -> Path:
    if not relative_path or Path(relative_path).is_absolute():
        raise ValueError("ruta_invalida")
    parts = Path(relative_path).parts
    if any(p in {"..", "."} for p in parts):
        raise ValueError("ruta_fuera_de_workspace")
    norm = "/".join(parts)
    if SECRET_FILE_RE.search(norm):
        raise ValueError("ruta_confidencial_prohibida")
    root = project_path(project)
    path = (root / relative_path).resolve(strict=False)
    if not path.is_relative_to(root):
        raise ValueError("ruta_fuera_de_workspace")
    return path


def redact(text: str, limit: int = 8000) -> str:
    if SECRET_FRAGMENT_RE.search(text):
        # Do not forward command outputs containing apparent credentials.
        return "[Salida ocultada por posible información privada]"
    return text[-limit:]


def run(argv: list[str], cwd: Path | None = None, timeout: int = 90) -> dict:
    try:
        result = subprocess.run(
            argv, cwd=cwd, text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
            timeout=min(max(timeout, 1), 180), check=False,
            env={k: v for k, v in os.environ.items()
                 if k not in {"OPENAI_API_KEY", "GH_TOKEN", "GITHUB_TOKEN"}},
        )
        return {"returncode": result.returncode, "output": redact(result.stdout)}
    except (OSError, subprocess.TimeoutExpired) as error:
        return {"error": type(error).__name__}


def sync_one(project: str) -> bool:
    if project == "bot":
        print("Bot-Matter está bloqueado. Se permite clonar solo para leer su documentación.")
    repo, branch = WORKSTREAMS[project]
    mkdir_private(ROOT)
    destination = ROOT / project
    if destination.exists() or destination.is_symlink():
        if not destination.is_dir() or not (destination / ".git").exists():
            print(f"{project}: destino existente no es clon Git. No se toca.")
            return False
        current = run(["git", "branch", "--show-current"], destination)
        if current.get("output", "").strip() != branch:
            print(f"{project}: rama local diferente; sin cambios para proteger el trabajo.")
            return False
        check = run(["git", "status", "--porcelain"], destination)
        if check.get("returncode") != 0:
            print(f"{project}: no se puede leer estado Git.")
            return False
        if check.get("output", "").strip():
            print(f"{project}: cambios locales sin guardar. Se omite fetch/sync.")
            return False
        result = run(["git", "fetch", "origin", branch], destination, 150)
        print(f"{project}: fetch {'OK' if result.get('returncode') == 0 else 'ERROR'} (no se hace reset ni pull).")
        return result.get("returncode") == 0
    url = f"https://github.com/Matterssmith/{repo}.git"
    print(f"{project}: clonando {repo} / {branch} (sin escribir credenciales)…")
    result = run(["git", "clone", "--single-branch", "--depth", "1",
                  "--branch", branch, url, str(destination)], ROOT, 180)
    if result.get("returncode") != 0:
        print(f"{project}: Git no pudo clonar; revisa conexión y credenciales privadas.")
        return False
    print(f"{project}: listo.")
    return True


def sync(project: str) -> int:
    if not shutil.which("git"):
        print("Instala git primero; se requiere Git para descargar los repositorios.")
        return 2
    if project == "all":
        print("Clonar todos puede utilizar varios GB. Repositorios privados requieren autenticación Git.")
        if not approve("¿Descargar hasta 27 workspaces independientes?"):
            return 1
        count = sum(sync_one(name) for name in WORKSTREAMS if name != "bot")
        print(f"Descargas verificadas: {count}/{len(WORKSTREAMS) - 1}. Bot-Matter bloqueado.")
        return 0 if count == len(WORKSTREAMS) - 1 else 1
    if project not in WORKSTREAMS:
        print("Proyecto desconocido. Ejecuta matter-ai projects.")
        return 2
    return 0 if sync_one(project) else 1


def approve(question: str) -> bool:
    if AUTO_LOCAL_EDITS and question.startswith("¿Escribir archivo "):
        return True
    try:
        answer = input(f"\n[CONFIRMAR] {question}\nEscribe SI para autorizar: ").strip()
    except (EOFError, KeyboardInterrupt):
        return False
    return answer == "SI"


def list_files(project: str, relative: str) -> dict:
    path = file_path(project, relative) if relative else project_path(project)
    if not path.is_dir():
        return {"error": "no_es_directorio"}
    entries = []
    for child in sorted(path.iterdir(), key=lambda p: p.name)[:120]:
        if SECRET_FILE_RE.search(child.name) or child.is_symlink():
            continue
        entries.append({"name": child.name, "type": "dir" if child.is_dir() else "file"})
    return {"entries": entries}


def read_file(project: str, relative: str, offset: int = 0) -> dict:
    path = file_path(project, relative)
    if not path.is_file() or path.stat().st_size > 2000000:
        return {"error": "fichero_ausente_o_demasiado_grande"}
    if not isinstance(offset, int) or offset < 0 or offset > 2000000:
        return {"error": "offset_invalido"}
    with path.open("r", encoding="utf-8", errors="replace") as file:
        file.seek(offset)
        contents = file.read(16000)
        end_offset = file.tell()
        next_part = file.read(1)
    if SECRET_FRAGMENT_RE.search(contents):
        return {"error": "posible_secreto_detectado"}
    return {"content": contents, "path": relative, "next_offset": end_offset if next_part else None}


def write_file(project: str, relative: str, contents: str) -> dict:
    if project == "bot":
        return {"error": "bot_matter_bloqueado"}
    path = file_path(project, relative)
    if len(contents) > 250000:
        return {"error": "contenido_demasiado_grande"}
    if SECRET_FRAGMENT_RE.search(contents):
        return {"error": "contiene_posibles_secretos"}
    if path.exists() and not path.is_file():
        return {"error": "destino_no_es_fichero"}
    if path.exists() and not os.access(path, os.W_OK):
        return {"error": "fichero_no_modificable"}
    if not approve(f"¿Escribir archivo {project}/{relative}?"):
        return {"error": "no_autorizado_por_operador"}
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(contents, encoding="utf-8")
    return {"ok": True, "path": relative, "bytes": len(contents.encode("utf-8"))}


def run_workstream(project: str, argv: list[str]) -> dict:
    if project == "bot":
        return {"error": "bot_matter_bloqueado"}
    cwd = project_path(project)
    if not isinstance(argv, list) or not argv or len(argv) > 50:
        return {"error": "argumentos_invalidos"}
    if not all(isinstance(x, str) and len(x) < 500 for x in argv):
        return {"error": "argumentos_invalidos"}
    command = Path(argv[0]).name
    if command in DANGEROUS_COMMANDS or "/" in argv[0]:
        return {"error": "comando_restringido"}
    # No shell=True, semicolons and pipes are just regular arguments.
    if command == "git":
        if len(argv) < 2:
            return {"error": "subcomando_git_requerido"}
        operation = argv[1]
        if operation in {"reset", "clean", "rebase", "push", "pull", "fetch", "checkout",
                         "switch", "merge", "tag", "branch", "config", "remote",
                         "add", "commit"}:
            if operation in {"reset", "clean", "rebase", "push", "pull", "checkout",
                             "switch", "merge", "tag", "config", "remote"}:
                return {"error": "operacion_git_restringida; utiliza workflow revisado"}
            if operation in {"add", "commit", "fetch", "branch"}:
                if not approve(f"¿Ejecutar git {operation} en {project}? {' '.join(argv[2:])[:120]}"):
                    return {"error": "no_autorizado_por_operador"}
        elif operation not in SAFE_READ:
            return {"error": "subcomando_git_no_permitido"}
    elif command in SAFE_RUN:
        if SAFE_RUN[command] and (len(argv) < 2 or argv[1] not in SAFE_RUN[command]):
            return {"error": "subcomando_no_permitido"}
        if command in {"python3", "python"} and (
            len(argv) < 3 or argv[2] not in {"unittest", "pytest", "compileall"}
        ):
            return {"error": "modulo_python_no_permitido"}
        if not approve(f"¿Ejecutar prueba/compilación {project}: {shlex.join(argv)[:160]}?"):
            return {"error": "no_autorizado_por_operador"}
    else:
        return {"error": "ejecutable_no_permitido"}
    return run(argv, cwd, 180)


TOOLS = [
    {
        "type": "function", "name": "list_workstreams",
        "description": "Show known Matter projects, branch and local Git checkout status.",
        "parameters": {"type": "object", "properties": {}, "required": [],
                       "additionalProperties": False},
        "strict": True,
    },
    {
        "type": "function", "name": "list_files",
        "description": "List allowed files in a local checked-out workstream directory.",
        "parameters": {
            "type": "object", "properties": {
                "project": {"type": "string"}, "path": {"type": "string"}
            }, "required": ["project", "path"], "additionalProperties": False,
        }, "strict": True,
    },
    {
        "type": "function", "name": "read_file",
        "description": "Read a 16k character chunk from a source file or README; supply offset=next_offset to continue reading.",
        "parameters": {
            "type": "object", "properties": {
                "project": {"type": "string"}, "path": {"type": "string"},
                "offset": {"type": "integer", "minimum": 0}
            }, "required": ["project", "path", "offset"], "additionalProperties": False,
        }, "strict": True,
    },
    {
        "type": "function", "name": "write_file",
        "description": "Replace a complete source file in local checkout after explicit confirmation.",
        "parameters": {
            "type": "object", "properties": {
                "project": {"type": "string"}, "path": {"type": "string"},
                "content": {"type": "string"}
            }, "required": ["project", "path", "content"], "additionalProperties": False,
        }, "strict": True,
    },
    {
        "type": "function", "name": "run_command",
        "description": "Execute strictly limited Git status/diff or Rust/Python tests in project after approval.",
        "parameters": {
            "type": "object", "properties": {
                "project": {"type": "string"},
                "argv": {"type": "array", "items": {"type": "string"}},
            }, "required": ["project", "argv"], "additionalProperties": False,
        }, "strict": True,
    },
]

SYSTEM = """Eres Matter-AI, un asistente para el ecosistema Matterssmith en Ubuntu Server.
Responde en español, directo. NO eres esta misma sesión de ChatGPT; usa OpenAI API.
Tu fuente de verdad es el contenido de los repositorios descargados y su rama.
No inventes implementación, estados, pruebas ni IDs. Distingue pendiente vs real.
Créditos de archivos nuevos: Hecho por Lournyx Matterssmith [Lou-Nyx].
Respeta AGENTS.md, README.md, docs/WORK_CONTINUITY.md de cada repo antes de editar.
Python/C++/Rust son principales; Go y JS son interoperabilidad. Respeta MSEL.
Bot-Matter sigue bloqueado. No abrir ni publicar secretos de Vault.
No modifiques live sessions, no migres ni reinicies sockets sin revisión manual.
Trabaja en workstreams/*, nunca en main/matter/alpha/stable. No auto-push.
Para editar primero lee fuentes; usa write_file con archivo completo.
No afirmes que ejecutaste algo cuando la herramienta lo negó.
Si falta el repositorio, pide: matter-ai sync NOMBRE.
En cada respuesta final reporta resumen, archivos, pruebas, pendientes y siguiente etapa.
"""


def project_status() -> list[dict]:
    rows = []
    for name, (repo, branch) in WORKSTREAMS.items():
        path = ROOT / name
        cloned = path.is_dir() and not path.is_symlink() and (path / ".git").exists()
        rows.append({"id": name, "repo": repo, "branch": branch,
                     "cloned": bool(cloned), "blocked": name == "bot"})
    return rows


def dispatch(name: str, args: dict) -> dict:
    try:
        if name == "list_workstreams":
            return {"workstreams": project_status()}
        if name == "list_files":
            return list_files(args["project"], args["path"])
        if name == "read_file":
            return read_file(args["project"], args["path"], args.get("offset", 0))
        if name == "write_file":
            return write_file(args["project"], args["path"], args["content"])
        if name == "run_command":
            return run_workstream(args["project"], args["argv"])
        return {"error": "herramienta_desconocida"}
    except (ValueError, OSError, UnicodeError) as err:
        return {"error": str(err)[:200]}


def call_openai(payload: dict) -> dict:
    key = api_key()
    if not key:
        raise RuntimeError("Falta API key. Ejecuta: matter-ai setup")
    request = urllib.request.Request(
        API_URL, data=json.dumps(payload, ensure_ascii=False).encode("utf-8"),
        headers={"Authorization": "Bearer " + key, "Content-Type": "application/json",
                 "User-Agent": "Matter-AI/0.1"}, method="POST",
    )
    try:
        with urllib.request.urlopen(request, timeout=120) as response:
            return json.load(response)
    except urllib.error.HTTPError as error:
        # Avoid returning response bodies that might echo credentials.
        raise RuntimeError(f"OpenAI HTTP {error.code}. Revisa API billing/modelo.") from None
    except urllib.error.URLError:
        raise RuntimeError("No hay conexión con OpenAI API.") from None


def ask(prompt: str, history: list[dict]) -> str:
    payload_input = [{"role": "user", "content": prompt}]
    if HANDOFF_PATH.is_file() and not HANDOFF_PATH.is_symlink():
        # Project status, not conversation memory. Capped for API cost.
        handoff = HANDOFF_PATH.read_text(encoding="utf-8")[:9000]
        payload_input.insert(0, {"role": "user",
                                "content": "Documento de traspaso Matter (revisar contra GitHub):\n" + handoff})
    # Compact context: never persist raw terminal output or credentials.
    for item in history[-4:]:
        payload_input.insert(-1, {"role": "user", "content": item["user"]})
        payload_input.insert(-1, {"role": "assistant", "content": item["assistant"]})
    for step in range(MAX_ACTIONS):
        response = call_openai({
            "model": config().get("model", MODEL_DEFAULT),
            "instructions": SYSTEM,
            "input": payload_input,
            "tools": TOOLS,
            "store": False,
            "max_output_tokens": 3500,
            "parallel_tool_calls": False,
        })
        output = response.get("output", [])
        payload_input.extend(output)
        calls = [x for x in output if x.get("type") == "function_call"]
        if not calls:
            answer = response.get("output_text") or "\n".join(
                block.get("text", "")
                for msg in output if msg.get("type") == "message"
                for block in msg.get("content", []) if block.get("type") == "output_text"
            )
            return answer.strip() or "No hubo respuesta de texto. Intenta otra instrucción."
        for action in calls:
            try:
                args = json.loads(action.get("arguments", "{}"))
                result = dispatch(action.get("name", ""), args)
            except (ValueError, KeyError, TypeError) as error:
                result = {"error": "argumentos_invalidos", "type": type(error).__name__}
            print(f"  • {action.get('name', 'función')}: {result.get('error', 'ok')}")
            payload_input.append({
                "type": "function_call_output",
                "call_id": action["call_id"],
                "output": json.dumps(result, ensure_ascii=False),
            })
    return "Se alcanzó el límite por turno. Los cambios locales permanecen; revisa Git y continúa."


def interactive() -> int:
    if not api_key():
        print("Antes de conversar: matter-ai setup")
        return 2
    print("Matter-AI · terminal interactiva · /salir para terminar")
    print("Traspaso Matter:", "cargado" if HANDOFF_PATH.is_file() else "no disponible")
    print("Repositorios: matter-ai projects · Descargar: matter-ai sync matter-rust")
    print("Modo opcional /auto on para no aprobar cada edición local; /auto off lo desactiva.")
    history: list[dict] = []
    while True:
        try:
            question = input("\nMatter> ").strip()
        except (EOFError, KeyboardInterrupt):
            print("\nConsola cerrada.")
            return 0
        if question in {"/salir", "salir", "/exit", "/quit"}:
            return 0
        global AUTO_LOCAL_EDITS
        if question == "/auto on":
            if approve("¿Permitir cambios de archivos locales sin confirmar cada archivo?"):
                AUTO_LOCAL_EDITS = True
                print("Modo local automático ACTIVADO. Sin push ni comandos peligrosos.")
            continue
        if question == "/auto off":
            AUTO_LOCAL_EDITS = False
            print("Modo local automático desactivado.")
            continue
        if not question:
            continue
        try:
            answer = ask(question, history)
            print("\n" + answer)
            history.append({"user": question, "assistant": answer[:4000]})
        except (RuntimeError, OSError, ValueError) as error:
            print("Error: " + str(error))


def main() -> int:
    parser = argparse.ArgumentParser(description="Matter-AI terminal multi-repositorios, sin Codex")
    parser.add_argument("action", nargs="?", default="chat",
                        help="chat, setup, plan, projects, status, sync, model, o una orden de texto")
    parser.add_argument("rest", nargs="*")
    args = parser.parse_args()
    if args.action == "setup":
        return setup()
    if args.action == "plan":
        if HANDOFF_PATH.is_file():
            print(HANDOFF_PATH.read_text(encoding="utf-8"))
            return 0
        print("No existe documento de traspaso. Reinstala desde la rama workstreams/02-base.")
        return 2
    if args.action == "projects" or args.action == "status":
        for item in project_status():
            label = "local" if item["cloned"] else "pendiente"
            if item["blocked"]:
                label += " / BLOQUEADO"
            print(f"{item['id']:20} {label:22} {item['branch']}")
        return 0
    if args.action == "sync":
        return sync(args.rest[0] if args.rest else "matter-rust")
    if args.action == "model":
        if not args.rest:
            print(config().get("model", MODEL_DEFAULT))
            return 0
        model = args.rest[0]
        if not re.fullmatch(r"[a-zA-Z0-9._-]{3,80}", model):
            print("Modelo inválido")
            return 2
        cfg = config()
        cfg["model"] = model
        atomic_private_file(CONFIG_PATH, json.dumps(cfg, indent=2) + "\n")
        print(f"Modelo: {model}")
        return 0
    if args.action == "chat":
        return interactive()
    if args.action == "help":
        parser.print_help()
        return 0
    question = " ".join([args.action, *args.rest])
    try:
        print(ask(question, []))
        return 0
    except (RuntimeError, OSError, ValueError) as error:
        print("Error: " + str(error))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
