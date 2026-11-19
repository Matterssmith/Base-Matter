## 2026-11-18 — Menú y laboratorio de interfaz en WhatsApp

El menú autorizado conserva las 21 entradas del contrato cerrado,
un botón de categorías y respuesta textual alternativa. Sólo en
self-chat verificado del propietario se activa MATTER_WA_MENU_LAB=1
para enviar tarjetas TS18 (Audio/MP4), TS19 (Audio/MP4/Enlace) y un
tono original de audio Ogg/Opus, además del HTML RichResponse cuando
MATTER_WA_RICH_HTML=1. El audio requiere pulsar reproducir;
HTML no ejecuta JavaScript ni garantiza soporte en WhatsApp.
Ningún click concede privilegios ni ejecuta descargas. Los comandos
en submenús sólo muestran sintaxis; help contiene los detalles.
Leer [WHATSAPP_UI_TESTS.md](WHATSAPP_UI_TESTS.md) para los pasos.

# Contrato cerrado de categorías — Base-Matter

## Separación obligatoria entre menú y ayuda (2026-11-18)

- `=/>menu` conserva el marco oficial y las 21 entradas exactas: sólo nombres, rutas y atributos verificables, sin explicación de ejecución.
- `=/>menu <categoria> [pagina]` enumera únicamente sintaxis/variantes de comandos disponibles. No imprimir descripciones, tutoriales ni comandos marcados pendientes. Las categorías sin comandos no llevan leyendas de vacío.
- `=/>menu all` presenta el índice de comandos disponibles, sin indicaciones de funcionamiento.
- `=/>help` **sin argumentos indica exclusivamente la sintaxis de ayuda** y no repite categorías, atributos de la sesión ni el marco principal.
- `=/>help <comando>` permite consultar descripción, estado y variantes. `=/>help <categoria> [pagina]` detalla comandos de esa categoría.
- Un comando sin argumentos y sin operación definida presenta **Uso** y las opciones permitidas. No debe devolver un menú de categorías como sustituto.
- `style none` conserva el mismo contenido informativo, sin arte Unicode. La separación menú/ayuda no cambia autorización, HMAC, roles o políticas.


## Formato obligatorio del menú global (noviembre 2026)

El comando `=/>menu` usa el encabezado `*꒦꒷✰꒷꒦︶꒦꒷✧꒷꒦︶꒦꒷✰꒷꒦*`, título de control global, las 21 categorías en orden, cierre `╰─╮╭────────⟨🌀`, autoridad descriptiva, panel del usuario/socket/rol/estado, instrucciones y cenefa final. El panel **debe** usar información autenticada del puente; un `Owner del socket` nunca implica por sí mismo `Owner Global` de Vault. La sección `Owner Global` describe la jerarquía, no prueba un rango concedido. No inventar alias WhatsApp ni mostrar PN/LID a terceros. `style none` mantiene categorías, rutas y estado en texto simple, sin cenefas. Se conservan **las 21 entradas incluso cuando alguna no tiene comandos**; no escribir «Sin comandos registrados» ni un rótulo de vacío. `menu all` omite encabezados sin comandos, y las páginas individuales no mencionan ausencia de comandos. El selector conserva un solo botón «Categorías» y páginas seguras. Toda respuesta debe cumplir límites de bytes del puente.


**Fuente de autoridad:** catálogo de 21 entradas entregado por la persona propietaria el 18 de noviembre de 2026. **Estado:** el catálogo visual y el selector con paginación ya están implementados en `alpha`; las carpetas de código heredadas aún conservan sus nombres internos. Falta confirmar CI completo de la última revisión y el comportamiento real de WhatsApp.

## Regla innegociable

- **Sólo** pueden existir las categorías/entradas del catálogo siguiente, en el **mismo orden**. No añadir "Bot", "Roles", "Sandbox", "Estado", "Navegación", "Ayuda" ni ninguna otra categoría fuera de la lista, aunque existan módulos internos con esos nombres.
- Los **comandos** se clasifican dentro de una entrada autorizada; un nombre de módulo, comando, plugin, prefijo o proveedor **no crea una categoría**. Se permite que una categoría esté vacía: mostrar "Sin comandos registrados" sin inventar funciones.
- **Todo** (`menu all`) es una **vista agregada** de los comandos de las otras categorías, no un espacio para alojar comandos exclusivos.
- Mantener **un único botón visible «Categorías»**, `single_select`, en el menú interactivo; no añadir botones paralelos «Ayuda», «Ver todo» ni más botones al menú. Los comandos escritos no son botones.
- Los filtros Anti están **apagados por defecto**. La clasificación nunca activa funciones, cambia permisos ni otorga roles.
- No cambiar la implementación de comandos existentes por una reorganización visual. Los alias de comandos o rutas anteriores pueden preservarse por compatibilidad, **pero jamás aparecen como categorías adicionales**.
- Los controles globales de Base-Matter usan `=/>`. El prefijo `#` del bot de plataforma es independiente; la fila «Conexiones» se mantiene `sockets` y el contrato del prefijo correspondiente debe verificarse antes de mostrarlo como activo.
- Si WhatsApp limita la cantidad de filas de `single_select`, paginar dentro del selector de **un solo botón** y mantener las 21 entradas accesibles; una fila «Más» sería navegación y **no** una nueva categoría.
- No añadir categorías automáticamente desde plugins, proveedores de IA, comandos importados o nuevas ramas. Una petición de crear otra categoría requiere modificación expresa de este contrato por la persona propietaria.

## Catálogo exacto (21 entradas, orden fijo)

| Nº | Categoría visible | Ruta canónica del menú | Identificador |
|---:|---|---|---|
| 1 | ⴵ ·· 𝙸𝙰 | `menu ia` | `ia` |
| 2 | ☾︎ ··· 𝙰𝚗𝚝𝚒 | `menu anti` | `anti` |
| 3 | ✦ · 𝚃𝚘𝚍𝚘 | `menu all` | `all` (vista agregada) |
| 4 | ⚙︎ · 𝙲𝚘𝚗𝚏𝚒𝚐𝚞𝚛𝚊𝚌𝚒ó𝚗 | `menu config` | `config` |
| 5 | ⇩ · 𝙳𝚎𝚜𝚌𝚊𝚛𝚐𝚊𝚜 | `menu downloads` | `downloads` |
| 6 | ◈ · 𝙴𝚌𝚘𝚗𝚘𝚖í𝚊 | `menu economy` | `economy` |
| 7 | ♫ · 𝙳𝚒𝚟𝚎𝚛𝚜𝚒ó𝚗 | `menu fun` | `fun` |
| 8 | ✰ · 𝙶𝚊𝚌𝚑𝚊 | `menu gacha` | `gacha` |
| 9 | 🎮 𝙹𝚞𝚎𝚐𝚘𝚜 | `menu games` | `games` |
| 10 | ♧ 𝙶𝚛𝚞𝚙𝚘 | `menu group` | `group` |
| 11 | ⓘ 𝙸𝚗𝚏𝚘𝚛𝚖𝚊𝚌𝚒ó𝚗 | `menu info` | `info` |
| 12 | 🔗 𝙴𝚗𝚕𝚊𝚌𝚎𝚜 | `menu links` | `links` |
| 13 | ✪ ·· 𝙿𝚛𝚒𝚗𝚌𝚒𝚙𝚊𝚕 | `menu main` | `main` |
| 14 | ◉ · 𝙼𝚘𝚍𝚘 | `menu mode` | `mode` |
| 15 | ♛ · 𝙿𝚛𝚘𝚙𝚒𝚎𝚝𝚊𝚛𝚒𝚘 | `menu owner` | `owner` |
| 16 | ♙ · 𝙿𝚎𝚛𝚏𝚒𝚕 | `menu profile` | `profile` |
| 17 | ♡ 𝚁𝚎𝚊𝚌𝚌𝚒𝚘𝚗𝚎𝚜 | `menu reactions` | `reactions` |
| 18 | ⌕ · 𝙱ú𝚜𝚚𝚞𝚎𝚍𝚊 | `menu search` | `search` |
| 19 | ⴵ · 𝙲𝚘𝚗𝚎𝚡𝚒𝚘𝚗𝚎𝚜 | `menu sockets` | `sockets` |
| 20 | ⚙︎ 𝚂𝚒𝚜𝚝𝚎𝚖𝚊 | `menu system` | `system` |
| 21 | ✧ 𝚄𝚝𝚒𝚕𝚒𝚍𝚊𝚍𝚎𝚜 | `menu utils` | `utils` |

## Diseño literal y único botón

```text
╭─💠 𝗖𝗮𝘁𝗲𝗴𝗼𝗿í𝗮𝘀
│·♪·╠[ ⴵ ·· 𝙸𝙰 [menu ia]
│·♪·╠[ ☾︎ ··· 𝙰𝚗𝚝𝚒 [menu anti]
│·♪·╠[ ✦ · 𝚃𝚘𝚍𝚘 [menu all]
│·♪·╠[ ⚙︎ · 𝙲𝚘𝚗𝚏𝚒𝚐𝚞𝚛𝚊𝚌𝚒ó𝚗 [menu config]
│·♪·╠[ ⇩ · 𝙳𝚎𝚜𝚌𝚊𝚛𝚐𝚊𝚜 [menu downloads]
│·♪·╠[ ◈ · 𝙴𝚌𝚘𝚗𝚘𝚖í𝚊 [menu economy]
│·♪·╠[ ♫ · 𝙳𝚒𝚟𝚎𝚛𝚜𝚒ó𝚗 [menu fun]
│·♪·╠[ ✰ · 𝙶𝚊𝚌𝚑𝚊 [menu gacha]
│·♪·╠[ 🎮 𝙹𝚞𝚎𝚐𝚘𝚜 [menu games]
│·♪·╠[ ♧ 𝙶𝚛𝚞𝚙𝚘 [menu group]
│·♪·╠[ ⓘ 𝙸𝚗𝚏𝚘𝚛𝚖𝚊𝚌𝚒ó𝚗 [menu info]
│·♪·╠[ 🔗 𝙴𝚗𝚕𝚊𝚌𝚎𝚜 [menu links]
│·♪·╠[ ✪ ·· 𝙿𝚛𝚒𝚗𝚌𝚒𝚙𝚊𝚕 [menu main]
│·♪·╠[ ◉ · 𝙼𝚘𝚍𝚘 [menu mode]
│·♪·╠[ ♛ · 𝙿𝚛𝚘𝚙𝚒𝚎𝚝𝚊𝚛𝚒𝚘 [menu owner]
│·♪·╠[ ♙ · 𝙿𝚎𝚛𝚏𝚒𝚕 [menu profile]
│·♪·╠[ ♡ 𝚁𝚎𝚊𝚌𝚌𝚒𝚘𝚗𝚎𝚜 [menu reactions]
│·♪·╠[ ⌕ · 𝙱ú𝚜𝚚𝚞𝚎𝚍𝚊 [menu search]
│·♪·╠[ ⴵ · 𝙲𝚘𝚗𝚎𝚡𝚒𝚘𝚗𝚎𝚜 [menu sockets]
│·♪·╠[ ⚙︎ 𝚂𝚒𝚜𝚝𝚎𝚖𝚊 [menu system]
│·♪·╚[ ✧ 𝚄𝚝𝚒𝚕𝚒𝚍𝚊𝚍𝚎𝚜 [menu utils]
╰───────────⟨ 🌀
[único botón interactivo: Categorías]
```

## Migración obligatoria de comandos registrados

Los módulos existentes `src/menu/*` pueden seguir como **archivos internos** durante la transición; sus nombres no se muestran como categorías. El controlador y la interfaz deben leer la clasificación canónica:

| Módulo histórico | Destino autorizado |
|---|---|
| `ai` | **IA**: `matter`; **Principal**: ayuda general `help` |
| `antis` | **Anti**: todos los filtros, `anti`, `antis` |
| `set` | **Configuración**: ajustes generales, idioma y estilo; **Anti**: `set anti`; **Propietario**: `set role` |
| `bot` | **Modo**: activación del bot y sus alcances |
| `roles` | **Propietario**: rangos y auditoría de autorizaciones |
| `status` | **Información**: estado y consultas |
| `navigation` | **Principal**: `menu`; **Información**: `credits` |
| `sandbox` | **Utilidades**: `sandbox`, `test`, `console`, `eval`; sólo si cuentan con su permiso correspondiente |
| `system` | **Sistema**: `system`, `update`, `restart`, `reload` |

Las entradas **Descargas, Economía, Diversión, Gacha, Juegos, Grupo, Enlaces, Perfil, Reacciones, Búsqueda y Conexiones** estarán vacías hasta que existan comandos **reales y autorizados** para ellas. No asignar por nombre o parecido si hay ambigüedad; clasificar de acuerdo con su función real.

## Verificaciones obligatorias antes de integrar en beta

1. Test de invariantes: el menú muestra **exactamente las 21 entradas, en este orden**, con identificadores únicos y ningún nombre adicional.
2. Todos los descriptores de comandos existentes aparecen en **una sola** categoría autorizada, salvo aliases que no se duplican en UI. `all` es agregado, no destino.
3. `menu anti`, `menu config`, `menu ia`, etc. renderizan contenido correcto; el enlace escrito anterior `menu antis`, `menu set`, `menu ai` puede redirigirse internamente.
4. `menu` en WhatsApp tiene **sólo** el botón «Categorías»; comprobar límites reales del selector antes de enviar 21 filas.
5. Se mantienen validación PN/LID, HMAC, antirreplay, owner/socket, bot OFF y Vault sin accesos añadidos.
6. Correr CI Rust Linux/Windows, CI proveedor WhatsApp y hacer prueba visual móvil. No declarar completo sólo por compilar.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
