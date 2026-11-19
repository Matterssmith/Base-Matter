# Base-Matter alpha — probar el menú en WhatsApp

## Inicio en Debian (solo una instancia por sesión)

```bash
cd /home/Sakura/Base-Matter
git status --short --branch
git fetch origin
git switch alpha
git pull --ff-only origin alpha
export MATTER_WA_NATIVE_BUTTONS=1
export MATTER_WA_MENU_LAB=1
export MATTER_WA_RICH_HTML=1
cargo run --locked
```

Si Git encuentra cambios/conflictos, no descartarlos. Si otra instancia
usa el mismo SQLite, detenerla con Ctrl+C antes. El lanzador inicia
controlador Rust y proveedor Rust, recupera la sesión o muestra el
pairing code de 8 caracteres. No se usa QR ni Node.js.

## Prueba en el chat propio de WhatsApp

```text
=/>status
=/>bot on
=/>menu
=/>menu anti
=/>menu all
=/>menu config
=/>help
=/>help antilink
```

`=/>menu` muestra primero el diseño del usuario y únicamente las categorías con comandos implementados,
con un solo botón **Categorías**. Las 21 entradas se conservan en el
catálogo interno; las vacías no aparecen en el menú ni en el selector.
Las rutas históricas de paginación siguen aceptándose para compatibilidad.

Con `MATTER_WA_MENU_LAB=1`, **solo en el chat privado verificado del
dueño**, el mismo comando también prueba mensajes adicionales:
- HTML RichResponse de pantalla experimental, solo si
  `MATTER_WA_RICH_HTML=1`. WhatsApp puede ignorarlo.
- Tarjeta TS18, con botones **Audio** y **MP4**.
- Tarjeta TS19, con **Audio**, **MP4** y **Enlace**.
- Sonido original de 0.45 segundos enviado por la API de audio
  cifrado de WhatsApp (Ogg/Opus). Se escucha pulsando reproducir;
  **no** se reproduce automáticamente en el menú ni en HTML.

Las pulsaciones deben devolver `=/>test ack audio|mp4|link`.
Son pruebas de transporte. No descargan archivos, no ejecutan código,
no alteran roles. `=/>test ui` y `=/>test row` siguen funcionando
como diagnósticos independientes.

El menú no explica la implementación; `=/>help` indica únicamente
cómo solicitar ayuda. `style none` conserva el menú sin decoración.
No se muestran categorías vacías. Los nombres visibles están en negritas;
las rutas de comandos, entre acentos graves de código WhatsApp.
Los cinco alfabetos y la plantilla reutilizable están en
`templates/message-alphabets.json` y `templates/message-frame.txt`.

**Aceptación real pendiente:** capturas del selector, TS18, TS19,
HTML, y audio en Android. Las pruebas GitHub Actions verifican
compilación y contratos offline, no lo que muestra el cliente.
Si el renderizado falla, el menú de texto permanece disponible.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
