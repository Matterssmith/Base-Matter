# Menú Matter desplegable para WhatsApp — alpha

Estado: cambio de código en `alpha`, **sin despliegue en idkbot ni verificación visual en teléfono**. Fecha: 2026-11-24.

## Contrato visible solicitado

- `=/>menu` / `=/>menú` presenta una tarjeta **breve** con dos controles nativos: selector `Categorías` y respuesta rápida `Ver texto`. No muestra inmediatamente el menú largo, las métricas del propietario ni el catálogo completo.
- Al seleccionar una fila del selector, el ID autorizado se resuelve mediante la lista fija en `providers/whatsapp-rust/src/ui.rs`; la respuesta ejecuta un comando **de lectura** como `=/>menu anti`, que entrega únicamente el detalle pedido. No se permite ejecutar una acción a partir de los títulos de las filas.
- El botón `Ver texto` usa exclusivamente `matter.read.menu.all` → `=/>menu all`. El menú completo aparece **solo cuando se solicita**.
- Las páginas del selector mantienen un cuerpo breve y las secciones con comandos no inventados. La selección está sujeta al estado del chat, identidad, HMAC y protección antirreplay existentes; las respuestas de texto siguen siendo fallback.
- La presentación de los mensajes nativos queda habilitada por defecto **solo al actualizar al binario de alpha**. Para desactivarla por incompatibilidad visual, definir `MATTER_WA_NATIVE_BUTTONS=0` y reiniciar el proveedor **con el procedimiento de recuperación existente** (sin crear otra sesión).
- Si el envío del selector falla, se envía el texto autenticado normal. Si WhatsApp acepta el envío pero **no dibuja** el botón, el fallback **manual** es escribir `=/>menu all` o `=/>menu anti`. No podemos detectar de forma fiable el renderizado en el móvil.

## Diferencia frente a HTML

WhatsApp no ejecuta HTML arbitrario dentro de mensajes de un bot. `native_flow_message` genera controles **de WhatsApp**, no un documento HTML. Las experiencias WhatsApp Flows oficiales requieren su infraestructura y cuentas habilitadas; sus pantallas se describen en Flow JSON y tampoco son HTML libre.

`providers/whatsapp-rust/src/rich_preview.rs` ofrece una **vista previa HTML para navegador**, con `<details>` cerrado por defecto y el texto visible al pulsar «Mostrar menú de texto». Se obtiene con `--rich-preview-html` y no se publica automáticamente como mensaje. El transporte experimental `AIRichResponseMessage` existente es solo un ensayo de propietario en chat propio, nunca una función oficial garantizada. No suplantar Meta AI, no activar HTML experimental en grupos y no inferir renderizado a partir del éxito del envío.

## Validación y despliegue seguro

1. Desde `Base-Matter/alpha`, compilar y ejecutar pruebas del proveedor con Rust:
   `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test ui`
   y `cargo test --manifest-path providers/whatsapp-rust/Cargo.toml --test rich_preview`.
2. Probar en Android / iOS / Web `=/>menu`, selector, cambio de página, categoría y botón «Ver texto»; confirmar que **la respuesta del botón** llega al controlador y que el ID no concede permisos.
3. Probar `MATTER_WA_NATIVE_BUTTONS=0`: el menú completo en texto se debe conservar. Verificar también `=/>help` independiente del selector, menús de propietario sin fugas y chat desactivado sin nuevos permisos.
4. Revisar CI Linux/Windows, formato Rust y pruebas de seguridad. El cambio en GitHub **no modifica el binario ya ejecutándose**.
5. Para cambiar el proveedor de `idkbot`, usar el procedimiento documentado de `scripts/whatsapp-alpha-existing.sh`, una sola sesión existente y `--resume-only` cuando corresponda. No borrar almacén de sesión, no generar pairing nuevo, no duplicar proveedor ni reiniciar Vault. Hacer rollback al binario anterior si no se muestra el selector.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
