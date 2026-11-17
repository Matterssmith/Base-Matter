#!/usr/bin/env python3
"""Safe GitHub repository-name migration planner.

No changes are made without an explicit --apply flag. The operator supplies
their own authenticated GitHub CLI session; tokens are never read or logged.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations

import argparse
import json
import subprocess
import sys

ORG = "Matterssmith"

RENAMES = {
    "Matter-Base": "Base-Matter",
    "Matter-System": "System-Matter",
    "Matter-Interfaces": "Interfaces-Matter",
    "Matter-Baileys": "WhatsApp-Matter",
    "Matter-Telegraf": "Telegram-Matter",
    "Matter-Discord": "Discord-Matter",
    "Matter-Extensions": "Extensions-Matter",
    "Matter-SDK": "SDK-Matter",
    "Matter-Docs": "Docs-Matter",
    "Matter-AI": "AI-Matter",
    "Matter-API": "API-Matter",
    "Matter-MCP": "MCP-Matter",
    "Matter-Bot": "Bot-Matter",
    "Matter-Web": "Web-Matter",
    "Matter-Automation": "Automation-Matter",
    "Matter-Cloud": "Cloud-Matter",
    "Matter-Marketplace": "Marketplace-Matter",
    "Matter-Server": "Server-Matter",
    "Matter-App": "App-Matter",
    "Matter-OS": "OS-Matter",
    "Matter-Vault": "Vault-Matter",
    "Matter-Outdated": "Outdated-Matter",
}


def execute(*args: str) -> str:
    result = subprocess.run(
        ["gh", *args],
        check=True,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    return result.stdout.strip()


def actual_repo(repo: str) -> str | None:
    try:
        output = execute("repo", "view", repo, "--json", "nameWithOwner")
        return json.loads(output)["nameWithOwner"]
    except (OSError, subprocess.CalledProcessError, KeyError, ValueError):
        return None


def plan(selected: list[str]) -> list[tuple[str, str]]:
    invalid = set(selected) - set(RENAMES)
    if invalid:
        raise ValueError("Unknown repository alias: " + ", ".join(sorted(invalid)))
    names = selected if selected else list(RENAMES)
    return [(name, RENAMES[name]) for name in names]


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apply", action="store_true", help="Perform approved renames")
    selection = parser.add_mutually_exclusive_group()
    selection.add_argument("--all", action="store_true", help="Explicitly include the full ecosystem")
    selection.add_argument("--only", action="append", default=[], metavar="OLD_REPO")
    args = parser.parse_args(argv)

    if args.apply and not (args.all or args.only):
        parser.error("Refusing to rename without --only NAME or --all")
    if not args.apply and (args.all or args.only):
        print("Preview mode: no remote changes will be made.")
    try:
        changes = plan(args.only)
    except ValueError as error:
        parser.error(str(error))

    for old, new in changes:
        print(f"{ORG}/{old} -> {ORG}/{new}")

    if not args.apply:
        print(f"{len(changes)} planned names; dry-run only.")
        return 0

    try:
        execute("auth", "status")
    except (OSError, subprocess.CalledProcessError):
        print("GitHub CLI auth is not available; no repositories renamed.", file=sys.stderr)
        return 1

    for old, new in changes:
        old_name = f"{ORG}/{old}"
        new_name = f"{ORG}/{new}"
        actual = actual_repo(old_name)
        if actual == new_name:
            print(f"ALREADY RENAMED: {new_name}")
            continue
        if actual != old_name:
            print(f"STOP: cannot verify original repository: {old_name}", file=sys.stderr)
            return 1
        if actual_repo(new_name) == new_name:
            print(f"STOP: destination name already exists: {new_name}", file=sys.stderr)
            return 1
        try:
            execute("repo", "rename", "--repo", old_name, new, "--yes")
        except (OSError, subprocess.CalledProcessError):
            print(f"STOP: rename failed for {old_name}", file=sys.stderr)
            return 1
        if actual_repo(new_name) != new_name:
            print(f"STOP: cannot verify new repository name {new_name}", file=sys.stderr)
            return 1
        print(f"CONFIRMED: {new_name}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
