#!/usr/bin/env bash
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
# Update tracked alpha source only. Never stop or build running binaries.
set -u
umask 077
main() {
  local root before remote local_count remote_count running changed
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)" || return 1
  cd "$root" || return 1
  export GIT_TERMINAL_PROMPT=0
  if [[ "$(git symbolic-ref --quiet --short HEAD)" != alpha ]] || ! git diff --quiet HEAD --; then
    printf '%s\n' 'Estado: Bloqueado' 'Motivo: rama distinta de alpha o cambios rastreados pendientes.'
    return 20
  fi
  before="$(git rev-parse HEAD)" || return 1
  if ! timeout 60s git fetch --no-tags origin refs/heads/alpha:refs/remotes/origin/alpha >/dev/null 2>&1; then
    printf '%s\n' 'Estado: Fallido' 'Motivo: no se pudo consultar origin/alpha.'
    return 1
  fi
  remote="$(git rev-parse refs/remotes/origin/alpha)" || return 1
  local_count="$(git rev-list --count "$remote..$before")" || return 1
  remote_count="$(git rev-list --count "$before..$remote")" || return 1
  running="${MATTER_WA_RUNNING_REVISION:-unknown}"
  if [[ "$local_count" != 0 ]]; then
    printf 'Estado: Bloqueado\nCommits locales pendientes: %s\nCommits remotos pendientes: %s\nNo se modificó el código.\n' "$local_count" "$remote_count"
    return 20
  fi
  if [[ "$before" == "$remote" ]]; then
    if [[ "$running" == "$before" ]]; then
      printf 'Estado: Actualizado\nVersión: %.7s\nCambios pendientes: 0\n' "$before"
    else
      printf 'Estado: Sin actualizaciones\nVersión: %.7s\nCambios pendientes: 0\nActivación: pendiente.\n' "$before"
    fi
    return 10
  fi
  changed="$(git diff --name-only "$before" "$remote" | wc -l)" || return 1
  if ! git merge --ff-only "$remote" >/dev/null 2>&1; then
    printf '%s\n' 'Estado: Fallido' 'Motivo: Git rechazó la actualización; revisar cambios locales.'
    return 1
  fi
  if [[ "$(git rev-parse HEAD)" != "$remote" ]] || ! git diff --quiet HEAD --; then
    printf '%s\n' 'Estado: Fallido' 'Motivo: revisión final no verificable.'
    return 1
  fi
  printf 'Estado: Código actualizado\nVersión: %.7s → %.7s\nCommits aplicados: %s\nArchivos modificados: %s\nCambios (máx. 5):\n' "$before" "$remote" "$remote_count" "$changed"
  git --no-pager log --format='%h %s' -n 5 "$before..$remote" | cut -c 1-100 | while IFS= read -r line; do
    printf '• %s\n' "$line"
  done
  printf 'Versión en ejecución: %.7s\n' "$running"
}
main "$@"
