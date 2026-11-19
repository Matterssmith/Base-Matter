# Ensayo desde WhatsApp: menú y botones en Base-Matter alpha

Estado: funciones Rust añadidas para prueba manual, NO verificación de renderizado real.

## Preparación segura

Usa la sesión de pruebas existente. El propietario puede probar en su chat privado o grupo verificado; otros usuarios sólo pueden probar NativeFlow y ButtonsRow en chats con el bot activado. HTML permanece restringido al chat privado del propietario.
No abras dos procesos sobre el mismo SQLite. Los scripts de Base-Matter siguen usando
código de vinculación por teléfono y no requieren QR.

Activa MATTER_WA_NATIVE_BUTTONS=1 al iniciar el proveedor, y deja
MATTER_WA_RICH_HTML=0 inicialmente. No es necesario reinicializar la sesión.

## Comandos para el propietario

- =/>menu : diseño original restaurado con nueve categorías reales, Todo y exactamente un selector «Categorías»
- =/>menu antis : resumen de protecciones
- =/>menu all : catálogo completo
- =/>help : navegación por categorías
- =/>help antilink : metadatos y uso del comando
- =/>help antis 2 : página de ayuda de categoría
- =/>test : qué pruebas visuales existen
- =/>test ui : envía 2 botones nativos y un selector de 2 filas
- =/>test row : envía tarjeta clásica de 3 botones: Audio, MP4, Enlace
- =/>test html : ensaya HTML solo con MATTER_WA_RICH_HTML=1

Pulsa **Botón A**, **Botón B**, **Fila uno** y **Fila dos**, uno por
uno; cada pulsación debe devolver un acuse diferente del controlador.
El acuse demuestra que llegó un control válido al HMAC tras autenticación,
pero un texto escrito manualmente con el mismo comando también produce
ese acuse: confirma visualmente que el origen fue la pulsación.

Si la plataforma acepta el mensaje y no muestra botones o HTML, la
compatibilidad del cliente no está demostrada. Reporta captura de pantalla,
versión de WhatsApp, el botón pulsado y el resultado, sin compartir
números de teléfono ni llaves privadas.

## Límites

- Los botones de ensayo solo solicitan información: no editan roles,
  activaciones, Vault, grupos ni anti.
- El mensaje Rich HTML puede ser ignorado por el cliente; la aceptación
  del transporte no confirma ejecución de JavaScript o sonidos.
- Pantalla completa, sonidos, stickers de rango y descarga no forman
  parte de esta prueba y no se anuncian como disponibles.
- No ejecutar código enviado por chat. Los tests de Rust
  siguen realizándose en CI, no dentro de mensajes de WhatsApp.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].

La variante `=/>test row` utiliza `ButtonsMessage` (familia clásica de
`SendButtonsRow` en Go). Esta prueba es deliberadamente de texto:
una miniatura real necesita un objeto de imagen cifrado y subido al
servidor de WhatsApp. No afirmar que este ensayo ya incluye miniatura.

`=/>test` sin argumentos debe responder incluso si el bot está apagado, indicando el estado. Si el chat está desactivado, `=/>test ui` y `=/>test row` no muestran botones. Revisa los logs del proveedor sin divulgar secretos.
