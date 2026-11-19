## 2026-11-19 — Burbuja de selección y tarjetas compatibles

- Reporte visual Android: al pulsar «Categorías», el cliente publicaba «☾︎ Anti / Abrir menú» en vez del comando elegido. El selector ahora usa `title: "=/>menu anti"` y elimina `description`. El ID opaco y su enrutamiento HMAC siguen intactos; la burbuja depende de cómo WhatsApp represente esa fila.
- `MATTER_WA_MENU_LAB=1` añade tarjetas NativeFlow de prueba TS18/TS19 en menús raíz de **chats privados habilitados**. Sus botones únicamente emiten ACK de lectura y pasan por las comprobaciones existentes. Si el envío nativo falla, se intenta la tarjeta legacy.
- El chat privado del propietario compara adicionalmente el transporte legacy y sigue siendo el único lugar que puede ensayar Rich HTML y audio. No se envía HTML, multimedia ni comparaciones de legacy a lectores externos o grupos.
- RichResponse HTML no equivale a HTML web y el cliente puede descartarlo sin dar error: estado **experimental sin visualización confirmada**. El HTML offline continúa disponible.
- No se modifican roles, antis, activación, secretos ni derechos de ejecución. Bot-Matter permanece bloqueado. CI Linux/Windows/proveedor y prueba de teléfono para este SHA pendientes. README revisados: 2/2.

# Base-Matter alpha — probar el menú en WhatsApp

## Nueva prueba de cita e instrucciones compactas (2026-11-19)

En el teléfono, abre `=/>menu` y comprueba que «Instrucciones» tiene **cuatro líneas** breves, sin espacios grandes entre ellas. Todas comienzan con `> ` en el texto enviado; WhatsApp debería representarlas como una cita. El contenido resume status con bot apagado, identidad administrativa, externos OFF y sockets/auditoría. En `style none` el contenido sigue breve, sin decoración ni citas.

Las capturas aportadas verifican el menú y el botón en Android anteriores a este cambio; falta confirmar el nuevo formato citado. Los botones TS18/TS19, audio y HTML experimental no cambian.



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

Con `MATTER_WA_MENU_LAB=1`, el menú raíz en un **chat privado habilitado**
puede añadir dos tarjetas NativeFlow: **TS18** (Audio, MP4) y **TS19**
(Audio, MP4, Enlace). Esas respuestas son únicamente de diagnóstico.
En el **self-chat autenticado del dueño** se añade la comparación
con tarjetas clásicas ButtonsMessage y un sonido Ogg/Opus original
reproducible al tocarlo. Solo allí se intenta HTML RichResponse
si `MATTER_WA_RICH_HTML=1`; WhatsApp puede ignorarlo aun si el
envío se acepta. Ni HTML ni sonido se ejecutan automáticamente.
No se envían tarjetas experimentales automáticamente a grupos.

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
