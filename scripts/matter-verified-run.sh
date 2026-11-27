#!/usr/bin/env bash
# Base-Matter: explicit verified build; start only from verified alpha binaries.
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
    fail 'Usage: bash scripts/matter-verified-run.sh --build-only | cargo run [-- --check-only|--help]'
    return 1
  fi
  case "${1-}" in
    --help|-h)
      printf '%s\n' 'bash scripts/matter-verified-run.sh --build-only: format, check, test, Clippy and cargo build; never connects'
      printf '%s\n' 'cargo run: verify existing binaries and preflight, then resume alpha; no nested Cargo calls'
      printf '%s\n' 'cargo run -- --check-only: verify existing binaries and preflight without connecting'
      printf '%s\n' 'Never pairs, creates a session, modifies Vault credentials or launches a second bot.'
      return 0
      ;;
    ""|--check-only|--build-only) ;;
    *)
      fail 'Only --build-only, --check-only and --help are accepted'
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
  if ! command -v python3 >/dev/null 2>&1 || ! command -v sha256sum >/dev/null 2>&1; then
    fail 'Python 3 and sha256sum are required'
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
  if [[ "${1-}" != "--build-only" && ! -f /etc/base-matter/vault-alpha.env ]]; then
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

  # The two manifests are independent. Resolve paths without calling Cargo on start.
  local target_dir provider_dir controller provider stamp revision actual
  target_dir="$(realpath -m -- "${CARGO_TARGET_DIR:-$root/target}")"
  provider_dir="$(realpath -m -- "${MATTER_PROVIDER_TARGET_DIR:-${CARGO_TARGET_DIR:-$root/providers/whatsapp-rust/target}}")"
  controller="$target_dir/debug/whatsapp-matter-control"
  provider="$provider_dir/debug/whatsapp-matter-rust-provider"
  stamp="$target_dir/.matter-alpha-verified-build"
  revision="$(git rev-parse HEAD)"
  if ! git diff --quiet HEAD --; then
    fail 'Tracked source changes must be committed before building or starting'
    return 1
  fi
  local -a base_cargo=(env "CARGO_TARGET_DIR=$target_dir" cargo)
  local -a provider_cargo=(env "CARGO_TARGET_DIR=$provider_dir" cargo)
  local -a manifest_files=("$controller" "$provider" "$root/Cargo.lock" "$root/providers/whatsapp-rust/Cargo.lock")
  if [[ "${1-}" == "--build-only" ]]; then
    mkdir -p -- "$target_dir"
    rm -f -- "$stamp"
    printf '%s\n' 'MATTER_GATE=FORMAT'
    cargo fmt --all -- --check
    cargo fmt --manifest-path providers/whatsapp-rust/Cargo.toml --all -- --check
    printf '%s\n' 'MATTER_GATE=CHECK_TEST_CLIPPY'
    "${base_cargo[@]}" check --locked --all-targets
    "${base_cargo[@]}" test --locked --all-targets
    "${base_cargo[@]}" clippy --locked --all-targets -- -D warnings
    "${provider_cargo[@]}" check --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
    "${provider_cargo[@]}" test --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets
    "${provider_cargo[@]}" clippy --manifest-path providers/whatsapp-rust/Cargo.toml --all-targets -- -D warnings
    printf '%s\n' 'MATTER_GATE=BUILD'
    "${base_cargo[@]}" build --locked --bins
    "${provider_cargo[@]}" build --locked --manifest-path providers/whatsapp-rust/Cargo.toml --bin whatsapp-matter-rust-provider
    if [[ "$(git rev-parse HEAD)" != "$revision" ]] || ! git diff --quiet HEAD --; then
      fail 'Source changed during verification; no build stamp published'
      return 1
    fi
  fi
  if [[ ! -x "$controller" || ! -x "$provider" || -L "$controller" || -L "$provider" ]]; then
    fail 'Verified binaries missing; run bash scripts/matter-verified-run.sh --build-only'
    return 1
  fi
  actual="$(printf '%s\n' "$revision"; sha256sum -- "${manifest_files[@]}")" || return 1
  if [[ "${1-}" == "--build-only" ]]; then
    local temporary
    temporary="$(mktemp "$target_dir/.matter-alpha-verified-build.XXXXXX")"
    printf '%s\n' "$actual" > "$temporary"
    mv -T -- "$temporary" "$stamp"
    printf '%s\n' 'MATTER_BUILD=VERIFIED'
    return 0
  fi
  if [[ ! -f "$stamp" || -L "$stamp" ]] || [[ "$(cat -- "$stamp")" != "$actual" ]]; then
    fail 'Build missing, outdated or changed; run bash scripts/matter-verified-run.sh --build-only'
    return 1
  fi
  printf '%s\n' 'MATTER_BUILD=REUSED_NO_NESTED_CARGO'

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
    fail 'An instance started during preflight; refusing parallel WhatsApp runtime'
    return 1
  fi

  printf '%s\n' 'MATTER_START=EXISTING_ALPHA_RESUME_ONLY'
  "${launcher[@]}" --resume-only "$controller" "$provider" "${vault[@]}"
}

main "$@"
