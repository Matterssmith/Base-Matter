#!/usr/bin/env python3
"""Isolated Base-Matter development worker; fixed Unix IPC, no WhatsApp credentials."""
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
import json
import os
import re
import signal
import socket
import socketserver
import stat
import struct
import subprocess
import threading
import time
from pathlib import Path

ROOT = Path("/var/lib/matter-codex")
JOBS = ROOT / "jobs"
WORK = ROOT / "work"
SOCKET = Path("/run/matter-codex/worker.sock")
REPO = "Matterssmith/Base-Matter"
URL = "https://github.com/" + REPO + ".git"
MAX = 4096
LOCK = threading.Lock()
JOB_ID = re.compile(r"^[0-9a-f]{32}$")
TERMINAL = {"ready", "failed", "no_changes", "closed"}

def run(args, cwd=None, timeout=90, data=None, capture=False, agent=False):
    env = os.environ.copy()
    env["GIT_TERMINAL_PROMPT"] = "0"
    if agent:
        # GitHub publication auth is not passed as environment to the model.
        for key in list(env):
            if key.startswith(("GH_", "GITHUB_", "MATTER_VAULT_", "MATTER_WA_", "SSH_")):
                env.pop(key, None)
        env["GIT_CONFIG_GLOBAL"] = "/dev/null"
        env["GIT_CONFIG_SYSTEM"] = "/dev/null"
    child = subprocess.Popen(
        args, cwd=cwd, env=env, stdin=subprocess.PIPE if data is not None else subprocess.DEVNULL,
        stdout=subprocess.PIPE if capture else subprocess.DEVNULL,
        stderr=subprocess.DEVNULL, start_new_session=True,
    )
    try:
        out, _ = child.communicate(data, timeout=timeout)
    except subprocess.TimeoutExpired:
        os.killpg(child.pid, signal.SIGKILL)
        child.communicate()
        raise RuntimeError("timeout")
    if child.returncode:
        raise RuntimeError("command_failed")
    return out or b""

def git(work, *args, capture=False, timeout=90):
    return run(["git", "-c", "core.hooksPath=/dev/null", *args], work,
               timeout=timeout, capture=capture)

def job_path(id):
    if not isinstance(id, str) or not JOB_ID.fullmatch(id):
        raise ValueError("invalid_id")
    return JOBS / (id + ".json")

def read_job(id):
    path = job_path(id)
    if not path.exists():
        return {"state": "missing"}
    return json.loads(path.read_text())

def save_job(id, value):
    path = job_path(id)
    temp = path.with_suffix(".tmp")
    temp.write_text(json.dumps(value, ensure_ascii=False))
    os.chmod(temp, 0o600)
    os.replace(temp, path)

def public(job):
    return {key: job[key] for key in ("state", "pr", "files", "commit") if key in job}

def safe_file(name):
    parts = name.lower().split("/")
    return (
        name and len(name) <= 240 and not any(ord(c) < 32 for c in name)
        and not name.startswith((".github/", ".git", "scripts/matter-codex", "deploy/systemd/"))
        and not any(p in {".env", "auth.json", "session.db", "runtime.env", "credentials"} for p in parts)
        and not name.lower().endswith((".key", ".pem", ".p12", ".db", ".sqlite", ".sqlite3"))
    )

def execute(id, instruction):
    work = WORK / id
    result = {"state": "running"}
    with LOCK:
        save_job(id, result)
    try:
        if work.exists():
            raise RuntimeError("existing_checkout")
        run(["git", "clone", "--single-branch", "--branch", "alpha", URL, str(work)], timeout=180)
        before = git(work, "rev-parse", "HEAD", capture=True).decode().strip()
        prompt = (
            "Implement the authorized owner task in this isolated Base-Matter checkout. "
            "Do not access host files, credentials, WhatsApp, Vault, or other repositories. "
            "Do not push, create commits, modify CI workflows or deployment credentials. "
            "Preserve the Matter architecture, owner checks and alpha safety gates. "
            "Review both README files, update relevant docs and add tests. "
            "Use only files in this checkout. Task:\n" + instruction
        )
        run(["codex", "exec", "--sandbox", "workspace-write", "--ask-for-approval", "never", "-"],
            work, timeout=1800, data=prompt.encode(), agent=True)
        if git(work, "rev-parse", "HEAD", capture=True).decode().strip() != before:
            raise RuntimeError("agent_changed_head")
        if git(work, "remote", "get-url", "origin", capture=True).decode().strip() != URL:
            raise RuntimeError("agent_changed_remote")
        git(work, "add", "-A")
        names = git(work, "diff", "--cached", "--name-only", "-z", capture=True).decode().split("\0")
        names = [name for name in names if name]
        if not names:
            result = {"state": "no_changes"}
        else:
            if len(names) > 100 or not all(safe_file(name) for name in names):
                raise RuntimeError("unsafe_changes")
            for name in names:
                path = work / name
                if path.is_symlink() or (path.exists() and path.stat().st_size > 2_000_000):
                    raise RuntimeError("unsafe_file")
            # Fixed checks; no unreviewed code is executed with publisher credentials.
            # Compilation/tests run in the same sandboxed agent, then independently in CI.
            branch = "codex/" + id
            git(work, "switch", "-c", branch)
            git(work, "-c", "user.name=Lou-Nyx", "-c",
                "user.email=apellidos05.nombre05@gmail.com",
                "commit", "-m", "feat(codex): authorized development task")
            commit = git(work, "rev-parse", "HEAD", capture=True).decode().strip()
            git(work, "push", URL, "HEAD:refs/heads/" + branch, timeout=180)
            link = run(["gh", "pr", "create", "--repo", REPO, "--base", "alpha", "--head", branch,
                        "--title", "Matter Codex development task", "--body",
                        "Automated candidate from the isolated worker. Review changes and CI before merging. No automatic merge."],
                       work, timeout=90, capture=True).decode().strip()
            match = re.fullmatch(r"https://github\.com/Matterssmith/Base-Matter/pull/([1-9][0-9]*)", link)
            if not match:
                raise RuntimeError("pr_not_confirmed")
            result = {"state": "review", "pr": int(match.group(1)), "commit": commit, "files": names[:5]}
    except (OSError, ValueError, RuntimeError, UnicodeError):
        result = {"state": "failed"}
    with LOCK:
        save_job(id, result)

def refresh_pr(id, job):
    try:
        raw = run(["gh", "pr", "view", str(job["pr"]), "--repo", REPO,
                   "--json", "state,baseRefName,mergeCommit"], timeout=25, capture=True)
        value = json.loads(raw)
        if value.get("baseRefName") != "alpha":
            return
        if value.get("state") == "MERGED" and value.get("mergeCommit", {}).get("oid"):
            job["state"] = "ready"
            job["commit"] = value["mergeCommit"]["oid"]
        elif value.get("state") == "CLOSED":
            job["state"] = "closed"
        else:
            return
        with LOCK:
            save_job(id, job)
    except (OSError, RuntimeError, ValueError, TypeError, KeyError):
        return

def work_loop():
    while True:
        for path in sorted(JOBS.glob("*.json")):
            id = path.stem
            if not JOB_ID.fullmatch(id):
                continue
            try:
                with LOCK:
                    job = read_job(id)
                if job["state"] == "queued":
                    execute(id, job["instruction"])
                elif job["state"] == "review":
                    refresh_pr(id, job)
                elif job["state"] == "running":
                    # A prior process disappeared; never repeat an uncertain job.
                    with LOCK:
                        save_job(id, {"state": "failed"})
            except (OSError, ValueError, KeyError):
                continue
        time.sleep(15)

class Handler(socketserver.StreamRequestHandler):
    def handle(self):
        self.request.settimeout(3)
        try:
            _, uid, _ = struct.unpack("3i", self.request.getsockopt(
                socket.SOL_SOCKET, socket.SO_PEERCRED, struct.calcsize("3i")))
            if uid != self.server.allowed_uid:
                return
            raw = self.rfile.read(MAX + 1)
            if len(raw) > MAX:
                return
            value = json.loads(raw)
            op = value.get("op")
            with LOCK:
                if op == "ping" and set(value) == {"op"}:
                    reply = {"state": "idle"}
                elif op == "status" and set(value) == {"op", "id"}:
                    reply = public(read_job(value["id"]))
                elif op == "submit" and set(value) == {"op", "id", "instruction"}:
                    id = value["id"]
                    instruction = value["instruction"]
                    if not isinstance(instruction, str) or not 1 <= len(instruction.encode()) <= 1200:
                        return
                    job = read_job(id)
                    if job["state"] == "missing":
                        # No overlapping model tasks; review remains tracked separately.
                        active = any(read_job(p.stem)["state"] in {"queued", "running"}
                                     for p in JOBS.glob("*.json") if JOB_ID.fullmatch(p.stem))
                        if active:
                            reply = {"state": "failed"}
                        else:
                            save_job(id, {"state": "queued", "instruction": instruction})
                            reply = {"state": "queued"}
                    else:
                        reply = public(job)
                else:
                    return
            self.wfile.write(json.dumps(reply).encode())
        except (OSError, ValueError, KeyError, TypeError, AttributeError):
            return

def main():
    os.umask(0o077)
    uid = int(os.environ["MATTER_CODEX_BOT_UID"])
    if uid <= 0 or uid == os.getuid() or os.getuid() == 0:
        raise RuntimeError("separate_nonroot_worker_required")
    JOBS.mkdir(mode=0o700, parents=True, exist_ok=True)
    WORK.mkdir(mode=0o700, parents=True, exist_ok=True)
    if SOCKET.exists() or SOCKET.is_symlink():
        if not stat.S_ISSOCK(SOCKET.lstat().st_mode):
            raise RuntimeError("invalid_socket")
        SOCKET.unlink()
    with socketserver.UnixStreamServer(str(SOCKET), Handler) as server:
        server.allowed_uid = uid
        os.chmod(SOCKET, 0o660)
        threading.Thread(target=work_loop, daemon=True).start()
        server.serve_forever()

if __name__ == "__main__":
    main()
