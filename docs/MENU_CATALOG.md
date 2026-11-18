# Menú modular de Base-Matter — categorías, estilos y compatibilidad

**Rama de desarrollo:** `alpha`. Esta revisión incorpora un catálogo
Rust reutilizable y un menú dinámico real en el controlador de WhatsApp.
No debe confundirse un descriptor del catálogo con la implementación
operativa de un comando ni con su autorización.

## Un archivo por comando, una carpeta por categoría

`src/menu.rs` define los contratos públicos `CommandSpec`,
`Category`, `Style`, `Language` y `MenuPreferences`. Cada
subdirectorio de `src/menu/` contiene su propio `mod.rs` y un archivo
Rust por comando:

- `status/status.rs`
- `bot/bot.rs`
- `set/set.rs`, `set/set_style.rs`, `set/set_lang.rs`,
  `set/set_role.rs`, `set/set_anti.rs`
- `antis/antis.rs`, `antis/anti.rs` y **un archivo independiente
  por cada uno de los 32 filtros** del catálogo existente
  `anti::RULES`
- `roles/roles.rs`, `roles/audit.rs`
- `system/system.rs`, `system/update.rs`,
  `system/restart.rs`, `system/reload.rs`
- `sandbox/sandbox.rs`, `sandbox/test.rs`,
  `sandbox/eval.rs`, `sandbox/console.rs`
- `ai/matter.rs`, `ai/help.rs`
- `navigation/menu.rs`, `navigation/credits.rs`

Cada archivo contiene **metadatos**: nombre, descripción español/inglés,
variantes de uso y estado de implementación. Los módulos son públicos,
permitiendo que un bot ajeno consulte los contratos sin recibir permiso
para ejecutar controles del propietario. Un nuevo comando se incorpora
al `mod.rs` de su categoría; el renderer y el conteo se actualizan
sin editar la plantilla principal ni duplicar archivos de menú.

**El código de ejecución y autorización permanece en el controlador,
AntiStore, HMAC bridge, Vault, contratos y proveedores Rust existentes.**
Los tipos no se convierten en operaciones ejecutables por figurar en
el menú. Las entradas marcadas `[pendiente]` son explícitamente
no operativas.

## Menú y páginas

La sintaxis usa el **prefijo global una sola vez** en el encabezado:

```text
menu
menu status
menu bot
menu set
menu antis
menu antis 2
menu roles
menu system
menu sandbox
menu ai
menu navigation
```

El controlador usa `menu::render`, no
`templates/matterssmith-menu.txt`, en modo de producción. Cada
página detalla un máximo de siete comandos y muestra la navegación
a la siguiente. El menú principal muestra todas las categorías.
No añade marca personal ni créditos al final.

## Estilo como una capa independiente

```text
set style classic
set style none
set lang es
set lang en
```

Solo el propietario autenticado del socket, desde su chat privado,
puede modificar estas opciones. Se guardan en un SQLite **distinto**
de la sesión, en la ruta del controlador con extensión
`.menu.sqlite`, y se conservan tras reiniciar.

- `classic`: usa los separadores solicitados `╞═╦═`,
  `│♪║`, `│♪╠`, `│♪╚`, título y líneas divisoras.
- `none`: presenta nombre, descripción y sintaxis sin símbolos
  decorativos.
- `es` y `en`: seleccionan las descripciones; el prefijo
  del control no se traduce.
- Los estilos personalizados de comunidades siguen aislados.
- Ningún estilo puede modificar roles, la configuración de Vault,
  el origen del mensaje ni los permisos del bot.

## WhatsApp: botones e HTML pequeño

La opción `MATTER_WA_NATIVE_BUTTONS=1` intenta enviar un único mensaje
que contiene el menú y tres respuestas rápidas reales:

- **Estado** → `=/>status`
- **Menú** → `=/>menu`
- **Antis** → `=/>menu antis`

Al pulsarlos, el evento se traduce a uno de esos tres comandos
**de solo lectura** y vuelve a pasar por la firma del puente,
autenticación PN/LID, reglas de chat y antirreplay. Ninguna
pulsación asigna rangos, inicia servicios, modifica Vault o
ejecuta comandos arbitrarios.

El texto auxiliar usa el conversor seguro `short_html_to_whatsapp`
para **formato breve tipo HTML**, que termina en marcas nativas de
WhatsApp. HTML real embebido sigue en el experimento
`MATTER_WA_RICH_HTML=1` *solo para el chat propio*, sin garantía
de visualización en Android/iOS. Si el transporte lo acepta y la
aplicación lo ignora, no existe fallback de visualización automática.

## Pendientes y seguridad

`system update/restart/reload`, `console`, `eval`,
`test`, `sandbox`, recepción de adjuntos citados y
`help` IA automática son **pendientes** hasta disponer de
implementación separada, permisos, revisión y pruebas.
`matter` consulta al broker Vault para propuestas de código,
pero sigue sin API activa ni escritura o ejecución automáticas.

Vault-Matter conserva el registro de **Aura Reed/Alya Core**
como permiso de IA **pendiente**: sin claves, grant, acceso
al modelo ni roles supuestos. Los perfiles GitHub del autor
identificado y solicitante se encuentran únicamente en el
registro de permisos, no en este menú.

No promover a beta sin CI Linux/Windows/proveedor y una prueba
manual confirmada en WhatsApp.
