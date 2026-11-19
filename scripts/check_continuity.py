#!/usr/bin/env python3
"""Enforce per-commit durable instruction and continuity records.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
"""
from __future__ import annotations
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
REQUIRED = (
    "AGENTS.md",
    "docs/DECISION_LOG.md",
    "docs/WORK_CONTINUITY.md",
    "docs/DEVELOPMENT_STATUS.md",
)


def main() -> int:
    try:
        changed = set(subprocess.check_output(
            ["git", "diff", "--name-only", "HEAD^", "HEAD"],
            cwd=ROOT, text=True,
        ).splitlines())
    except (OSError, subprocess.CalledProcessError) as exc:
        print(f"Cannot check continuity: {exc}", file=sys.stderr)
        return 1
    missing = set(REQUIRED) - changed
    if missing:
        print("Continuity files must change in every commit:", ", ".join(sorted(missing)), file=sys.stderr)
        return 1
    for relative in REQUIRED:
        if not (ROOT / relative).is_file():
            print(f"Missing continuity document: {relative}", file=sys.stderr)
            return 1
    state = (ROOT / "docs/WORK_CONTINUITY.md").read_text(encoding="utf-8")
    if "README revisados: 2/2" not in state:
        print("Document the review of both README files", file=sys.stderr)
        return 1
    print("Per-commit continuity records verified (4/4); README review noted.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
