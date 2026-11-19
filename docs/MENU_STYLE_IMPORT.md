## Citas de WhatsApp incorporadas (2026-11-19)

`format_segment("Línea uno\\nLínea dos", TextFormat::Quote)` produce dos líneas con `> ` al inicio para que WhatsApp las represente como citas. Compatible con texto de código inline; no es HTML ni ejecuta comandos. El alfabeto y el marco siguen disponibles para mensajes independientes del menú.

## 2026-11-19 — Tipografía WhatsApp y categorías vacías ocultas (alpha)

- Idea de Dev MS Leito Delfin 3 Siss: separar decoración y texto funcional usando negrita `*texto*`, código inline \`texto\` y bloque \`\`\`texto\`\`\`; no enviar los marcadores como instrucciones a ejecutar.
- El catálogo contractual de 21 rutas permanece registrado. La **lista visible** se calcula únicamente a partir de descriptores `implemented: true`; `Todo` conserva la vista agregada. Una categoría vacía no aparece en `=/>menu`, `menu all` ni en el selector NativeFlow «Categorías».
- `templates/menu-visible-categories.json` es el manifiesto compartido del selector Rust. `src/menu.rs` valida por prueba que coincide con el inventario real: una implementación nueva exige actualizar el manifiesto en el mismo cambio. Las páginas históricas pueden seguir aceptándose por compatibilidad, sin mostrar filas vacías.
- Los nombres de categorías aparecen en negritas WhatsApp y las rutas de comandos usan monoespaciado; la cenefa ornamental se conserva. `style none` permanece en texto plano.
- Se guardan cinco alfabetos Unicode y los formatos de texto en `templates/message-alphabets.json`, la plantilla `templates/message-frame.txt`, y el renderizador reutilizable `src/message_typography.rs`. Accentos se conservan cuando no existe glifo equivalente.
- Seguridad sin cambios: botones sólo de lectura, ninguna categoría gana permisos, filtros Anti siguen OFF y no se activa Bot-Matter. Requiere CI Linux/Windows/proveedor y prueba real del teléfono.
- README revisados: 2/2. CI del nuevo SHA: pendiente.

### Uso genérico de formato

`message_typography::font_text("Aviso", "sans_bold")` cambia el alfabeto visible.
`message_typography::format_segment("=/>status", TextFormat::InlineCode)` formatea instrucciones como texto.
`message_typography::render_message("Aviso", "Texto normal", "serif_bold")` coloca un mensaje arbitrario en la cenefa común.
La biblioteca no envía mensajes ni autoriza comandos. Los cinco alfabetos soportados son `normal`, `serif_bold`, `sans_bold`, `monospace` y `fullwidth`.

# Base-Matter — Menú universal y estética externa

`=/>menu`, `=/>help` y `=/>h` son comandos reales de Base-Matter
con una respuesta de texto simple que enumera solo capacidades activas.
La identidad visual no se inventa en el núcleo.

## Importación futura, exclusivamente estética

1. Identificar un bot externo del que el propietario quiera copiar el
   estilo visual. Obtener una respuesta **real** de su comando `menu`
   mediante un canal autorizado o un ejemplo proporcionado voluntariamente.
2. Analizar solo marco, separadores, iconos, posición de etiquetas y
   cabeceras sin reproducir contenido textual o lista de comandos.
3. Proponer una vista previa con los comandos de Base-Matter y pedir
   aprobación antes de usar ese tema en una comunidad.
4. Validar Unicode y tamaño, almacenar tema por comunidad, conservar
   el marco de autorización y ofrecer reversión al listado plano.

**No implementado:** observador de mensajes de otros bots, petición de
sus menús ni importador persistente. No es posible copiar fielmente
un diseño concreto sin una respuesta de referencia. Esta limitación
es deliberada: no asumir origen de mensajes ni leer conversaciones
ajenas sin autorización.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
