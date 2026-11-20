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
if [ -f "$source_dir/matter_branches.py" ]; then
    install -m 0700 -- "$source_dir/matter_branches.py" "$target_dir/matter_branches.py"
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
