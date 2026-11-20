#!/usr/bin/env bash
# Ubuntu — usuario Sakura, acceso administrativo opcional y continuidad Matter.
# Créditos:
# Hecho por Lournyx Matterssmith [Lou-Nyx].
set -euo pipefail
umask 077

usage() {
    cat <<'HELP'
Uso:
  sudo bash tools/setup-sakura.sh --admin --password-prompt --linger
  sudo bash tools/setup-sakura.sh --admin --authorized-key-file /ruta/clave.pub --linger

Opciones:
  --admin              Agregar Sakura al grupo sudo (solicitud explícita).
  --password-prompt    Pedir contraseña nueva con passwd; no se imprime.
  --authorized-key-file RUTA
                       Agregar una clave SSH pública (nunca una clave privada).
  --linger             Habilitar systemd user al arrancar, después de crear Sakura.
  --help               Mostrar esta ayuda sin modificar nada.

Predeterminado: crea Sakura con contraseña bloqueada y sin sudo.
No crea sudoers sin contraseña ni modifica sshd_config.
Si Sakura ya existe, conserva sus datos y no la vuelve a crear.
HELP
}

main() {
    local want_admin=0 want_password=0 want_linger=0 public_key_file=''
    local home_dir='' primary_group='' uid='' key_line=''
    local was_created=0

    while [ "$#" -gt 0 ]; do
        case "$1" in
            --admin) want_admin=1; shift ;;
            --password-prompt) want_password=1; shift ;;
            --authorized-key-file)
                if [ "$#" -lt 2 ]; then
                    printf '%s\n' 'ERROR: falta ruta para --authorized-key-file.' >&2
                    return 2
                fi
                public_key_file="$2"
                shift 2
                ;;
            --linger) want_linger=1; shift ;;
            --help|-h) usage; return 0 ;;
            *)
                printf 'ERROR: opción desconocida: %s\n' "$1" >&2
                usage >&2
                return 2
                ;;
        esac
    done

    if [ "$(id -u)" -ne 0 ]; then
        printf '%s\n' 'ERROR: ejecuta el script con sudo desde una cuenta administradora.' >&2
        return 2
    fi
    for command in useradd usermod passwd install id getent chown chmod grep; do
        if ! command -v "$command" >/dev/null 2>&1; then
            printf 'ERROR: falta el ejecutable del sistema: %s\n' "$command" >&2
            return 2
        fi
    done
    if [ "$want_password" -eq 1 ] && [ ! -t 0 ]; then
        printf '%s\n' 'ERROR: configurar contraseña requiere un terminal interactivo.' >&2
        return 2
    fi
    if [ "$want_admin" -eq 1 ] && ! getent group sudo >/dev/null; then
        printf '%s\n' 'ERROR: falta el grupo sudo. Verifica que realmente sea Ubuntu.' >&2
        return 2
    fi
    if [ -n "$public_key_file" ]; then
        if [ ! -f "$public_key_file" ] || [ -L "$public_key_file" ]; then
            printf '%s\n' 'ERROR: la clave pública SSH no existe o es un enlace.' >&2
            return 2
        fi
        if [ "$(wc -l < "$public_key_file")" -gt 1 ]; then
            printf '%s\n' 'ERROR: solo se admite una clave pública por archivo.' >&2
            return 2
        fi
        key_line="$(cat -- "$public_key_file")"
        if ! printf '%s\n' "$key_line" |
            grep -Eq '^(ssh-ed25519|ssh-rsa|ecdsa-sha2-nistp(256|384|521)|sk-ssh-ed25519@openssh.com) [A-Za-z0-9+/=]+( .*)?$'; then
            printf '%s\n' 'ERROR: no se reconoce una clave SSH pública válida.' >&2
            return 2
        fi
    fi

    if id Sakura >/dev/null 2>&1; then
        printf '%s\n' 'Sakura ya existe. Se mantienen su UID, grupos y archivos.'
    else
        # Se usa useradd: adduser puede rechazar la S mayúscula por su política de nombres.
        useradd --create-home --user-group --shell /bin/bash Sakura
        was_created=1
        printf '%s\n' 'Sakura creada con contraseña inicialmente bloqueada.'
    fi

    uid="$(id -u Sakura)"
    if [ "$uid" -eq 0 ]; then
        printf '%s\n' 'ERROR: la cuenta Sakura no debe tener UID 0.' >&2
        return 2
    fi
    home_dir="$(getent passwd Sakura | cut -d: -f6)"
    primary_group="$(id -gn Sakura)"
    if [ -z "$home_dir" ] || [ "$home_dir" = "/" ] || [ -L "$home_dir" ]; then
        printf '%s\n' 'ERROR: directorio HOME de Sakura no válido.' >&2
        return 2
    fi

    # Nunca se borra el HOME existente ni se alteran otros usuarios.
    install -d -m 0700 -o Sakura -g "$primary_group" -- "$home_dir/Matter"
    install -d -m 0700 -o Sakura -g "$primary_group" -- "$home_dir/Matter/workspaces"
    install -d -m 0700 -o Sakura -g "$primary_group" -- "$home_dir/Matter/checkpoints"
    install -d -m 0700 -o Sakura -g "$primary_group" -- "$home_dir/.config/matter-ai"

    if [ -n "$public_key_file" ]; then
        if [ -L "$home_dir/.ssh" ] || [ -L "$home_dir/.ssh/authorized_keys" ]; then
            printf '%s\n' 'ERROR: rutas SSH simbólicas prohibidas.' >&2
            return 2
        fi
        install -d -m 0700 -o Sakura -g "$primary_group" -- "$home_dir/.ssh"
        touch -- "$home_dir/.ssh/authorized_keys"
        chown Sakura:"$primary_group" "$home_dir/.ssh/authorized_keys"
        chmod 0600 "$home_dir/.ssh/authorized_keys"
        if ! grep -Fxq -- "$key_line" "$home_dir/.ssh/authorized_keys"; then
            printf '%s\n' "$key_line" >> "$home_dir/.ssh/authorized_keys"
        fi
        printf '%s\n' 'SSH: clave pública autorizada (sin copiar clave privada).'
    fi

    if [ "$want_admin" -eq 1 ]; then
        usermod -aG sudo Sakura
        printf '%s\n' 'Administrador: Sakura incorporada al grupo sudo.'
    fi
    if [ "$want_password" -eq 1 ]; then
        passwd Sakura
        printf '%s\n' 'Contraseña de Sakura configurada mediante passwd.'
    elif [ "$was_created" -eq 1 ]; then
        printf '%s\n' 'La contraseña sigue bloqueada: para SSH usa clave pública o configura passwd.'
    else
        printf '%s\n' 'Contraseña existente no modificada.'
    fi

    printf '%s\n' 'Verificación de identidad y grupos:'
    id Sakura
    getent passwd Sakura

    if [ "$want_linger" -eq 1 ]; then
        if command -v loginctl >/dev/null 2>&1; then
            if loginctl enable-linger Sakura; then
                printf '%s\n' 'Linger habilitado para Sakura: systemd --user puede iniciar sin sesión.'
            else
                printf '%s\n' 'AVISO: linger no habilitado; revisa systemd/logind.' >&2
            fi
        else
            printf '%s\n' 'AVISO: loginctl no está disponible.' >&2
        fi
    fi

    printf '\n%s\n' 'Sakura está creada; el instalador no toca la sesión WhatsApp ni sus datos.'
    printf '%s\n' 'Desde otra cuenta administradora: sudo -iu Sakura'
    printf '%s\n' 'Si tiene contraseña configurada, Sakura puede utilizar sudo normalmente.'
    printf '%s\n' 'El servicio autónomo Matter-AI sigue sin activarse hasta autorizarlo.'
}

main "$@"
