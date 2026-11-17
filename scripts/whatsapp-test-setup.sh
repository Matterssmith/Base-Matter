#!/usr/bin/env bash
# Base-Matter: create private config for a controlled WhatsApp pairing trial.
# Créditos:
# Hecho por Lournyx Matterssmith [Lou-Nyx].
set -euo pipefail
setup() {
    umask 077
    local directory="$HOME/.local/share/base-matter/whatsapp-test"
    local config="$directory/runtime.env"
    if [[ -L "$directory" || -L "$config" ]]; then
        printf 'Refusing symlink in private session path.\n' >&2
        return 1
    fi
    if [[ "${1:-}" == "--enable-recovery" ]]; then
        if [[ ! -f "$config" || -L "$config" ]]; then
            printf 'Existing regular runtime.env is required for recovery activation.\n' >&2
            return 1
        fi
        if grep -q '^export MATTER_WA_RECOVERY_KEY_HEX=' "$config"; then
            printf 'Recovery key already present; existing configuration retained.\n'
            return 0
        fi
        local recovery_key=""
        recovery_key="$(openssl rand -hex 32)"
        printf 'export MATTER_WA_RECOVERY_KEY_HEX=%q\n' "$recovery_key" >> "$config"
        chmod 600 "$config"
        printf 'Added a separate recovery key. Never share it or store it in Git.\n'
        return 0
    fi
    if [[ -e "$config" ]]; then
        printf 'Existing private configuration retained: %s\n' "$config"
        printf 'No keys or WhatsApp session files were replaced.\n'
        return 0
    fi
    if ! command -v openssl >/dev/null 2>&1; then
        printf 'OpenSSL is required to generate private keys.\n' >&2
        return 1
    fi
    mkdir -p -m 700 "$directory"
    chmod 700 "$directory"

    local phone="" jid="" socket="" owner=""
    read -r -p 'Authorized test phone (international digits only): ' phone
    if [[ ! "$phone" =~ ^[1-9][0-9]{7,14}$ ]]; then
        printf 'Phone must have 8 to 15 international digits.\n' >&2
        return 1
    fi
    read -r -p "Your private JID [$phone@s.whatsapp.net]: " jid
    if [[ -z "$jid" ]]; then jid="$phone@s.whatsapp.net"; fi
    if [[ ! "$jid" =~ ^[A-Za-z0-9._:-]+@(s[.]whatsapp[.]net|lid)$ ]]; then
        printf 'Private JID must use @s.whatsapp.net or @lid.\n' >&2
        return 1
    fi
    read -r -p 'Internal socket name [wa-test-1]: ' socket
    if [[ -z "$socket" ]]; then socket="wa-test-1"; fi
    read -r -p 'Internal socket owner [owner-test]: ' owner
    if [[ -z "$owner" ]]; then owner="owner-test"; fi
    if [[ ! "$socket" =~ ^[A-Za-z0-9._:-]{1,128}$ ||
          ! "$owner" =~ ^[A-Za-z0-9._:-]{1,128}$ ]]; then
        printf 'Invalid socket or owner ID.\n' >&2
        return 1
    fi

    local transport_key="" replay_key="" recovery_key=""
    transport_key="$(openssl rand -hex 32)"
    replay_key="$(openssl rand -hex 32)"
    recovery_key="$(openssl rand -hex 32)"
    if [[ "$transport_key" == "$replay_key" ]]; then
        printf 'Key generation must produce distinct secrets.\n' >&2
        return 1
    fi
    {
        printf 'export MATTER_WA_PHONE=%q\n' "$phone"
        printf 'export MATTER_WA_OWNER_JID=%q\n' "$jid"
        printf 'export MATTER_WA_SOCKET_ID=%q\n' "$socket"
        printf 'export MATTER_WA_OWNER_ID=%q\n' "$owner"
        printf 'export MATTER_WA_BRIDGE_ADDR=%q\n' '127.0.0.1:39819'
        printf 'export MATTER_WA_BRIDGE_KEY_HEX=%q\n' "$transport_key"
        printf 'export MATTER_WA_REPLAY_KEY_HEX=%q\n' "$replay_key"
        printf 'export MATTER_WA_RECOVERY_KEY_HEX=%q\n' "$recovery_key"
        printf 'export MATTER_WA_SESSION_DB=%q\n' "$directory/session.db"
        printf 'export MATTER_WA_ACTIVATION_DB=%q\n' "$directory/activation.db"
        printf 'export MATTER_WA_REPLAY_DB=%q\n' "$directory/replay.db"
    } > "$config"
    chmod 600 "$config"
    printf 'Private configuration created: %s\n' "$config"
    printf 'Do not share the pairing code, session database or this file.\n'
}
setup "$@"
