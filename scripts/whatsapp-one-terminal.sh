#!/usr/bin/env bash
# Base-Matter — one terminal / two isolated Rust processes, one foreground command.
# Créditos:
# Hecho por Lournyx Matterssmith [Lou-Nyx].
set -Eeuo pipefail
CONTROLLER_PID=""
cleanup() {
    if [[ -n "$CONTROLLER_PID" ]] && kill -0 "$CONTROLLER_PID" 2>/dev/null; then
        kill "$CONTROLLER_PID" 2>/dev/null || true
        wait "$CONTROLLER_PID" 2>/dev/null || true
    fi
}
trap cleanup EXIT

run() {
    umask 077
    local root branch state config port controller pid
    root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
    branch="$(git -C "$root" symbolic-ref --quiet --short HEAD)" || {
        printf 'Cannot identify the checked-out branch.\n' >&2
        return 1
    }
    case "$branch" in
        alpha) port=39821 ;;
        beta) port=39822 ;;
        stable) port=39823 ;;
        *) printf 'Only alpha, beta and stable are supported for isolated sessions.\n' >&2; return 1 ;;
    esac

    if [[ "${1:-}" == "--help" ]]; then
        printf 'Run without arguments: prepare the private branch session, start controller, then connect WhatsApp.\n'
        printf 'For offline verification, run cargo check/test separately.\n'
        return 0
    fi
    if [[ -n "${1:-}" ]]; then
        printf 'Unsupported launch option.\n' >&2
        return 2
    fi

    state="$HOME/.local/share/base-matter/branches/$branch/whatsapp-test"
    config="$state/runtime.env"
    if [[ -L "$state" || -L "$config" ]]; then
        printf 'Refusing symbolic links in session configuration.\n' >&2
        return 1
    fi
    mkdir -p -m 700 -- "$state"
    chmod 700 -- "$state"
    if [[ ! -f "$config" ]]; then
        MATTER_WA_STATE_DIR="$state" \
        MATTER_WA_PORT="$port" \
        MATTER_WA_BRANCH="$branch" \
        MATTER_WA_SIMPLE_SETUP=1 \
        bash "$root/scripts/whatsapp-test-setup.sh"
    fi
    if [[ ! -f "$config" || -L "$config" ]]; then
        printf 'Missing regular private runtime.env.\n' >&2
        return 1
    fi
    if [[ "$(stat -c '%a' -- "$config")" != "600" ]]; then
        printf 'runtime.env must have permission 0600.\n' >&2
        return 1
    fi

    # Generated locally with restrictive permissions. Never print its secrets.
    source "$config"
    if [[ "$MATTER_WA_BRIDGE_ADDR" != "127.0.0.1:$port" ]]; then
        printf 'Unexpected branch controller port; refusing to cross sessions.\n' >&2
        return 1
    fi
    if [[ "$MATTER_WA_SESSION_DB" != "$state/session.db" ||
          "$MATTER_WA_ACTIVATION_DB" != "$state/activation.db" ||
          "$MATTER_WA_REPLAY_DB" != "$state/replay.db" ]]; then
        printf 'Branch database isolation mismatch; refusing to start.\n' >&2
        return 1
    fi
    cd "$root"
    # Reuse the same Cargo artifacts across all checkouts/branches.
    # An absolute CARGO_TARGET_DIR makes the launch path unambiguous.
    if [[ -n "${CARGO_TARGET_DIR:-}" && "${CARGO_TARGET_DIR}" != /* ]]; then
        printf 'CARGO_TARGET_DIR must be an absolute path.\\n' >&2
        return 1
    fi
    cargo build --locked --bin whatsapp-matter-control
    controller="${CARGO_TARGET_DIR:-$root/target}/debug/whatsapp-matter-control"
    "$controller" --check
    if [[ ! -f "$MATTER_WA_ACTIVATION_DB" ]]; then
        "$controller" --init
    fi

    "$controller" --serve >>"$state/controller.log" 2>&1 </dev/null &
    pid=$!
    CONTROLLER_PID="$pid"
    # The EXIT trap stops only this launcher-owned child.
    sleep 1
    if ! kill -0 "$pid" 2>/dev/null; then
        printf 'Controller failed to start; latest nonsecret diagnostic:\n' >&2
        tail -n 8 -- "$state/controller.log" >&2
        return 1
    fi
    printf 'Base-Matter %s: controller started; WhatsApp pairing or session resume follows.\n' "$branch"
    cargo run --manifest-path "$root/providers/whatsapp-rust/Cargo.toml"
}

run "$@"
