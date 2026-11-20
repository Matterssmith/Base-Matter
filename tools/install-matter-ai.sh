#!/usr/bin/env bash
# Matter-AI — instalador local sin root, Node ni Codex.
# Créditos:
# Hecho por Lournyx Matterssmith [Lou-Nyx].
set -euo pipefail

source_dir="$(cd -- "$(dirname -- "$0")" && pwd)"
source_agent="$source_dir/matter_ai.py"
target_dir="$HOME/.local/share/matter-ai"
binary_dir="$HOME/.local/bin"
target_agent="$target_dir/matter_ai.py"
target_bin="$binary_dir/matter-ai"

if ! command -v python3 >/dev/null 2>&1; then
    printf '%s\n' 'ERROR: Ubuntu requiere python3. Instálalo con apt e intenta otra vez.' >&2
    false
fi

if ! command -v git >/dev/null 2>&1; then
    printf '%s\n' 'AVISO: falta Git; instalarlo permite descargar workstreams.'
fi

install -d -m 0700 -- "$target_dir" "$binary_dir"
install -m 0700 -- "$source_agent" "$target_agent"
if [ -f "$source_dir/matter_local.py" ]; then
    install -m 0700 -- "$source_dir/matter_local.py" "$target_dir/matter_local.py"
fi
if [ -f "$source_dir/matter_branches.py" ]; then
    install -m 0700 -- "$source_dir/matter_branches.py" "$target_dir/matter_branches.py"
fi
if [ -f "$source_dir/matter_autopilot.py" ]; then
    install -m 0700 -- "$source_dir/matter_autopilot.py" "$target_dir/matter_autopilot.py"
fi
if [ -f "$source_dir/matter-autopilot.service" ]; then
    install -d -m 0700 -- "$HOME/.config/systemd/user"
    install -m 0600 -- "$source_dir/matter-autopilot.service" "$HOME/.config/systemd/user/matter-autopilot.service"
fi
if [ -f "$source_dir/../docs/MATTER_AI_HANDOFF.md" ]; then
    install -m 0600 -- "$source_dir/../docs/MATTER_AI_HANDOFF.md" "$target_dir/MATTER_AI_HANDOFF.md"
fi
cat > "$target_bin" <<'LAUNCHER'
#!/usr/bin/env sh
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
exec python3 "$HOME/.local/share/matter-ai/matter_ai.py" "$@"
LAUNCHER
chmod 0755 -- "$target_bin"
cat > "$binary_dir/matter-autopilot" <<'AUTOPILOT'
#!/usr/bin/env sh
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
exec python3 "$HOME/.local/share/matter-ai/matter_autopilot.py" "$@"
AUTOPILOT
chmod 0755 -- "$binary_dir/matter-autopilot"

printf '\n%s\n' 'Matter-AI instalado para este usuario, sin sudo.'
printf 'Ejecutable: %s\n' "$target_bin"
printf '\n%s\n' 'Comandos iniciales:'
printf '%s\n' '  ~/.local/bin/matter-ai setup'
printf '%s\n' '  ~/.local/bin/matter-ai projects'
printf '%s\n' '  ~/.local/bin/matter-ai sync matter-rust'
printf '%s\n' '  ~/.local/bin/matter-ai save matter-rust'
printf '%s\n' '  ~/.local/bin/matter-ai promote matter-rust beta (crea PR draft si beta existe)'
printf '%s\n' '  ~/.local/bin/matter-ai'
printf '\n%s\n' 'La clave API no se almacena en GitHub. Los repos privados requieren credenciales Git por separado.'
printf '%s\n' 'Si tu PATH incluye ~/.local/bin podrás escribir simplemente: matter-ai'

printf '%s\n' '  ~/.local/bin/matter-autopilot preview    # comprobación sin API'
printf '%s\n' '  ~/.local/bin/matter-autopilot configure  # límites de API y proyectos'
printf '%s\n' '  ~/.local/bin/matter-autopilot start      # solicita autorización para trabajar'
printf '%s\n' '  ~/.local/bin/matter-autopilot status / stop'
printf '%s\n' 'Servicio systemd de usuario instalado, pero NO activado automáticamente.'

printf '%s\n' 'Modo local sin coste por llamada:'
printf '%s\n' '  Instala Ollama desde https://docs.ollama.com/linux'
printf '%s\n' '  ollama pull qwen3:0.6b'
printf '%s\n' '  ~/.local/bin/matter-ai backend local'
printf '%s\n' '  ~/.local/bin/matter-ai'
printf '%s\n' '  ~/.local/bin/matter-autopilot doctor'
printf '%s\n' 'Ollama local NO es GPT-6; en equipos de 4 GB se recomienda comenzar con qwen3:0.6b.'
