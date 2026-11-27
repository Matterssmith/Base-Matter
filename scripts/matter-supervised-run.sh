#!/usr/bin/env bash
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
# Fixed alpha supervisor. Update is source-only; restart activates it.
set -u
umask 077

main() {
  local root status branch revision next state result request verified
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)" || return 1
  cd "$root" || return 1
  branch="$(git symbolic-ref --quiet --short HEAD)" || return 1
  if [[ "$branch" != alpha ]]; then
    printf '%s\n' 'MATTER_MAINTENANCE=REFUSED_BRANCH' >&2
    return 1
  fi
  state="$HOME/.local/share/base-matter/branches/alpha/whatsapp-test"
  request="$state/maintenance-restart.pending.json"
  result="$state/maintenance-restart.result"
  export MATTER_WA_SUPERVISED=1
  export GIT_TERMINAL_PROMPT=0
  while true; do
    revision="$(git rev-parse HEAD)" || return 1
    export MATTER_WA_RUNNING_REVISION="$revision"
    status=0
    bash "$root/scripts/matter-verified-run.sh" || status=$?
    case "$status" in
      75)
        printf '%s\n' 'MATTER_RESTART=AUTHORIZED'
        if [[ ! -f "$request" || -L "$request" || -L "$result" ]]; then
          printf '%s\n' 'MATTER_RESTART=REFUSED_MISSING_REPORT' >&2
          return 1
        fi
        next="$(git rev-parse HEAD)" || return 1
        verified=verified-reused
        if [[ "$revision" != "$next" ]]; then
          if ! bash "$root/scripts/matter-verified-run.sh" --build-only; then
            printf '%s\n' 'build-failed' > "$result"
            printf '%s\n' 'MATTER_RESTART=FAILED_BUILD' >&2
            return 1
          fi
          verified=verified-built
        fi
        if ! bash "$root/scripts/matter-verified-run.sh" --check-only; then
          printf '%s\n' 'start-failed' > "$result"
          printf '%s\n' 'MATTER_RESTART=FAILED_PREFLIGHT' >&2
          return 1
        fi
        printf '%s\n' "$verified" > "$result"
        printf '%s\n' 'MATTER_RESTART=VERIFIED_WAITING_CONNECTION'
        ;;
      76)
        printf '%s\n' 'MATTER_UPDATE=LEGACY_PROVIDER_REQUIRES_MIGRATION' >&2
        return 1
        ;;
      *)
        if [[ "$status" != 0 && -f "$request" && ! -L "$request" && ! -L "$result" && "$(cat "$result" 2>/dev/null)" == verified* ]]; then
          printf '%s\n' 'start-failed' > "$result"
        fi
        return "$status"
        ;;
    esac
  done
}
main "$@"
