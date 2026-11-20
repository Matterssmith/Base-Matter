# WhatsApp-Matter Rust: vínculo por código y recuperación del estado real

## 2026-11-20 — Incidente del servidor instabox-se

Se confirmó `matter-vault.service` con IPC autenticado al cliente `base-wa-test-1` de `wa-test-1`. Ya recuperados 4.7 GB, binarios controlador/proveedor para Base alpha `158eb4792cf1bf42b0b6aae70b1ffa90a4d4d8a3` con CI correctos. El lanzador `scripts/whatsapp-vault-existing.sh` pasó `--check` de ambos y arrancó controlador; el proveedor informó `WhatsApp-Matter Rust session starting. Pairing code only.` y `Pairing code request failed or was rate limited`. **No hubo `WhatsApp session connected`, por tanto el proveedor no autentica WhatsApp y no se ha probado el comando en el teléfono.**

La frase del error era ambigua: `on_pair_code_error` descartaba la categoría del proveedor. Ahora distingue `BadRequest/RateOverlimit` (posible límite de solicitudes), `FeatureNotAvailable`, `Forbidden`, otros rechazos y eventos sin clasificación; omite valores sensibles y no propone reintentos automáticos. El upstream `whatsapp-rust 0.7.0` salta solicitud de pairing si hay una sesión autenticada restaurada. Una base SQLite existente **no demuestra** credenciales de un dispositivo vinculado.

Se detectó otra omisión: `signed_verified_control` no permitía `=/>vault` para el chat propio, pese a que el controlador ya lo reconocía. Se agregó sólo a esa lista privada y se probó la firma del propietario y el rechazo de grupo/terceros. No se autoriza `=/>vault` públicamente.

## Diagnóstico offline obligatorio (sin contraseñas, JID ni registros)

Sin iniciar el proveedor ni pedir más códigos:

```bash
bash /home/Sakura/Base-Matter/scripts/whatsapp-vault-existing.sh --help
```

La comprobación anterior es sólo ejemplo de ruta; el lanzador requiere dos binarios y no debe iniciarse para diagnosticar. En su lugar usar exclusivamente `python3 scripts/whatsapp-session-inspect.py`, que no carga `runtime.env`, no lee contraseñas ni publica datos de SQLite y abre la base en modo lectura. El informe indica tamaño de las bases, integridad y si hay registros de dispositivo sin imprimir su contenido. Es sólo una pista: no demuestra que WhatsApp acepte esas credenciales.

Si el almacén original no tiene dispositivo, o la vinculación fue invalidada en el teléfono, se necesitará autorización expresa del propietario y una vinculación por código **una sola vez después de esperar el intervalo que indique WhatsApp**; no recrear bases de datos ni repetir el proceso por adivinanza. Si existen credenciales, diagnosticar la versión/proveedor/conexión o un logout sin borrar contenido.

No ejecutar `cargo run`, no borrar `session.db*`, `runtime.env`, locks ni bases del controlador. No activar Bot-Matter ni editar ramas beta/stable. El servicio Vault independiente continúa operativo.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
