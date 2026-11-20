#!/usr/bin/env python3
"""
Matter-Autopilot — scheduled, bounded autonomous Matter workstream runner.

Keeps the literal instruction "continúa el proyecto matter" across runs.
Only edits source/doc files in verified workstreams; NO bot activation, protected
branch writes, session migration or privileged shell operations.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations

import argparse
import datetime as dt
import fcntl
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

import matter_ai
import matter_branches

ORDER = tuple(name for name in matter_ai.WORKSTREAMS if name != "bot")
INSTRUCTION = "continúa el proyecto matter"
ROOT = Path.home() / ".local" / "state" / "matter-ai"
CONFIG_FILE = matter_ai.CONFIG_DIR / "autopilot.json"
STATE_FILE = ROOT / "autopilot-state.json"
LOCK_FILE = ROOT / "autopilot.lock"
UNIT_NAME = "matter-autopilot.service"
MAX_SUMMARY = 850
DEFAULT = {
    "schema": 1, "enabled": False, "interval_seconds": 900,
    "daily_cycles": 12, "max_api_calls_per_cycle": 5,
    "auto_push": False, "projects": list(ORDER),
}
VALID_ID = re.compile(r"^[a-z][a-z0-9-]{0,45}$")


def utcnow() -> dt.datetime:
    return dt.datetime.now(dt.timezone.utc)


def initialize_dirs() -> None:
    matter_ai.mkdir_private(ROOT)
    matter_ai.mkdir_private(matter_ai.CONFIG_DIR)


def atomic_json(path: Path, obj: dict) -> None:
    matter_ai.atomic_private_file(path, json.dumps(obj, indent=2, ensure_ascii=False) + "\n")


def config() -> dict:
    if not CONFIG_FILE.is_file() or CONFIG_FILE.is_symlink():
        return dict(DEFAULT)
    document = json.loads(CONFIG_FILE.read_text(encoding="utf-8"))
    if document.get("schema") != 1:
        raise ValueError("versión de configuración no compatible")
    settings = dict(DEFAULT)
    settings.update(document)
    if (not isinstance(settings["interval_seconds"], int)
        or not 60 <= settings["interval_seconds"] <= 86400
        or not isinstance(settings["daily_cycles"], int)
        or not 1 <= settings["daily_cycles"] <= 288
        or not isinstance(settings["max_api_calls_per_cycle"], int)
        or not 1 <= settings["max_api_calls_per_cycle"] <= 12
        or type(settings["enabled"]) is not bool
        or type(settings["auto_push"]) is not bool):
        raise ValueError("configuración fuera de límites")
    projects = settings["projects"]
    if (not isinstance(projects, list) or len(projects) > len(ORDER)
        or any(not isinstance(p, str) or p not in ORDER for p in projects)
        or len(set(projects)) != len(projects)):
        raise ValueError("lista de proyectos inválida")
    return settings


def state() -> dict:
    today = utcnow().date().isoformat()
    default = {
        "schema": 1, "day": today, "daily_used": 0,
        "next_index": 0, "cycles_total": 0,
        "last_project": None, "last_result": "not_started",
        "last_run_utc": None, "next_allowed_utc": None,
        "notes": {}, "pending_push": {},
    }
    if STATE_FILE.is_symlink() or not STATE_FILE.is_file():
        return default
    try:
        stored = json.loads(STATE_FILE.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError):
        # Fail closed: invalid state must not reset the daily budget.
        raise ValueError("estado de autopilot ilegible: revisar archivo manualmente")
    if not isinstance(stored, dict) or stored.get("schema") != 1:
        raise ValueError("estado autopilot no compatible")
    for key in default:
        if key in stored:
            default[key] = stored[key]
    if default["day"] != today:
        default["day"] = today
        default["daily_used"] = 0
        default["next_allowed_utc"] = None
    if not isinstance(default["daily_used"], int) or default["daily_used"] < 0:
        raise ValueError("contador diario inválido")
    if not isinstance(default["next_index"], int) or default["next_index"] < 0:
        raise ValueError("índice de proyecto inválido")
    if not isinstance(default["notes"], dict) or not isinstance(default["pending_push"], dict):
        raise ValueError("estado de continuidad inválido")
    return default


def lock_runner() -> object:
    initialize_dirs()
    lock = LOCK_FILE.open("a+", encoding="utf-8")
    os.chmod(LOCK_FILE, 0o600)
    try:
        fcntl.flock(lock.fileno(), fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        lock.close()
        raise RuntimeError("Ya existe un Matter-Autopilot activo") from None
    return lock


def systemctl(*params: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(["systemctl", "--user", *params], capture_output=True,
                          text=True, check=False, timeout=25)


def eligible(settings: dict, st: dict) -> list[str]:
    available = []
    for project in settings["projects"]:
        folder = matter_ai.ROOT / project
        if not folder.is_dir() or folder.is_symlink():
            continue
        try:
            actual = matter_branches.directory_for(project)
            dirty = matter_branches.required(
                matter_branches.run_git(actual, "status", "--porcelain"), "status"
            )
        except (ValueError, OSError, subprocess.TimeoutExpired):
            continue
        if not dirty:
            available.append(project)
    return available


def safe_summary(value: str) -> str:
    text = matter_ai.redact(value, MAX_SUMMARY)
    # Avoid long identifiers/contact records in local journals and API context.
    text = re.sub(r"\b\+?\d{10,15}\b", "[identificador_oculto]", text)
    text = re.sub(r"\b[^\s@]{1,60}@[a-zA-Z0-9.-]+\.[A-Za-z]{2,}\b",
                  "[correo_oculto]", text)
    return text[-MAX_SUMMARY:]


def read_last_note(st: dict, project: str) -> str:
    value = st["notes"].get(project)
    if isinstance(value, str):
        return value[-MAX_SUMMARY:]
    return ""


def confirm(question: str, expected: str = "SI") -> bool:
    if not sys.stdin.isatty():
        return False
    try:
        value = input(f"{question}\nEscribe {expected}: ").strip()
    except (EOFError, KeyboardInterrupt):
        return False
    return value == expected


def git_commit(project: str, changed: set[str], push: bool) -> dict:
    """Persist only files the agent edited during THIS cycle on the verified branch."""
    root = matter_branches.directory_for(project)
    before = matter_branches.run_git(root, "diff", "--cached", "--name-only")
    if before.returncode or before.stdout:
        raise ValueError("índice Git ocupado; no se publican archivos anteriores")
    actual = matter_branches.files_to_save(root)
    if not actual:
        return {"status": "no_changes"}
    if set(actual) != changed:
        raise ValueError("archivos inesperados: guardar manualmente tras revisión")
    for path in actual:
        # The branch saver independently validates sensitive paths and content.
        matter_branches.safe_file(root, path)
        if not re.fullmatch(r"[\w.\-/]+", path, re.UNICODE):
            raise ValueError("ruta no permitida en guardado automático")
    add = matter_branches.run_git(root, "add", "--", *actual)
    matter_branches.required(add, "git add")
    staged = matter_branches.required(
        matter_branches.run_git(root, "diff", "--cached", "--name-only", "-z"), "index")
    if set(x.decode("utf-8", "replace") for x in staged.split(b"\0") if x) != set(actual):
        raise ValueError("staging inesperado; detener guardado")
    matter_branches.required(
        matter_branches.run_git(root, "diff", "--cached", "--check"), "whitespace")
    # Git hooks are suppressed for unattended source, but no shell command is run.
    message = f"chore(matter-autopilot): advance {project} workstream"
    commit = matter_branches.run_git(root, "-c", "core.hooksPath=/dev/null",
                                    "commit", "-m", message)
    matter_branches.required(commit, "git commit")
    head = matter_branches.required(
        matter_branches.run_git(root, "rev-parse", "HEAD"), "HEAD").decode().strip()
    matter_branches.checkpoint(project, "autopilot_commit_local", actual, head)
    if not push:
        return {"status": "committed_local", "sha": head, "paths": actual}

    # Never push a protected ref or any branch other than this exact workstream.
    branch = matter_ai.WORKSTREAMS[project][1]
    if not branch.startswith("workstreams/"):
        raise ValueError("rama protegida")
    result = matter_branches.run_git(
        root, "-c", "core.hooksPath=/dev/null",
        "push", "origin", "HEAD:refs/heads/" + branch, timeout=150
    )
    if result.returncode:
        return {"status": "push_pending", "sha": head, "paths": actual}
    matter_branches.checkpoint(project, "autopilot_pushed", actual, head)
    return {"status": "pushed", "sha": head, "paths": actual}


def retry_pushes(settings: dict, st: dict) -> None:
    if not settings["auto_push"]:
        return
    for project, sha in list(st["pending_push"].items()):
        if project not in settings["projects"] or project not in ORDER:
            continue
        try:
            root = matter_branches.directory_for(project)
            head = matter_branches.required(
                matter_branches.run_git(root, "rev-parse", "HEAD"), "HEAD").decode().strip()
            if head != sha or not re.fullmatch(r"[0-9a-f]{40}", sha):
                continue
            branch = matter_ai.WORKSTREAMS[project][1]
            result = matter_branches.run_git(
                root, "-c", "core.hooksPath=/dev/null", "push", "origin",
                "HEAD:refs/heads/" + branch, timeout=150
            )
            if result.returncode == 0:
                st["pending_push"].pop(project, None)
                print(f"[Matter] recuperación GitHub exitosa: {project} {sha[:12]}", flush=True)
        except (OSError, ValueError, subprocess.TimeoutExpired):
            continue


def work_prompt(project: str, st: dict) -> str:
    repo, branch = matter_ai.WORKSTREAMS[project]
    note = read_last_note(st, project)
    return (
        INSTRUCTION + "\n"
        f"Proyecto activo de esta ronda: {project} ({repo}; rama {branch}).\n"
        "Lee README, AGENTS y continuidad disponibles, identifica un cambio REAL "
        "pequeño y verificable; si falta información, deja diagnóstico concreto. "
        "Si hay trabajo seguro, modifica sólo archivos permitidos en ESTE proyecto. "
        "No inventes pruebas realizadas; no actives servicios ni roles. "
        "No intentes push, merge, shell, Vault secrets, workflows ni tests que ejecuten código. "
        "Tras tu turno, registra resumen, archivos editados, bloqueo y siguiente microtarea. "
        "Un supervisor validará y guardará cambios por commit en workstreams/*.\n"
        f"Último resumen local de este proyecto (puede ser obsoleto): {note or 'sin avance previo'}\n"
        + matter_ai.autonomy_policy()
    )


def run_cycle(settings: dict, st: dict) -> dict:
    """One bounded API turn. Consume budget BEFORE any external call."""
    if st["daily_used"] >= settings["daily_cycles"]:
        return {"status": "daily_limit"}
    projects = eligible(settings, st)
    if not projects:
        return {"status": "no_clean_workspaces"}
    idx = st["next_index"] % len(projects)
    project = projects[idx]
    st["next_index"] += 1
    st["daily_used"] += 1
    st["cycles_total"] += 1
    st["last_project"] = project
    st["last_run_utc"] = utcnow().isoformat()
    st["last_result"] = "started"
    # Persist budget BEFORE API request; crash never resets the spent-turn count.
    atomic_json(STATE_FILE, st)
    matter_ai.AUTOPILOT_MODE = True
    matter_ai.AUTO_LOCAL_EDITS = True
    matter_ai.LOCAL_EDITS.clear()
    matter_ai.MAX_ACTIONS = settings["max_api_calls_per_cycle"]
    try:
        answer = matter_ai.ask(work_prompt(project, st), [])
        files = set(matter_ai.LOCAL_EDITS.get(project, set()))
        outcome = git_commit(project, files, settings["auto_push"]) if files else {
            "status": "no_changes",
        }
        if outcome.get("status") == "push_pending":
            st["pending_push"][project] = outcome["sha"]
        if outcome.get("status") == "pushed":
            st["pending_push"].pop(project, None)
        st["notes"][project] = safe_summary(answer)
        st["last_result"] = outcome["status"]
        atomic_json(STATE_FILE, st)
        print(f"[Matter] {project}: {outcome['status']}; ciclo {st['daily_used']}/{settings['daily_cycles']}", flush=True)
        return outcome
    except (OSError, RuntimeError, ValueError, subprocess.TimeoutExpired) as error:
        st["last_result"] = "failed_" + type(error).__name__
        st["notes"][project] = "La ronda falló. Revisar archivos locales y estado Git antes de continuar."
        # 429/rate limit: defer at least one hour. Other failures avoid tight retry.
        delay = 3600 if "429" in str(error) else max(300, settings["interval_seconds"])
        st["next_allowed_utc"] = (utcnow() + dt.timedelta(seconds=delay)).isoformat()
        atomic_json(STATE_FILE, st)
        print(f"[Matter] {project}: error {type(error).__name__}; suspensión temporal {delay}s.", flush=True)
        return {"status": "error", "error": type(error).__name__}
    finally:
        matter_ai.AUTO_LOCAL_EDITS = False
        matter_ai.AUTOPILOT_MODE = False
        matter_ai.LOCAL_EDITS.clear()


def seconds_to_wait(settings: dict, st: dict) -> int:
    if st["daily_used"] >= settings["daily_cycles"]:
        tomorrow = dt.datetime.combine(
            utcnow().date() + dt.timedelta(days=1),
            dt.time.min, tzinfo=dt.timezone.utc
        )
        return max(60, int((tomorrow - utcnow()).total_seconds()))
    until = st.get("next_allowed_utc")
    if isinstance(until, str):
        try:
            moment = dt.datetime.fromisoformat(until)
            if moment.tzinfo is not None and moment > utcnow():
                return max(60, int((moment - utcnow()).total_seconds()))
        except ValueError:
            pass
    return settings["interval_seconds"]


def once(offline: bool = False) -> int:
    settings = config()
    with lock_runner():
        st = state()
        if offline:
            projects = eligible(settings, st)
            print(json.dumps({"prompt": INSTRUCTION, "eligible": projects,
                              "used_today": st["daily_used"],
                              "limit": settings["daily_cycles"],
                              "auto_push": settings["auto_push"]},
                             ensure_ascii=False, indent=2))
            return 0
        if not matter_ai.api_key():
            print("Falta API OpenAI: matter-ai setup")
            return 2
        retry_pushes(settings, st)
        atomic_json(STATE_FILE, st)
        result = run_cycle(settings, st)
        return 0 if result["status"] not in {"error", "no_clean_workspaces"} else 1


def loop() -> int:
    settings = config()
    if not settings["enabled"]:
        print("Autopilot deshabilitado; usa matter-autopilot start.")
        return 1
    if not matter_ai.api_key():
        print("Falta API key: matter-ai setup")
        return 2
    with lock_runner():
        print("[Matter] autopilot iniciado; instrucción constante: " + INSTRUCTION, flush=True)
        while True:
            settings = config()
            if not settings["enabled"]:
                print("[Matter] servicio deshabilitado.", flush=True)
                return 0
            st = state()
            retry_pushes(settings, st)
            atomic_json(STATE_FILE, st)
            wait = seconds_to_wait(settings, st)
            if st["daily_used"] < settings["daily_cycles"] and wait <= settings["interval_seconds"]:
                result = run_cycle(settings, st)
                wait = seconds_to_wait(settings, st)
                if result["status"] == "no_clean_workspaces":
                    wait = max(wait, 1800)
            # Break long sleep to process stop/pause/reconfig promptly.
            remaining = wait
            while remaining > 0:
                pause = min(remaining, 30)
                time.sleep(pause)
                remaining -= pause
                if not config()["enabled"]:
                    return 0


def configure(args: argparse.Namespace) -> int:
    old = config()
    projects = [p.strip() for p in args.projects.split(",") if p.strip()] if args.projects else list(ORDER)
    if not projects or any(p not in ORDER for p in projects) or len(set(projects)) != len(projects):
        print("Proyectos inválidos; usa matter-ai projects.")
        return 2
    if not (60 <= args.interval <= 86400 and 1 <= args.daily <= 288 and 1 <= args.calls <= 12):
        print("Fuera de límites: intervalo 60–86400s, ciclos diarios 1–288, llamadas 1–12.")
        return 2
    push = old["auto_push"] if args.push is None else args.push
    if push and not old["auto_push"] and not confirm(
        "Esto AUTORIZA commits/push automáticos a ramas workstreams/*, "
        "sin aprobación por cada cambio. No publica a alpha/beta/stable.",
        "AUTORIZO_PUSH"
    ):
        print("Push automático no autorizado.")
        return 1
    document = {
        "schema": 1, "enabled": old["enabled"],
        "interval_seconds": args.interval, "daily_cycles": args.daily,
        "max_api_calls_per_cycle": args.calls, "auto_push": push,
        "projects": projects,
    }
    initialize_dirs()
    atomic_json(CONFIG_FILE, document)
    print(f"Configurado: intervalo {args.interval}s, ciclos/día {args.daily}, "
          f"pasos API/ciclo {args.calls}, push: {'sí' if push else 'no'}")
    print("Aviso: los límites de ciclos NO son un límite monetario en dólares.")
    return 0


def control(action: str) -> int:
    initialize_dirs()
    cfg = config()
    if action == "start":
        if not matter_ai.api_key():
            print("Falta API key. Usa matter-ai setup antes de iniciar.")
            return 2
        if not confirm(
            "El servicio llamará periódicamente a OpenAI API y podrá editar "
            "archivos en workstreams limpios. Puede generar costes de API."
        ):
            print("Inicio cancelado.")
            return 1
        cfg["enabled"] = True
        atomic_json(CONFIG_FILE, cfg)
        systemctl("daemon-reload")
        result = systemctl("enable", "--now", UNIT_NAME)
    else:
        cfg["enabled"] = False
        atomic_json(CONFIG_FILE, cfg)
        result = systemctl("disable", "--now", UNIT_NAME)
    if result.returncode:
        print("systemctl --user falló; comprueba que systemd user esté disponible.")
        return 1
    print("Servicio", "activo" if action == "start" else "detenido")
    return 0


def status() -> int:
    settings = config()
    st = state()
    running = systemctl("is-active", UNIT_NAME)
    print(json.dumps({
        "instruction": INSTRUCTION,
        "service": running.stdout.strip() if running.returncode == 0 else "inactive",
        "enabled": settings["enabled"],
        "auto_push": settings["auto_push"],
        "interval_seconds": settings["interval_seconds"],
        "daily_cycles": settings["daily_cycles"],
        "daily_used": st["daily_used"],
        "last_project": st["last_project"],
        "last_result": st["last_result"],
        "last_run_utc": st["last_run_utc"],
        "pending_push": sorted(st["pending_push"]),
        "workspaces_ready": eligible(settings, st),
        "budget_notice": "API billing must be capped separately on your OpenAI account",
    }, indent=2, ensure_ascii=False))
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Matter AI — auto-reanudación bajo Sakura")
    subs = parser.add_subparsers(dest="command", required=True)
    opt = subs.add_parser("configure", help="Configure iteration budget and project scope")
    opt.add_argument("--interval", type=int, default=900, metavar="SECONDS")
    opt.add_argument("--daily", type=int, default=12, metavar="CYCLES")
    opt.add_argument("--calls", type=int, default=5, metavar="MAX_STEPS")
    opt.add_argument("--projects", type=str, default="", metavar="CSV")
    push = opt.add_mutually_exclusive_group()
    push.add_argument("--push-workstreams", dest="push", action="store_true")
    push.add_argument("--no-push", dest="push", action="store_false")
    opt.set_defaults(push=None)
    for command in ("start", "stop", "status", "once", "preview", "run"):
        subs.add_parser(command)
    arguments = parser.parse_args()
    try:
        if arguments.command == "configure":
            return configure(arguments)
        if arguments.command in ("start", "stop"):
            return control(arguments.command)
        if arguments.command == "status":
            return status()
        if arguments.command == "preview":
            return once(offline=True)
        if arguments.command == "once":
            return once()
        if arguments.command == "run":
            return loop()
    except (RuntimeError, ValueError, OSError) as error:
        print(f"[Matter] {type(error).__name__}: {str(error)[:200]}", file=sys.stderr)
        return 1
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
