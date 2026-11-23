# Motor de plantillas de menú Matter — contrato oficial

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
