#!/usr/bin/env bash
# Smoke test: no WhatsApp, network, Git credentials or actual database.
# Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].
set -Eeuo pipefail
umask 077
root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd -P)"
source "$root/scripts/matter-supervised-run.sh"
temp="$(mktemp -d)"
trap 'rm -rf -- "$temp"' EXIT
old=1111111111111111111111111111111111111111
new=2222222222222222222222222222222222222222
make_snapshot() {
  local dir="$temp/$1"
  mkdir -m 700 "$dir"
  printf '%s\n' "$1" > "$dir/revision"
  printf '#!/usr/bin/env bash\nreturn 0\n' > "$dir/launcher.sh"
  printf '#!/usr/bin/env bash\ntrue\n' > "$dir/whatsapp-matter-control"
  cp "$dir/whatsapp-matter-control" "$dir/whatsapp-matter-rust-provider"
  chmod 700 "$dir/whatsapp-matter-control" "$dir/whatsapp-matter-rust-provider"
  (cd "$dir" && sha256sum -- revision whatsapp-matter-control whatsapp-matter-rust-provider launcher.sh > SHA256SUMS)
}
make_snapshot "$old"
make_snapshot "$new"
snapshot_valid "$temp/$old"
snapshot_valid "$temp/$new"
atomic_text "$temp/last-good" "$old"
[[ "$(cat "$temp/last-good")" == "$old" ]]
printf 'modified\n' >> "$temp/$new/whatsapp-matter-control"
if snapshot_valid "$temp/$new"; then
  printf '%s\n' 'FAIL: modified executable accepted' >&2
  return 1 2>/dev/null || false
fi
# Pruning never removes active or last-good snapshots.
prune_snapshots "$temp" "$new" "$old"
[[ -d "$temp/$old" && -d "$temp/$new" ]]
snapshot_valid "$temp/$old"
printf '%s\n' 'MATTER_RECOVERY_SMOKE=OK'
