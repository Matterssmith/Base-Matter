#!/usr/bin/env python3
"""Matter-Base tracked-file, documentation and release hygiene audit.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations

import pathlib
import re
import subprocess
import sys
from urllib.parse import unquote, urlsplit


ROOT = pathlib.Path(__file__).resolve().parent.parent
CREDIT = "Hecho por Lournyx Matterssmith [Lou-Nyx]."
LINK = re.compile(r"!?\[[^\]]*\]\(([^)]+)\)")
FORBIDDEN_EXTENSIONS = {".pem", ".p12", ".pfx", ".jks", ".keystore", ".sqlite", ".db"}
FORBIDDEN_NAMES = {
    ".env",
    "id_rsa",
    "id_ed25519",
    "credentials.json",
    "service-account.json",
    "session.json",
}


def tracked_files() -> list[pathlib.Path]:
    command = subprocess.run(
        ["git", "ls-files", "-z"],
        cwd=ROOT,
        check=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return [pathlib.Path(name.decode("utf-8")) for name in command.stdout.split(b"\0") if name]


def verify() -> list[str]:
    errors: list[str] = []
    files = tracked_files()
    readmes = [path for path in files if path.name.lower() == "readme.md"]
    if not readmes:
        errors.append("Missing tracked README.md")
    if pathlib.Path("LICENSE") not in files:
        errors.append("Missing tracked LICENSE for the declared MIT license")

    for path in files:
        filename = path.name.lower()
        if filename in FORBIDDEN_NAMES or path.suffix.lower() in FORBIDDEN_EXTENSIONS:
            errors.append(f"Sensitive file must not be tracked: {path}")
        if any(part.lower() in {"private-keys", "sessions", "secrets"} for part in path.parts):
            errors.append(f"Sensitive directory must not be tracked: {path}")

    for path in readmes:
        content = (ROOT / path).read_text(encoding="utf-8")
        if CREDIT not in content:
            errors.append(f"Missing required project credit in {path}")
        for raw in LINK.findall(content):
            address = raw.strip()
            if address.startswith("<") and address.endswith(">"):
                address = address[1:-1]
            parsed = urlsplit(address)
            if parsed.scheme or parsed.netloc or address.startswith("#"):
                continue
            relative = unquote(parsed.path)
            if not relative:
                continue
            destination = (ROOT / path.parent / relative).resolve()
            if not destination.is_relative_to(ROOT):
                errors.append(f"README link escapes the repository: {path} -> {address}")
            elif not destination.exists():
                errors.append(f"Missing README link target: {path} -> {address}")
        print(f"README verified: {path}")

    print(
        f"Audit: tracked={len(files)}, readmes={len(readmes)}, "
        f"violations={len(errors)}"
    )
    return errors


def main() -> int:
    try:
        errors = verify()
    except (OSError, subprocess.CalledProcessError, UnicodeError) as exc:
        print(f"Repository audit failed: {exc}", file=sys.stderr)
        return 1
    for error in errors:
        print(f"ERROR: {error}", file=sys.stderr)
    return 1 if errors else 0


if __name__ == "__main__":
    sys.exit(main())
