# LuferOS/Baileys-Rust — procedencia y límites del snapshot

Este directorio contiene una **copia literal no modificada** y **no compilada** de archivos del proyecto independiente **Baileys-Rust**, mantenido por Luis Fernando Guzmán Niño (**LuferOS**).

| Campo | Valor |
|---|---|
| Origen | https://github.com/LuferOS/Baileys-Rust |
| Archivo | `src/Utils/experiments.rs` |
| Commit fijado | `d8e1d41d1b2bb2301385aaa28e196c0fb6855367` |
| Blob SHA de código | `d9ce040e2c7f0d5b06950860bca9eae594f49651` |
| Licencia | Apache License 2.0 (incluida íntegra en `LICENSE`) |
| Aviso de autoría | `NOTICE`, conservado íntegramente |

La inclusión es para **revisión y portabilidad eventual**, sin integración activa en Base-Matter. **El archivo es propiedad intelectual del proyecto original**, no se le atribuye autoría a Matterssmith. Mantener avisos de copyright, licencia, NOTICE y avisar expresamente en cada archivo modificado si se llega a hacer una obra derivada.

## Qué funciona en el código original

El constructor produce estructuras protobuf de botones/NativeFlow, WebView por URL, carruseles y RichResponse con bloques de texto, código, tablas y LaTeX. También construye nodos binarios auxiliares `biz` y `bot` y un secret aleatorio. Los tests internos verifican que hay un nodo para un WebView y RichResponse; **no realizan un envío de red ni garantizan renderizado en un teléfono**.

El código **no interpreta HTML**, no empaqueta páginas HTML arbitrarias en WhatsApp y no prueba que el cliente admita JavaScript dentro del mensaje. `open_webview` indica abrir una URL; el renderizador y la política dependen del cliente WhatsApp.

## Incompatibilidad actual

- El código fuente externo requiere `baileys-rs`, `crate::proto`, `prost`, `crate::wa_binary`, `rand` y `serde_json`.
- El proveedor vivo de Base-Matter depende de `whatsapp-rust =0.7.0` y `buffa`/proto `wa::`; los tipos no son intercambiables ni una sustitución directa.
- El provider WhatsApp en Base ya tiene `rich_preview.rs` y `rich_transport.rs`, con ensayo opt-in en self-chat y fallback, sin visualización Rich HTML confirmada.

## Condiciones para una adaptación futura

1. Crear una **traducción original** de los tipos de la referencia al protobuf del proveedor instalado o usar un plugin/bridge aislado que mantenga la versión/licencia de LuferOS.
2. Verificar que `whatsapp-rust` ofrece tipos para cada primitive, además del árbol binario y empaquetado real.
3. Ejecutar pruebas de serialización/round-trip y controles de longitud, URLs HTTPS, permisos y origen. Nunca suplantar cuentas `0@bot` o marcas de Meta AI.
4. Probar explícitamente envío y visualización en Android, iOS y WhatsApp Web, con fallback de texto. No promover el experimento a grupos o producción sin esos resultados.

Créditos de integración y auditoría:
Hecho por Lournyx Matterssmith [Lou-Nyx].
Autor y copyright del software de referencia: Luis Fernando Guzmán Niño (LuferOS).
