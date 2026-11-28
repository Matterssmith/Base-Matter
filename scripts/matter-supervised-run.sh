#!/usr/bin/env bash
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
# Fixed alpha supervisor with immutable, private verified runtime recovery.
set -u
umask 077

safe_dir() {
  [[ -d "$1" && ! -L "$1" && "$(stat -c '%a:%u' "$1")" == "700:$(id -u)" ]]
}

safe_file() {
  [[ -f "$1" && ! -L "$1" && "$(stat -c '%u' "$1")" == "$(id -u)" ]]
}

no_runtime() {
  ! pgrep -f '[w]hatsapp-matter-control.*--serve|[w]hatsapp-matter-rust-provider.*--resume-only' >/dev/null 2>&1
}

valid_revision() {
  [[ "$1" =~ ^[0-9a-f]{40}$ ]]
}

# A manifest of independently copied, verified binaries and the launcher.
# Never copy the WhatsApp session, Vault key, runtime.env or mutable DB here.
snapshot_valid() {
  local dir="$1" sha
  safe_dir "$dir" && safe_file "$dir/revision" && safe_file "$dir/SHA256SUMS" &&
    safe_file "$dir/whatsapp-matter-control" && safe_file "$dir/whatsapp-matter-rust-provider" &&
    safe_file "$dir/launcher.sh" || return 1
  sha="$(cat "$dir/revision")" || return 1
  valid_revision "$sha" && [[ "$(basename "$dir")" == "$sha" ]] || return 1
  [[ -x "$dir/whatsapp-matter-control" && -x "$dir/whatsapp-matter-rust-provider" ]] || return 1
  # Only compare the fixed list; do not execute arbitrary manifest paths.
  local actual
  actual="$(cd "$dir" && sha256sum -- revision whatsapp-matter-control whatsapp-matter-rust-provider launcher.sh)" || return 1
  [[ "$actual" == "$(cat "$dir/SHA256SUMS")" ]]
}

atomic_text() {
  local file="$1" value="$2" temporary
  [[ ! -L "$file" ]] || return 1
  temporary="$(mktemp "${file}.XXXXXX")" || return 1
  if ! printf '%s\n' "$value" > "$temporary" || ! mv -T -- "$temporary" "$file"; then
    rm -f -- "$temporary"
    return 1
  fi
}

save_snapshot() {
  local root="$1" recovery="$2" revision="$3" target provider stamp dir temp actual
  valid_revision "$revision" || return 1
  dir="$recovery/$revision"
  if [[ -e "$dir" || -L "$dir" ]]; then
    snapshot_valid "$dir" || return 1
    return 0
  fi
  # This revalidates the exact binaries against the build stamp and preflight.
  bash "$root/scripts/matter-verified-run.sh" --check-only || return 1
  target="$(realpath -m -- "${CARGO_TARGET_DIR:-$root/target}")" || return 1
  provider="$(realpath -m -- "${MATTER_PROVIDER_TARGET_DIR:-${CARGO_TARGET_DIR:-$root/providers/whatsapp-rust/target}}")" || return 1
  stamp="$target/.matter-alpha-verified-build"
  safe_file "$stamp" || return 1
  temp="$(mktemp -d "$recovery/.snapshot.XXXXXX")" || return 1
  if ! cp --reflink=auto -- "$target/debug/whatsapp-matter-control" "$temp/whatsapp-matter-control" ||
     ! cp --reflink=auto -- "$provider/debug/whatsapp-matter-rust-provider" "$temp/whatsapp-matter-rust-provider" ||
     ! cp -- "$root/scripts/whatsapp-alpha-existing.sh" "$temp/launcher.sh" ||
     ! printf '%s\n' "$revision" > "$temp/revision"; then
    rm -rf -- "$temp"
    return 1
  fi
  # Check copies against the original verified stamp, not just their own hash.
  actual="$(printf '%s\n' "$revision"; sha256sum -- "$target/debug/whatsapp-matter-control" "$provider/debug/whatsapp-matter-rust-provider" "$root/Cargo.lock" "$root/providers/whatsapp-rust/Cargo.lock")" || { rm -rf -- "$temp"; return 1; }
  if [[ "$actual" != "$(cat "$stamp")" ]] ||
     [[ "$(git -C "$root" rev-parse HEAD)" != "$revision" ]] ||
     ! git -C "$root" diff --quiet HEAD -- ||
     ! cmp -s "$target/debug/whatsapp-matter-control" "$temp/whatsapp-matter-control" ||
     ! cmp -s "$provider/debug/whatsapp-matter-rust-provider" "$temp/whatsapp-matter-rust-provider" ||
     ! cmp -s "$root/scripts/whatsapp-alpha-existing.sh" "$temp/launcher.sh"; then
    rm -rf -- "$temp"
    return 1
  fi
  if ! (cd "$temp" && sha256sum -- revision whatsapp-matter-control whatsapp-matter-rust-provider launcher.sh > SHA256SUMS) ||
     ! mv -T -- "$temp" "$dir"; then
    rm -rf -- "$temp"
    return 1
  fi
  snapshot_valid "$dir"
}

prune_snapshots() {
  local recovery="$1" active="$2" good="$3" dir name
  for dir in "$recovery"/*; do
    name="$(basename "$dir")"
    if valid_revision "$name" && [[ "$name" != "$active" && "$name" != "$good" ]] &&
       snapshot_valid "$dir"; then
      rm -rf -- "$dir"
    fi
  done
}

run_snapshot() {
  local root="$1" dir="$2" revision
  snapshot_valid "$dir" && no_runtime || return 1
  revision="$(cat "$dir/revision")" || return 1
  export MATTER_WA_RUNNING_REVISION="$revision"
  export MATTER_WA_RECOVERY_ROOT="$root"
  # Same session/Vault preflight, single-instance guard and backup as normal.
  bash "$dir/launcher.sh" --resume-only "$dir/whatsapp-matter-control" "$dir/whatsapp-matter-rust-provider" --vault-env /etc/base-matter/vault-alpha.env
}

main() {
  local root state recovery request result connected good_file lock_fd
  local active good next status verified failures=0 candidate started marker_revision
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)" || return 1
  cd "$root" || return 1
  [[ "$(git symbolic-ref --quiet --short HEAD)" == alpha ]] || return 1
  state="$HOME/.local/share/base-matter/branches/alpha/whatsapp-test"
  safe_dir "$state" || return 1
  recovery="$state/runtime-recovery"
  if [[ ! -e "$recovery" && ! -L "$recovery" ]]; then
    mkdir -m 700 -- "$recovery" || return 1
  fi
  safe_dir "$recovery" || return 1
  [[ ! -L "$recovery/supervisor.lock" ]] || return 1
  exec {lock_fd}>"$recovery/supervisor.lock" || return 1
  flock -n "$lock_fd" || { printf '%s\n' 'MATTER_RECOVERY=SUPERVISOR_ALREADY_RUNNING' >&2; return 1; }
  no_runtime || return 1
  request="$state/maintenance-restart.pending.json"
  result="$state/maintenance-restart.result"
  connected="$state/maintenance-runtime-connected"
  good_file="$recovery/last-good"
  [[ ! -L "$request" && ! -L "$result" && ! -L "$connected" && ! -L "$good_file" ]] || return 1
  export MATTER_WA_SUPERVISED=1 GIT_TERMINAL_PROMPT=0
  export MATTER_WA_RICH_HTML="${MATTER_WA_RICH_HTML:-1}" MATTER_WA_HTML_AUTOSEND="${MATTER_WA_HTML_AUTOSEND:-1}"
  export MATTER_WA_NATIVE_BUTTONS="${MATTER_WA_NATIVE_BUTTONS:-1}" MATTER_WA_DIAG="${MATTER_WA_DIAG:-1}" MATTER_WA_MENU_LAB="${MATTER_WA_MENU_LAB:-0}"
  good=""
  if safe_file "$good_file"; then
    good="$(cat "$good_file")"
    valid_revision "$good" && snapshot_valid "$recovery/$good" || { printf '%s\n' 'MATTER_RECOVERY=INVALID_LAST_GOOD' >&2; return 1; }
  fi
  # If the supervisor itself was interrupted, retain the revision whose
  # authenticated connection was already observed by the provider.
  if safe_file "$connected"; then
    marker_revision="$(cat "$connected")"
    if valid_revision "$marker_revision" && snapshot_valid "$recovery/$marker_revision"; then
      good="$marker_revision"
      atomic_text "$good_file" "$good" || return 1
    fi
  fi
  active="$(git rev-parse HEAD)" || return 1
  if ! save_snapshot "$root" "$recovery" "$active"; then
    [[ -n "$good" ]] || { printf '%s\n' 'MATTER_RECOVERY=NO_VERIFIED_RUNTIME' >&2; return 1; }
    active="$good"
    printf '%s\n' 'MATTER_RECOVERY=LAST_GOOD'
  fi
  while true; do
    no_runtime || return 1
    # Do not resume an old executable after the operator has switched branch.
    [[ "$(git symbolic-ref --quiet --short HEAD)" == alpha ]] || return 1
    prune_snapshots "$recovery" "$active" "$good"
    rm -f -- "$connected" || return 1
    status=0
    started=$SECONDS
    printf 'MATTER_RUNTIME=%s\n' "$active"
    run_snapshot "$root" "$recovery/$active" || status=$?
    # Promote only after the provider confirms its authenticated connection.
    if safe_file "$connected" && [[ "$(cat "$connected")" == "$active" ]]; then
      good="$active"
      atomic_text "$good_file" "$good" || return 1
      # A momentary connect/disconnect must not reset the retry budget.
      if (( SECONDS - started >= 120 )); then
        failures=0
      fi
    fi
    case "$status" in
      75)
        printf '%s\n' 'MATTER_RESTART=AUTHORIZED'
        if ! safe_file "$request" || [[ -L "$result" ]]; then
          printf '%s\n' 'MATTER_RESTART=REFUSED_MISSING_REPORT' >&2
          return 1
        fi
        # An authenticated accepted restart is also proof of a usable runtime.
        good="$active"
        atomic_text "$good_file" "$good" || return 1
        next="$(git rev-parse HEAD)" || return 1
        verified=verified-reused
        candidate="$active"
        if [[ "$active" != "$next" ]]; then
          if bash "$root/scripts/matter-verified-run.sh" --build-only &&
             save_snapshot "$root" "$recovery" "$next"; then
            verified=verified-built
            candidate="$next"
          else
            atomic_text "$result" build-failed-recovered || return 1
            printf '%s\n' 'MATTER_RESTART=FAILED_BUILD' 'MATTER_RECOVERY=RESUMING_LAST_GOOD' >&2
            active="$good"
            continue
          fi
        fi
        atomic_text "$result" "$verified" || return 1
        active="$candidate"
        failures=0
        printf '%s\n' 'MATTER_RESTART=VERIFIED_WAITING_CONNECTION'
        ;;
      76)
        printf '%s\n' 'MATTER_UPDATE=LEGACY_PROVIDER_REQUIRES_MIGRATION' >&2
        return 1
        ;;
      0|130|143)
        return "$status"
        ;;
      *)
        # No repeated pairing or endless retry loop. If a candidate failed
        # before connecting, recover the previous verified runtime once.
        if [[ -n "$good" && "$active" != "$good" ]]; then
          if safe_file "$request"; then
            atomic_text "$result" start-failed-recovered || return 1
          fi
          active="$good"
          failures=0
          printf '%s\n' 'MATTER_RECOVERY=RESUMING_LAST_GOOD' >&2
          continue
        fi
        failures=$((failures + 1))
        if (( failures > 2 )); then
          printf '%s\n' 'MATTER_RECOVERY=STOPPED_AFTER_BOUNDED_RETRIES' >&2
          return "$status"
        fi
        printf 'MATTER_RECOVERY=RETRY_%s\n' "$failures" >&2
        sleep "$((failures * 5))"
        ;;
    esac
  done
}
if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  main "$@"
fi
