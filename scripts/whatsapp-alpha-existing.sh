#!/usr/bin/env bash
# Base-Matter alpha: resume ONLY the existing isolated alpha WhatsApp session.
# Does not use wa-test-1, does not pair, does not initialize or use Vault.
# Créditos:
# Hecho por Lournyx Matterssmith [Lou-Nyx].
set -Eeuo pipefail
umask 077

CONTROLLER_PID=""
cleanup() {
  if [[ -n "$CONTROLLER_PID" ]] && kill -0 "$CONTROLLER_PID" 2>/dev/null; then
    kill "$CONTROLLER_PID" 2>/dev/null || true
    wait "$CONTROLLER_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

fail() {
  printf 'ALPHA_REFUSED=%s\n' "$1" >&2
  return 1
}

main() {
  if (( $# != 3 )); then
    fail 'Usage: bash scripts/whatsapp-alpha-existing.sh --check|--resume-only /absolute/controller /absolute/provider'
    return 1
  fi
  local mode="$1" controller="$2" provider="$3"
  if [[ "$mode" != "--check" && "$mode" != "--resume-only" ]]; then
    fail 'Only --check and --resume-only are supported'
    return 1
  fi
  if [[ "$controller" != /* || "$provider" != /* ||
        ! -f "$controller" || -L "$controller" || ! -x "$controller" ||
        ! -f "$provider" || -L "$provider" || ! -x "$provider" ]]; then
    fail 'Existing absolute executable binaries required'
    return 1
  fi

  local root state config expected_port vault_var
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
  if [[ "$(git -C "$root" symbolic-ref --quiet --short HEAD)" != "alpha" ]]; then
    fail 'Only alpha branch is allowed'
    return 1
  fi
  state="$HOME/.local/share/base-matter/branches/alpha/whatsapp-test"
  config="$state/runtime.env"
  expected_port="127.0.0.1:39821"
  if [[ -L "$state" || ! -d "$state" ||
        -L "$config" || ! -f "$config" ]]; then
    fail 'Isolated alpha state/config must already exist'
    return 1
  fi
  if [[ "$(stat -c '%a' "$state")" != "700" ||
        "$(stat -c '%a' "$config")" != "600" ||
        "$(stat -c '%u' "$config")" != "$(id -u)" ]]; then
    fail 'Alpha state must be mode 0700; runtime.env owner-only 0600'
    return 1
  fi

  # Source ONLY a pre-existing locally owned private file, never Git content.
  source "$config"
  for vault_var in ${!MATTER_VAULT_@}; do
    unset "$vault_var"
  done
  if [[ "${MATTER_WA_SOCKET_ID:-}" == "wa-test-1" ||
        -z "${MATTER_WA_SOCKET_ID:-}" ||
        "${MATTER_WA_BRIDGE_ADDR:-}" != "$expected_port" ||
        "${MATTER_WA_SESSION_DB:-}" != "$state/session.db" ||
        "${MATTER_WA_ACTIVATION_DB:-}" != "$state/activation.db" ||
        "${MATTER_WA_REPLAY_DB:-}" != "$state/replay.db" ]]; then
    fail 'Refusing original socket, incorrect port, or mixed databases'
    return 1
  fi

  local name file
  for name in session.db activation.db replay.db; do
    file="$state/$name"
    if [[ ! -f "$file" || -L "$file" ||
          "$(stat -c '%u' "$file")" != "$(id -u)" ||
          ! -r "$file" || ! -w "$file" ]]; then
      fail 'Expected locally owned alpha database is missing or unsafe'
      return 1
    fi
  done

  # Provider and controller checks are non-mutating and do not connect.
  "$controller" --check
  "$provider" --check
  printf '%s\n' 'ALPHA_PREFLIGHT=OK' 'ALPHA_VAULT=ISOLATED_NOT_ENROLLED' 'ALPHA_PAIRING=DISABLED'

  if [[ "$mode" == "--check" ]]; then
    return 0
  fi

  # SQLite backup API snapshots the consistent committed DB, even with WAL.
  # All snapshot files remain private on this host; none are sent to GitHub.
  local backup_root snapshot
  backup_root="$state/offline-backups"
  if [[ -L "$backup_root" ]]; then
    fail 'Unsafe backup directory'
    return 1
  fi
  mkdir -p -m 700 "$backup_root"
  if [[ "$(stat -c '%a' "$backup_root")" != "700" ]]; then
    fail 'Backup directory not private'
    return 1
  fi
  snapshot="$backup_root/before-resume-$(date -u +%Y%m%dT%H%M%SZ)-$$"
  mkdir -m 700 "$snapshot"

  if ! python3 - "$state" "$snapshot" <<'PY'
import sqlite3
import sys
from pathlib import Path

source_dir = Path(sys.argv[1])
destination_dir = Path(sys.argv[2])
try:
    for name in ("session.db", "activation.db", "replay.db"):
        src = source_dir / name
        dst = destination_dir / name
        with sqlite3.connect(src.as_uri() + "?mode=ro", uri=True) as source:
            source.execute("PRAGMA query_only=ON")
            if source.execute("PRAGMA quick_check(1)").fetchone() != ("ok",):
                raise RuntimeError("integrity")
            with sqlite3.connect(dst) as destination:
                source.backup(destination)
                if destination.execute("PRAGMA quick_check(1)").fetchone() != ("ok",):
                    raise RuntimeError("snapshot_integrity")
except (sqlite3.Error, OSError, RuntimeError):
    print("ALPHA_SNAPSHOT=ERROR")
    raise SystemExit(1)
print("ALPHA_SNAPSHOT=VERIFIED_PRIVATE")
PY
  then
    fail 'Snapshot failed: no WhatsApp connection attempted'
    return 1
  fi

  if [[ -L "$state/alpha-resume-controller.log" ]]; then
    fail 'Unsafe controller log path'
    return 1
  fi
  "$controller" --serve >>"$state/alpha-resume-controller.log" 2>&1 </dev/null &
  CONTROLLER_PID=$!
  sleep 1
  if ! kill -0 "$CONTROLLER_PID" 2>/dev/null; then
    fail 'Alpha controller did not start (private log retained)'
    return 1
  fi
  printf '%s\n' 'ALPHA_CONTROLLER=STARTED' 'ALPHA_PROVIDER=RESUME_ONLY'
  # The existing alpha device is the only store opened; no pair-code request.
  "$provider" --resume-only
}

main "$@"
