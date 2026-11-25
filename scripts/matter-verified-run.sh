#!/usr/bin/env bash
# Base-Matter: fail-closed Cargo check/test/build gate before alpha WhatsApp.
# No pairing, no new sessions, no secrets, no Vault key generation.
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
set -Eeuo pipefail
umask 077

fail() {
  printf 'MATTER_RUN_REFUSED=%s\n' "$1" >&2
  return 1
}

main() {
  if (( $# > 1 )); then
    fail 'Usage: cargo run [-- --check-only|--help]'
    return 1
  fi
  case "${1-}" in
    --help|-h)
      printf '%s\n' 'cargo run: format -> check -> test -> build -> preflight -> resume existing WhatsApp alpha'
      printf '%s\n' 'cargo run -- --check-only: run all gates and preflight without connecting WhatsApp'
      printf '%s\n' 'Never pairs, creates a session, modifies Vault credentials or launches a second bot.'
      return 0
      ;;
    ""|--check-only) ;;
    *)
      fail 'Only --check-only and --help are accepted'
      return 1
      ;;
  esac

  local root branch
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
  branch="$(git -C "$root" symbolic-ref --quiet --short HEAD)" || {
    fail 'An attached alpha Git branch is required'
    return 1
  }
  if [[ "$branch" != "alpha" ]]; then
    fail 'Only branch alpha may resume the existing WhatsApp session'
    return 1
  fi
  if [[ ! -d "$root/providers/whatsapp-rust" || ! -f "$root/scripts/whatsapp-alpha-existing.sh" ]]; then
    fail 'The required provider and resume-only launcher are absent'
    return 1
  fi
  if ! command -v cargo >/dev/null 2>&1 || ! command -v python3 >/dev/null 2>&1; then
    fail 'Cargo and Python 3 are required'
    return 1
  fi
  if ! command -v pgrep >/dev/null 2>&1; then
    fail 'procps pgrep is required to enforce the single-instance guard'
    return 1
  fi
  # Check before building so two terminals cannot accidentally overwrite
  # binaries while this instance serves active WhatsApp chats.
  if pgrep -f '[w]hatsapp-matter-control.*--serve|[w]hatsapp-matter-rust-provider.*--resume-only' >/dev/null 2>&1; then
    fail 'An existing WhatsApp controller/provider is running; use Ctrl+C in its tmux window first'
    return 1
  fi
  if [[ ! -f /etc/base-matter/vault-alpha.env ]]; then
    fail 'Dedicated alpha Vault enrollment is required; never downgrade silently'
    return 1
  fi

  cd "$root"
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-2}"
  export MATTER_WA_RICH_HTML="${MATTER_WA_RICH_HTML:-1}"
  export MATTER_WA_HTML_AUTOSEND="${MATTER_WA_HTML_AUTOSEND:-1}"
  export MATTER_WA_NATIVE_BUTTONS="${MATTER_WA_NATIVE_BUTTONS:-1}"
  export MATTER_WA_DIAG="${MATTER_WA_DIAG:-1}"
  export MATTER_WA_MENU_LAB="${MATTER_WA_MENU_LAB:-0}"

  printf '%s\n' 'MATTER_GATE=FORMAT'
  cargo fmt --all -- --check
  cargo fmt --manifest-path providers/whatsapp-rust/Cargo.toml --all -- --check

  printf '%s\n' 'MATTER_GATE=CHECK'
  cargo check --all-targets
  cargo check --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets

  printf '%s\n' 'MATTER_GATE=TEST'
  cargo test --all-targets
  cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets

  printf '%s\n' 'MATTER_GATE=BUILD'
  cargo build --bin whatsapp-matter-control
  cargo build --manifest-path providers/whatsapp-rust/Cargo.toml --bin whatsapp-matter-rust-provider

  local target_dir provider_dir controller provider
  target_dir="$(cargo metadata --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
  provider_dir="$(cargo metadata --manifest-path providers/whatsapp-rust/Cargo.toml --no-deps --format-version 1 | python3 -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')"
  controller="$target_dir/debug/whatsapp-matter-control"
  provider="$provider_dir/debug/whatsapp-matter-rust-provider"
  if [[ ! -x "$controller" || ! -x "$provider" || -L "$controller" || -L "$provider" ]]; then
    fail 'The verified controller/provider binaries were not built safely'
    return 1
  fi

  local -a launcher=(bash "$root/scripts/whatsapp-alpha-existing.sh")
  local -a vault=(--vault-env /etc/base-matter/vault-alpha.env)
  printf '%s\n' 'MATTER_GATE=SESSION_AND_VAULT_PREFLIGHT'
  "${launcher[@]}" --check "$controller" "$provider" "${vault[@]}"
  printf '%s\n' 'MATTER_GATE=PASSED'

  if [[ "${1-}" == "--check-only" ]]; then
    printf '%s\n' 'MATTER_START=SKIPPED_CHECK_ONLY'
    return 0
  fi

  # Fail closed if a second instance appeared while the gates were running.
  if pgrep -f '[w]hatsapp-matter-control.*--serve|[w]hatsapp-matter-rust-provider.*--resume-only' >/dev/null 2>&1; then
    fail 'An instance started during the checks; refusing parallel WhatsApp runtime'
    return 1
  fi

  printf '%s\n' 'MATTER_START=EXISTING_ALPHA_RESUME_ONLY'
  "${launcher[@]}" --resume-only "$controller" "$provider" "${vault[@]}"
}

main "$@"
