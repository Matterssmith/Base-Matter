#!/usr/bin/env python3
"""
Matter branch lifecycle: checkpoint, save, branch inventory, reviewed promotions.
No force-push, reset, automatic merges, or direct writes to stable/alpha/beta.
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations

import datetime
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile

import matter_ai

CHECKPOINT_ROOT = Path.home() / "Matter" / "checkpoints"
ALLOWED_BASES = {"alpha", "beta", "stable", "matter", "main"}
BRANCH_NAME = re.compile(r"^[A-Za-z0-9][A-Za-z0-9._/-]{0,125}$")
BLOCK_PARTS = {".git", ".ssh", ".env", ".aws", ".gnupg", ".venv",
               "node_modules", "target", "session", "sessions", "credentials", "secrets"}
BLOCK_SUFFIXES = (".key", ".pem", ".p12", ".pfx", ".db", ".sqlite", ".session")
TOKEN_PATTERN = re.compile(
    r"-----BEGIN [A-Z ]*PRIVATE KEY-----|sk-[A-Za-z0-9_-]{20,}"
    r"|gh[pousr]_[A-Za-z0-9_]{25,}|github_pat_[A-Za-z0-9_]{25,}",
    re.IGNORECASE,
)


def run_git(directory: Path, *arguments: str, timeout: int = 120) -> subprocess.CompletedProcess[bytes]:
    env = {k: v for k, v in os.environ.items()
           if k not in {"OPENAI_API_KEY", "GH_TOKEN", "GITHUB_TOKEN"}}
    return subprocess.run(
        ["git", *arguments], cwd=directory, capture_output=True,
        timeout=timeout, check=False, env=env
    )


def required(result: subprocess.CompletedProcess[bytes], operation: str) -> bytes:
    if result.returncode:
        raise ValueError("Git no completó " + operation + "; revisa conexión/permisos.")
    return result.stdout


def directory_for(project: str) -> Path:
    if project == "bot":
        raise ValueError("Bot-Matter sigue bloqueado.")
    root = matter_ai.project_path(project)
    repo, branch = matter_ai.WORKSTREAMS[project]
    actual = required(run_git(root, "remote", "get-url", "origin"), "remote").decode().strip().lower()
    allowed = {
        f"https://github.com/Matterssmith/{repo}.git".lower(),
        f"https://github.com/Matterssmith/{repo}".lower(),
        f"git@github.com:Matterssmith/{repo}.git".lower()
    }
    if actual not in allowed:
        raise ValueError("Remoto distinto a Matterssmith, rechazado.")
    if not branch.startswith("workstreams/"):
        raise ValueError("Solo se guardan workstreams verificados.")
    return root


def safe_file(root: Path, name: str) -> None:
    if not name or Path(name).is_absolute() or "\x00" in name:
        raise ValueError("Ruta inválida.")
    parts = name.replace("\\", "/").split("/")
    if any(p in BLOCK_PARTS or p.startswith(".env") or p in {".", ".."} for p in parts):
        raise ValueError("Ruta confidencial o traversal bloqueado.")
    if name.lower().endswith(BLOCK_SUFFIXES) or name.startswith("vault/") or name.startswith(".github/"):
        raise ValueError("Archivo protegido: " + name)
    resolved_root = root.resolve()
    path = resolved_root.joinpath(*parts)
    if path.is_symlink() or not path.resolve().is_relative_to(resolved_root):
        raise ValueError("Enlace o ruta fuera del checkout.")
    if not path.is_file() or path.stat().st_size > 2_000_000:
        raise ValueError("Archivo eliminado/binario/grande: " + name)
    try:
        content = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        raise ValueError("Solo archivos de texto: " + name) from None
    if TOKEN_PATTERN.search(content):
        raise ValueError("Posible secreto en archivo: " + name)


def files_to_save(root: Path) -> list[str]:
    dirty = required(run_git(root, "diff", "--name-only", "--diff-filter=ACMRT", "-z", "HEAD"), "diff")
    untracked = required(run_git(root, "ls-files", "--others", "--exclude-standard", "-z"), "ls-files")
    removed = required(run_git(root, "diff", "--name-only", "--diff-filter=D", "-z", "HEAD"), "deleted")
    if removed:
        raise ValueError("Hay archivos eliminados: revisión separada requerida.")
    names = sorted({
        name.decode("utf-8", "surrogateescape")
        for data in (dirty, untracked)
        for name in data.split(b"\0") if name
    })
    if len(names) > 100:
        raise ValueError("Más de 100 archivos; divide el guardado.")
    for name in names:
        safe_file(root, name)
    return names


def confirm(message: str) -> bool:
    if not sys.stdin.isatty():
        return False
    try:
        return input("\n" + message + "\nEscribe SI para continuar: ").strip() == "SI"
    except (EOFError, KeyboardInterrupt):
        return False


def checkpoint(project: str, stage: str, names: list[str], sha: str | None = None) -> Path:
    root = directory_for(project)
    current = required(run_git(root, "rev-parse", "HEAD"), "rev-parse").decode().strip()
    now = datetime.datetime.now(datetime.timezone.utc)
    target = CHECKPOINT_ROOT / project
    target.mkdir(mode=0o700, parents=True, exist_ok=True)
    os.chmod(target, 0o700)
    record = {
        "schema": 1, "time_utc": now.isoformat(), "workstream": project,
        "branch": matter_ai.WORKSTREAMS[project][1],
        "local_sha": current, "stage": stage, "files": names,
        "published_sha": sha, "credits": "Hecho por Lournyx Matterssmith [Lou-Nyx].",
        "warning": "Solo metadatos. Los archivos se respaldan realmente mediante commit+push."
    }
    with tempfile.NamedTemporaryFile("w", encoding="utf-8", dir=target, delete=False) as handle:
        temp = Path(handle.name)
        json.dump(record, handle, indent=2, ensure_ascii=False)
        handle.write("\n")
    try:
        os.chmod(temp, 0o600)
        final = target / (now.strftime("%Y%m%dT%H%M%SZ") + "-" + os.urandom(4).hex() + ".json")
        os.replace(temp, final)
        return final
    finally:
        if temp.exists():
            temp.unlink()


def status(project: str) -> int:
    root = directory_for(project)
    head = required(run_git(root, "rev-parse", "--short", "HEAD"), "HEAD").decode().strip()
    print(project, "branch", matter_ai.WORKSTREAMS[project][1], "HEAD", head)
    raw = required(run_git(root, "status", "--short"), "status")
    print(raw.decode("utf-8", "replace")[:3000] or "Sin cambios locales.")
    return 0


def branches(project: str) -> int:
    root = directory_for(project)
    response = required(run_git(root, "ls-remote", "--heads", "origin"), "ls-remote")
    refs = sorted(
        line.split(b"refs/heads/", 1)[1].decode("utf-8", "replace")
        for line in response.splitlines() if b"refs/heads/" in line
    )
    print("Ramas remotas:", len(refs))
    print("\n".join(refs[:120]))
    return 0


def save(project: str, local_only: bool = False) -> int:
    # Vault documentation/source may be saved after review; ciphertext and
    # secret-bearing vault/ paths remain excluded by safe_file().
    root = directory_for(project)
    if required(run_git(root, "diff", "--cached", "--name-only", "-z"), "staged"):
        raise ValueError("Hay archivos staged anteriores; resolver antes.")
    names = files_to_save(root)
    if not names:
        print("Sin cambios; no se requiere commit.")
        return 0
    print("Checkpoint inicial:", checkpoint(project, "working-tree", names))
    print("Archivos:", "\n".join(names))
    branch = matter_ai.WORKSTREAMS[project][1]
    if not confirm("¿Crear commit en " + branch + " con estos archivos?"):
        print("No autorizado, nada modificado.")
        return 1
    required(run_git(root, "add", "--", *names), "add")
    staged = required(run_git(root, "diff", "--cached", "--name-only", "-z"), "staged")
    indexed = {item.decode("utf-8", "replace") for item in staged.split(b"\0") if item}
    if indexed != set(names):
        raise ValueError("Índice Git inesperado; commit detenido.")
    required(run_git(root, "diff", "--cached", "--check"), "diff-check")
    required(run_git(root, "var", "GIT_AUTHOR_IDENT"), "identidad de autor")
    commit = run_git(root, "commit", "-m", f"checkpoint({project}): preserve Matter progress")
    required(commit, "commit")
    head = required(run_git(root, "rev-parse", "HEAD"), "HEAD").decode().strip()
    print("Commit local:", head[:12])
    print("Checkpoint:", checkpoint(project, "committed", names, head))
    if local_only:
        print("Solo local: NO respaldado en GitHub.")
        return 0
    if not confirm("¿Publicar commit en origin/" + branch + "? Sin forzar."):
        print("Commit local conservado. No respaldado en GitHub.")
        return 0
    pushed = run_git(root, "push", "origin", "HEAD:refs/heads/" + branch, timeout=150)
    if pushed.returncode:
        print("Push rechazado; commit local conservado sin sobreescribir nada.")
        return 1
    print("Respaldo GitHub confirmado, commit", head[:12])
    print("Checkpoint:", checkpoint(project, "pushed", names, head))
    return 0


def promote(project: str, base: str) -> int:
    if not BRANCH_NAME.fullmatch(base) or ".." in base or "//" in base:
        raise ValueError("Rama destino inválida.")
    if base not in ALLOWED_BASES and not base.startswith(("feat/", "release/", "hotfix/")):
        raise ValueError("Destino debe ser alpha/beta/stable/matter/main o rama especial.")
    root = directory_for(project)
    branch = matter_ai.WORKSTREAMS[project][1]
    if base == branch:
        raise ValueError("El destino no puede ser la misma rama.")
    if required(run_git(root, "status", "--porcelain"), "status"):
        raise ValueError("Hay cambios locales; guarda primero.")
    required(run_git(root, "ls-remote", "--exit-code", "--heads", "origin",
                     "refs/heads/" + base), "revisión de rama destino")
    required(run_git(root, "ls-remote", "--exit-code", "--heads", "origin",
                     "refs/heads/" + branch), "revisión de rama origen")
    if not shutil.which("gh"):
        raise ValueError("Falta GitHub CLI; configurar gh auth login.")
    if not confirm("¿Abrir PR DRAFT: " + branch + " → " + base + "? Sin merge."):
        print("PR no autorizado.")
        return 1
    title = "[Matter] " + project + ": promoción a " + base
    body = (
        "Workstream: " + project + "\nOrigen: " + branch + "\nDestino: " + base + "\n\n"
        "Revisar CI, pruebas y permisos. Merge exclusivamente tras autorización.\n\n"
        "Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx]."
    )
    env = {k: v for k, v in os.environ.items() if k != "OPENAI_API_KEY"}
    result = subprocess.run(
        ["gh", "pr", "create", "--draft", "--base", base, "--head", branch,
         "--title", title, "--body", body],
        cwd=root, env=env, capture_output=True, timeout=110, check=False
    )
    if result.returncode:
        print("PR falló; comprueba gh auth y PR existente.")
        return 1
    response = result.stdout.decode("utf-8", "replace").strip()
    repo = matter_ai.WORKSTREAMS[project][0]
    expected = "https://github.com/Matterssmith/" + repo + "/pull/"
    print(response if response.startswith(expected) else "PR draft creado.")
    return 0


def execute(action: str, parameters: list[str]) -> int:
    if not parameters:
        print("Indica proyecto: matter-ai save matter-rust")
        return 2
    project = parameters[0]
    if project not in matter_ai.WORKSTREAMS:
        print("Proyecto desconocido. Ejecuta matter-ai projects.")
        return 2
    try:
        if action == "workstatus":
            return status(project)
        if action == "branches":
            return branches(project)
        if action == "checkpoint":
            root = directory_for(project)
            names = files_to_save(root)
            print("Checkpoint de metadatos:", checkpoint(project, "working-tree", names))
            return 0
        if action == "save":
            return save(project, "--local" in parameters[1:])
        if action == "promote":
            if len(parameters) != 2:
                print("Uso: matter-ai promote PROYECTO alpha|beta|stable|feat/RAMA")
                return 2
            return promote(project, parameters[1])
        print("Comando no reconocido.")
        return 2
    except (ValueError, OSError, subprocess.TimeoutExpired) as issue:
        print("No se completó:", str(issue))
        return 1


if __name__ == "__main__":
    raise SystemExit(execute(sys.argv[1], sys.argv[2:]))
