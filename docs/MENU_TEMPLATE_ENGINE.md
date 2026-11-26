## 2026-11-26 — Consola como referencia oficial del estilo Classic

El propietario prefiere el diseño de la consola y exige que su presentación también proceda de la plantilla. templates/menu/theme.json define el emblema único, prefijo del encabezado, ramas, líneas, cierre conectado con ✦ y conector centrado. Console, menú Classic y marco de control usan el mismo cierre conectado. El prefijo de las filas de comandos se comparte con las ramas de consola; las secciones informativas conservan su estructura y Pong su cierre histórico 🌀. La paleta ANSI, etiquetas y distribución de los datos de consola ahora también están en el JSON; la lógica solo selecciona colores semánticos y aporta datos. No se emiten secuencias ANSI en WhatsApp. Se aplicaron las cinco diferencias únicas de rustfmt de los CI del núcleo y proveedor del SHA 50905c6; también se adaptaron las dos aserciones del catálogo que exigían el cierre y conector antiguos, verificando su composición mediante la plantilla compartida. No se desactivan pruebas. CI nuevo pendiente; verificar ambos workflows antes de declarar aprobado. Cambiar las plantillas exige recompilar; se mantiene la alineación de dos celdas del emoji. No se modifican permisos, sesiones, Vault ni MSEL. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

# Motor de plantillas de menú Matter — contrato oficial

## 2026-11-26 — Plantilla clásica compartida y centrado del emblema

El propietario exige que los cambios de plantilla se propaguen a los consumidores del diseño clásico. La fuente común de adornos es templates/menu/theme.json (classic_frame y decoration); src/classic_template.rs la expande antes de insertar datos y se comparte con el proveedor autónomo. Menú Classic dinámico, menú de control heredado, marco de control, marco de mensajes, Pong y consola consumen esos componentes. Los layouts propios declaran contenido y estructura; los datos no se reinterpretan como adornos. El cierre conectado y el conector se centran en columnas 4–5, contando desde 1, suponiendo un emoji de dos celdas. Se añade el mismo patrón de ramas del menú a la consola. El menú plain y temas personalizados no se modifican. Los binarios deben recompilarse al modificar la plantilla; no hay hot reload. No se cambian permisos, Vault, MSEL ni sesión. El CI del SHA 53687dc aprobó ambos workflows; el nuevo cambio requiere verificación de formato, compilación y render real en SSH/WhatsApp. README revisados: 2/2.

Créditos: Hecho por Lournyx Matterssmith [Lou-Nyx].

## 2026-11-23 — Orden exacto del menú clásico

La vista clásica pública conserva obligatoriamente: (1) título + Categorías, (2) Información Matter inmediatamente después del conector «││», (3) ficha del usuario y socket, (4) Instrucciones. Los tres encabezados usan «╭─ 💠» (espacio U+0020 antes de 💠); el cierre de categorías usa la variante centrada compartida «╰──╮╭───────⟨ ✦» (espacio antes de 🌀). Hay una línea vacía entre Información Matter y la ficha de usuario, y otra entre la ficha y las Instrucciones, sin líneas triples. El propietario autenticado conserva sus bloques adicionales entre la ficha y las Instrucciones; los visitantes públicos nunca los reciben. No fijar datos dinámicos en la plantilla ni cambiar style none.

Estado: implementación en Base-Matter/alpha, sin despliegue de binarios. La fuente de verdad de las vistas dinámicas es templates/menu/, no el diseño copiado de un tercero ni cadenas decorativas dentro de src/menu.rs.

## Arquitectura: equivalente conceptual de HTML + estilos + datos

1. **Datos y categorías:** src/menu.rs conserva el catálogo de categorías y CommandSpec. El nombre mostrado viene de Category.es/en asociado al catálogo de comandos, independientemente del nombre del directorio o archivo Rust. Se agregan solo categorías con al menos un comando marcado implemented. El identificador all es una vista agregada, no una categoría física.
2. **Valores en vivo:** src/menu/theme.rs consulta la vista verificada (usuario, rango, estado, socket), command_count(), visible_category_ids() y MenuTelemetry::capture(). El número de comandos y categorías se calcula en cada render; no se escribe una cifra en las plantillas. Los datos de servidor y memoria solo se incluyen para socket_owner verificado.
3. **Diseño:** templates/menu/theme.json configura identidad, prefijo de control, letras, fuentes, nombres de encabezados, símbolos de ramas, textos ES/EN y textos sin fuente verificada. Las carpetas classic/ y plain/ alojan, por separado, root.txt, category.txt, category-row.txt, command-row.txt, all.txt, all-category-row.txt, information.txt, system.txt, empty.txt, pagination.txt e instruction-row.txt. El fichero owner.txt es solo para la vista clásica autenticada. El tema clásico conserva la estructura visual propia de Matter; el modo plain no contiene adornos.
4. **Interacción:** los botones/fila de categoría solo son texto generado con el id registrado. Las operaciones reales pasan por el router, los permisos y el proveedor. Una plantilla no ejecuta comandos ni aumenta roles.
5. **Formato:** estilo y contenido son independientes de las entradas. La plantilla compone el mensaje WhatsApp como texto UTF-8, no como HTML interactivo: WhatsApp no procesa hojas CSS arbitrarias a través de un mensaje. Se imita la **separación de responsabilidades** de HTML/CSS, no se invoca un navegador.
6. **Interpolación:** cada archivo de presentación usa variables @Name. El sustituidor es de una sola pasada; un nombre de usuario que contenga @Commands no se reinterpreta. @@ representa un @ literal. Los tokens desconocidos se rechazan y el mensaje tiene límite de bytes y caracteres de control.
7. **Edición:** las plantillas están incluidas en el binario mediante include_str! por seguridad/reproducibilidad. Cambiar el estilo requiere compilar el controlador de prueba y validarlo; no se reemplazan archivos en un WhatsApp en marcha por edición remota.

## Variables admitidas

- Identidad: @Bot, @Project, @Owner, @Prefix, @Version, @Date, @User, @Role, @Status, @Socket.
- Conteo real: @Commands (total CommandSpec implemented), @CategoriesActive (categorías con comandos implementados; no incluye all), @CategoryCommands (implementados de esa categoría).
- Categorías: @CategoryRows, @CategoryName, @CategoryId, @CategoryIcon, @Branch, @AllRows, @AllTitle.
- Comandos: @CommandRows, @Usage, @Page, @Pages, @NextPage, @Pagination, @MenuCommand.
- Componentes visuales: @Title, @RootTitle, @CategoriesTitle, @InformationTitle, @SystemTitle, @InstructionsTitle, @Information, @SystemBlock, @OwnerBlock, @Instructions.
- Métricas: @Host, @OS, @Uptime, @ServerRAM, @ControllerRAM, @ProviderRAM.
- Pendientes de fuente verificada: @SubBots, @PremiumBots, @URL, @ProviderRAM. Muestran la traducción configurada de valor no verificado; nunca inventar cero, cifras ajenas ni URL de terceros.
- Las etiquetas localizadas son @LabelUser, @LabelBot, @LabelProject, @LabelOwner, @LabelPrefix, @LabelCommands, @LabelCategoriesActive, @LabelVersion, @LabelDate, @LabelRole, @LabelStatus, @LabelSocket, @LabelSubBots, @LabelPremiumBots, @LabelURL, @LabelHost, @LabelOS, @LabelUptime, @LabelServerRAM, @LabelControllerRAM, @LabelProviderRAM.

Ejemplo de un fragmento de plantilla personalizable:

~~~text
@LabelCommands: @Commands
@LabelCategoriesActive: @CategoriesActive
@CategoryIcon @CategoryName
@Prefix@Usage
~~~

No establecer manualmente Commands ni CategoriesActive en JSON; sus valores proceden del catálogo. CategoryName no procede del nombre de carpeta. El diseño izquierdo de comandos se cambia únicamente en command-row.txt; la fuente de nombre de categoría es fonts.category; la fuente de encabezado fonts.header. Las etiquetas se configuran en es.labels y en.labels.

## Faltantes reales para automatización integral

- Los campos @Bot, @Owner, @Prefix y @URL aún recurren a la configuración del tema, no a metadatos verificables por cada instancia. No confundir alias del proyecto con rango Vault. Implementar un registro confiable y tipado de bot/sockets antes de cambiar esto a autodetección.
- @SubBots y @PremiumBots requieren registro real de instancias, planes, estado de conexión y autorización.
- @ProviderRAM y el uptime de WhatsApp requieren telemetría autenticada del proceso proveedor. @Uptime y @ControllerRAM actuales pertenecen al controlador Rust.
- Actualmente la lista de CommandSpec implementados es catálogo declarado; no garantiza que cada handler esté conectado a una plataforma o que esté habilitado en el chat. Distinguir en el futuro CommandsAvailable de CommandsActive por proveedor sin inventar.
- Los estilos personalizados por usuario/comunidad y hot-reload seguro requieren un registro autorizado, validación de rutas y cache versionada. No aceptar plantillas de texto enviadas por WhatsApp como código/HTML ejecutable.
- Pruebas obligatorias: cargo fmt, cargo check --all-targets, cargo test, cargo clippy --all-targets -- -D warnings, CI Linux/Windows y proveedor, test en teléfono para el socket autorizado. El fallback legado templates/matterssmith-menu.txt es independiente y no es la vista dinámica de producción; migrarlo después sin cortar la sesión.

## Política de seguridad

La salida nunca debe contener secretos, rutas privadas, números/IDs de sesión, tokens ni claves. Los valores no autorizan acciones. La sección @SystemBlock se omite para lectores públicos. No cambiar Vault, propietarios, sesiones o sockets en producción por la edición del diseño.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
