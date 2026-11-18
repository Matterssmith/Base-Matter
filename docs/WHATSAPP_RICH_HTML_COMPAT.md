# Base-Matter — Compatibilidad de HTML enriquecido en WhatsApp

**Estado:** investigación offline en `alpha`. No es una capacidad de envío
HTML real y no habilita ninguna acción por defecto.

## Qué reveló el material facilitado

El archivo de prueba `xd.zip` del 18-11-2026 incluye Go, JavaScript y HTML
para **menús, juegos y pantallas Rich**. Los principales componentes observados:

- `help_rich.go` genera una plantilla HTML con `html/template`; intenta
  representar una hoja mediante `embedded_screens` y
  `FOAIDNixelButtonSheets`. Advierte que el menú animado no está disponible
  en WhatsApp Web/iPhone en esa implementación.
- `commands_juegos_rich.go` monta `GenAIaeacdsnwHtmlPrimitive` dentro
  de `sections` y `embedded_screens`. Depende de seis HTML en
  `assets/` que **no** están incluidos en el ZIP suministrado.
- `testeos.js` construye `unifiedResponse.data` (JSON codificado en
  base64), con contenidos de HTML/markdown y media. Para enviar, envuelve
  la respuesta con `botForwardedMessage.richResponseMessage`, marca el
  contenido como reenviado y atribuye un supuesto bot `0@bot`.
  Ese origen **no se debe reproducir** en Base-Matter.
- `menu_rich.html` puede servir de inspiración visual. Los HTML y JS
  independientes son muestras de interfaz, no plugins de Base-Matter.

Los nombres `GenAIaeacdsnwHtmlPrimitive` y
`FOAIDNixelButtonSheets` son experimentales; que existan en un JSON
no prueba que un cliente WhatsApp los renderice, ni que una cuenta ajena
a Meta AI tenga permiso para enviarlos.

## Lo que realmente se puede aprovechar

| Componente | Reutilización en Base-Matter |
| --- | --- |
| Menú, CSS y diseño responsive | Vista previa HTML **local** y estática |
| Escape de HTML de plantillas | Escape propio en Rust, sin ejecutar JS |
| JSON con `embedded_screens` y `sections` | Prueba de esquema, **sin enviar** |
| NativeFlow protobuf de `whatsapp-rust` 0.7 | Tres botones reales de solo lectura, opt-in |
| BotJID `0@bot` y `isForwarded` falsificados | **Rechazado**, no suplantar origen |
| Scripts HTML, solicitudes HTTP, audio y juegos | Fuera del núcleo y sin ejecución |
| Supuesta detección Android/iOS por JID | **Rechazada**, no es prueba de dispositivo |

## Prototipo sin WhatsApp

`providers/whatsapp-rust/src/rich_preview.rs` construye dos resultados
**sin requerir variables de entorno, llaves, bases de sesión ni red**:

```bash
cd /home/Sakura/Base-Matter
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-html > matter-preview.html
cargo run --manifest-path providers/whatsapp-rust/Cargo.toml -- --rich-preview-json > matter-preview.json
```

La primera salida es HTML estático, con el **menú original completo**,
texto escapado y tres botones de apariencia deshabilitados. La segunda es
JSON experimental `embedded_screens`/`FOAIDNixelButtonSheets`, sin
`botForwardedMessage`, sin fingir un bot de Meta AI y sin ejecutarse
como mensaje WhatsApp.

Las pruebas impiden scripts, atributos inyectados, datos peligrosos,
sobrecargas y encabezados falsificados en el JSON de ejemplo.
**No se copia ni ejecuta el código del ZIP.** El HTML de vista previa
es un documento local; los botones NativeFlow se prueban por separado.

## Prueba futura legítima

Para poder prometer HTML dentro de WhatsApp hace falta confirmar una
API documentada o un mecanismo legítimo de renderizado para terceros.
El simple envío de `richResponseMessage` no demuestra soporte de
HTML interactivo. Una prueba real deberá incluir Android, iPhone y
Web, sin suplantación, con fallback al menú de texto original.

```
Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
```
