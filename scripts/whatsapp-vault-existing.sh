#!/usr/bin/env bash
# Base-Matter alpha — safe legacy wa-test-1 runtime with precompiled CI binaries.
# Never creates credentials, changes socket identity, regenerates sessions, or builds Rust.
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
  printf 'DENEGADO: %s\n' "$*" >&2
  return 1
}

main() {
  if (( $# != 2 )); then
    fail 'Uso: bash scripts/whatsapp-vault-existing.sh /ruta/whatsapp-matter-control /ruta/whatsapp-matter-rust-provider'
    return 1
  fi
  local controller="$1" provider="$2"
  if [[ "$controller" != /* || "$provider" != /* || ! -f "$controller" || ! -x "$controller" || ! -f "$provider" || ! -x "$provider" ]]; then
    fail 'Dos binarios absolutos, regulares y ejecutables requeridos'
    return 1
  fi

  local root config vault
  root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
  config="$HOME/.local/share/base-matter/whatsapp-test/runtime.env"
  vault="/etc/base-matter/vault.env"
  if [[ ! -f "$config" || -L "$config" || ! -f "$vault" || -L "$vault" ]]; then
    fail 'Falta configuración privada existente; no se crea ni se reemplaza'
    return 1
  fi
  if [[ "$(stat -c %a "$config")" != 600 || "$(stat -c %a "$vault")" != 600 ]]; then
    fail 'runtime.env y vault.env deben tener modo 0600'
    return 1
  fi
  if [[ "$(git -C "$root" symbolic-ref --quiet --short HEAD)" != "alpha" ]]; then
    fail 'Solo rama alpha existente'
    return 1
  fi
  if ! id -nG | tr ' ' '\n' | grep -Fxq 'matter-vault-ipc'; then
    fail 'La sesión Sakura no heredó el grupo matter-vault-ipc. Abrir nueva sesión Sakura.'
    return 1
  fi
  # Shell restricted to config files owned locally, never sourced from Git.
  source "$config"
  set -a
  source "$vault"
  set +a

  if [[ -z "${MATTER_WA_SOCKET_ID:-}" || "$MATTER_WA_SOCKET_ID" != "${MATTER_VAULT_SOCKET_ID:-}" ]]; then
    fail 'Identidad de WhatsApp y Vault no coincide; no se mezclan sesiones'
    return 1
  fi
  if [[ "${MATTER_VAULT_CLIENT_ID:-}" != "base-$MATTER_WA_SOCKET_ID" ]]; then
    fail 'Identificador de cliente Vault no corresponde al socket'
    return 1
  fi
  if [[ "${MATTER_VAULT_IPC_SOCKET:-}" != "/run/matter-vault/vault.sock" ||
        "${MATTER_VAULT_CLIENT_KEY_FILE:-}" != "/etc/base-matter/client.key" ]]; then
    fail 'Canal Vault no esperado'
    return 1
  fi
  if [[ "${MATTER_WA_BRIDGE_ADDR:-}" != 127.0.0.1:* ]]; then
    fail 'Control WhatsApp debe usar loopback'
    return 1
  fi
  if [[ ! -f "${MATTER_WA_ACTIVATION_DB:-/nonexistent}" ||
        ! -f "${MATTER_WA_SESSION_DB:-/nonexistent}" ]]; then
    fail 'Falta sesión o activación original; no se reinicializa ni se empareja'
    return 1
  fi
  "$controller" --check || return 1
  "$provider" --check || return 1

  cd "$root"
  local state
  state="$(dirname -- "$config")"
  "$controller" --serve >>"$state/controller-vault.log" 2>&1 </dev/null &
  CONTROLLER_PID=$!
  sleep 1
  if ! kill -0 "$CONTROLLER_PID" 2>/dev/null; then
    fail 'No arrancó el controlador: revisar controller-vault.log privado'
    return 1
  fi
  printf '%s\n' 'Controlador Base/Vault iniciado en el socket preexistente; proveedor WhatsApp en primer plano.'
  # Always attempt the existing session first. Do not consume the phone's
  # pairing-code budget when the database already contains a device record.
  "$provider" --resume-only
}

main "$@"
