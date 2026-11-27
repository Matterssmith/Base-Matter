#!/usr/bin/env bash
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
# Fixed alpha maintenance supervisor. Never pair, escalate or execute chat text.
set -u
umask 077

main() {
  local root status branch
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)" || return 1
  cd "$root" || return 1
  branch="$(git symbolic-ref --quiet --short HEAD)" || return 1
  if [[ "$branch" != alpha ]]; then
    printf '%s\n' 'MATTER_MAINTENANCE=REFUSED_BRANCH' >&2
    return 1
  fi
  export MATTER_WA_SUPERVISED=1
  export GIT_TERMINAL_PROMPT=0
  while true; do
    status=0
    bash "$root/scripts/matter-verified-run.sh" || status=$?
    case "$status" in
      75)
        printf '%s\n' 'MATTER_RESTART=AUTHORIZED'
        ;;
      76)
        printf '%s\n' 'MATTER_UPDATE=AUTHORIZED'
        if ! git diff --quiet HEAD -- || [[ "$(git symbolic-ref --quiet --short HEAD)" != alpha ]]; then
          printf '%s\n' 'MATTER_UPDATE=REFUSED_DIRTY_OR_WRONG_BRANCH' >&2
          return 1
        fi
        git pull --ff-only origin alpha || return 1
        if [[ "$(git rev-parse HEAD)" != "$(git rev-parse refs/remotes/origin/alpha)" ]]; then
          printf '%s\n' 'MATTER_UPDATE=REFUSED_DIVERGENT_REVISION' >&2
          return 1
        fi
        bash "$root/scripts/matter-verified-run.sh" --build-only || return 1
        printf '%s\n' 'MATTER_UPDATE=VERIFIED'
        ;;
      *) return "$status" ;;
    esac
  done
}

main "$@"
