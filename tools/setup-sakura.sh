#!/usr/bin/env bash
# Matter-AI — provisión conservadora de usuario Ubuntu "Sakura".
# Créditos:
# Hecho por Lournyx Matterssmith [Lou-Nyx].
set -euo pipefail
umask 077

main() {
usage() {
    cat <<'HELP'
Uso: sudo bash tools/setup-sakura.sh [--password-prompt] [--authorized-key-file /ruta/id_ed25519.pub]
Por defecto se crea Sakura con contraseña BLOQUEADA, no vacía.
--password-prompt: solicita contraseña sin mostrarla (opcional).
--authorized-key-file: instala una clave PÚBLICA SSH (no privada).
Nunca crea sudoers sin contraseña; no toca sshd_config ni inicia servicios.
HELP
}

password_prompt=0
public_key_file=''
while [ "$#" -gt 0 ]; do
    case "$1" in
        --password-prompt)
            password_prompt=1
            shift
            ;;
        --authorized-key-file)
            if [ "$#" -lt 2 ]; then
                usage >&2
                printf 'ERROR: falta ruta de clave pública\n' >&2
                false
            fi
            public_key_file="$2"
            shift 2
            ;;
        --help|-h)
            usage
            return 0
            ;;
        *)
            usage >&2
            printf 'ERROR: opción no reconocida.\n' >&2
            false
            ;;
    esac
done
if [ -n "$public_key_file" ] && [ ! -f "$public_key_file" ]; then
    printf 'ERROR: no existe el archivo de clave pública\n' >&2
    false
fi

if [ "$(id -u)" -ne 0 ]; then
    printf 'ERROR: ejecutar con sudo para crear Sakura.\n' >&2
    false
fi

for command in useradd usermod passwd install chown chmod getent; do
    if ! command -v "$command" >/dev/null 2>&1; then
        printf 'ERROR: falta comando del sistema: %s\n' "$command" >&2
        false
    fi
done

if id Sakura >/dev/null 2>&1; then
    printf '%s\n' 'Usuario Sakura ya existe: se conserva UID, grupos y archivos.'
else
    useradd --create-home --shell /bin/bash --user-group Sakura
    printf '%s\n' 'Usuario Sakura creado con cuenta de contraseña bloqueada.'
fi

home_dir="$(getent passwd Sakura | cut -d: -f6)"
if [ -z "$home_dir" ] || [ "$home_dir" = "/" ] || [ -L "$home_dir" ]; then
    printf '%s\n' 'ERROR: home de Sakura no seguro.' >&2
    false
fi
install -d -m 0700 -o Sakura -g Sakura -- "$home_dir"
install -d -m 0700 -o Sakura -g Sakura -- "$home_dir/Matter"
install -d -m 0700 -o Sakura -g Sakura -- "$home_dir/Matter/workspaces"
install -d -m 0700 -o Sakura -g Sakura -- "$home_dir/Matter/checkpoints"
install -d -m 0700 -o Sakura -g Sakura -- "$home_dir/.config"
install -d -m 0700 -o Sakura -g Sakura -- "$home_dir/.config/matter-ai"

if [ -n "$public_key_file" ]; then
    # Permitir únicamente una línea simple de clave SSH pública (sin opciones de comando).
    key_line="$(cat -- "$public_key_file")"
    if [ "$(printf '%s\n' "$key_line" | wc -l)" -ne 1 ]; then
        printf '%s\n' 'ERROR: se requiere una sola clave pública SSH.' >&2
        false
    fi
    if ! printf '%s\n' "$key_line" | grep -Eq '^(ssh-ed25519|ssh-rsa|ecdsa-sha2-nistp(256|384|521)|sk-ssh-ed25519@openssh.com) [A-Za-z0-9+/=]+( .*)?$'; then
        printf '%s\n' 'ERROR: formato de clave pública SSH inválido.' >&2
        false
    fi
    if printf '%s\n' "$key_line" | grep -Eq 'PRIVATE KEY|BEGIN OPENSSH'; then
        printf '%s\n' 'ERROR: no se admiten claves privadas.' >&2
        false
    fi
    if [ -L "$home_dir/.ssh" ] || [ -L "$home_dir/.ssh/authorized_keys" ]; then
        printf '%s\n' 'ERROR: ruta SSH simbólica no autorizada.' >&2
        false
    fi
    install -d -m 0700 -o Sakura -g Sakura -- "$home_dir/.ssh"
    touch -- "$home_dir/.ssh/authorized_keys"
    chown Sakura:Sakura "$home_dir/.ssh/authorized_keys"
    chmod 0600 "$home_dir/.ssh/authorized_keys"
    if ! grep -Fxq -- "$key_line" "$home_dir/.ssh/authorized_keys"; then
        printf '%s\n' "$key_line" >> "$home_dir/.ssh/authorized_keys"
    fi
    printf '%s\n' 'Clave pública instalada; su llave privada permanece en tu equipo.'
fi

if [ "$password_prompt" -eq 1 ]; then
    if [ ! -t 0 ]; then
        printf '%s\n' 'ERROR: cambio de contraseña requiere terminal interactiva.' >&2
        false
    fi
    # passwd solicita dos veces la contraseña sin exponerla en comandos.
    passwd Sakura
else
    # Para cuentas nuevas useradd deja el password bloqueado por defecto.
    # Si la cuenta existía, NO cambia ni elimina su autenticación.
    printf '%s\n' 'Contraseña no alterada. Si Sakura es nueva, no admite login por contraseña.'
fi

printf '\n%s\n' 'Sakura preparada. No se concedió sudo sin contraseña.'
printf '%s\n' 'Para abrir sesión desde el administrador: sudo -iu Sakura'
printf '%s\n' 'Para acceso remoto, configura y comprueba SSH con clave pública o elige --password-prompt.'
printf '%s\n' 'Para instalar Matter-AI dentro de Sakura, clona Base-Matter desde esa sesión.'
printf '%s\n' 'Ningún proyecto fue borrado, fusionado ni actualizado automáticamente.'

}
main "$@"
