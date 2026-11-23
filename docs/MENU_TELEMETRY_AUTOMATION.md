## 2026-11-23 — Diseño anterior sustituido por motor de plantillas

La disposición visual anterior de información de bot y sistema fue retirada del código Rust por solicitud del propietario. Su contrato operativo/privacidad se conserva, pero los campos se representan exclusivamente mediante tokens de templates/menu/*.txt y configuración templates/menu/theme.json. Los valores se obtienen en src/menu/theme.rs; src/menu/telemetry.rs no debe contener diseño. Ver docs/MENU_TEMPLATE_ENGINE.md como fuente de verdad para el renderizado. Datos que carecen de fuentes siguen sin verificación. No se ha actualizado el proveedor real ni sus sesiones.

# Menú Matter — información dinámica del bot y del host

Estado: implementación parcial en `Base-Matter/alpha`; despliegue WhatsApp pendiente de pruebas en dispositivo. Fecha: 2026-11-23. Idea de diseño suministrada por el propietario a partir de un ejemplo externo «Aura Reed Bot». Solo se aprovechan los **campos de información**, no su identidad ni sus cifras.

## Contrato del menú

Mantener el marco y la tipografía propios de Matterssmith en `=/>menu`, sin mezclarlo con identidad de otros bots. Se incluye un bloque «Información Matter» y uno «Sistema · Host» (el segundo solo para el propietario del socket autenticado). El registro de comandos sigue siendo derivado de descriptores reales, no de un número hardcodeado.

| Campo | Fuente implementada | Condición de publicación |
|---|---|---|
| Usuario, rango, estado, socket | `MenuViewer` recibido tras puente autenticado | Usuario público con identificadores restringidos; nunca inferir rango global |
| Bot / instancia principal | Configuración actual «Principal» | Debe ser sustituida por metadatos verificables por socket cuando haya múltiples instancias |
| Versión Base | `CARGO_PKG_VERSION` compilada en el binario | Versión de Base; no afirmar que es la versión del proveedor ni de MSEL |
| Propietario del proyecto | Etiqueta pública «Lou-Nyx» asociada a Matterssmith | No equivale a rol Vault; el rango real viene de `MenuViewer` |
| Prefijo | `=/>` de control | En el futuro obtener del router si cada bot tiene prefijo individual `#` |
| Fecha | `date +%d/%m/%Y` del host | Valor en cada llamada; depende de la zona horaria configurada en el servidor |
| Número de comandos | `menu::command_count()` | Cuenta del catálogo `implemented: true`; NO número de handlers probados ni activos |
| Servidor | `/proc/sys/kernel/hostname` en Linux, validado | Solo propietario autenticado del socket; nunca IP, rutas, IDs, tokens |
| Sistema operativo | Constantes de compilación `std::env::consts::{OS,ARCH}` | Solo propietario; refleja plataforma de ejecución del controlador |
| Uptime del proceso de control | `/proc/self/stat` + `/proc/uptime` y `getconf CLK_TCK` | Solo propietario; no confundir con uptime del bot proveedor |
| RAM del servidor | `/proc/meminfo` MemTotal−MemAvailable | Solo propietario; no confundir memoria libre con RSS ni uso de contenedores |
| RAM del controlador | `/proc/self/status` VmRSS | Solo propietario; NO total de RAM de todos los procesos del bot |
| Sub-bots / premium-bots | No hay contador fiable enlazado a registry actual | Mostrar «sin registro verificado», nunca inventar 130/3 |
| URL | Marcador «sin configurar» | No copiar el dominio del bot externo; pendiente de configuración por instancia |
| Uptime y RAM del proveedor WhatsApp | Faltan métricas del proceso proveedor | Mostrar pendiente, no falsear como métricas del controlador |

## Requisitos para que sea 100% automático

1. **Registro unificado firmado:** mantener `instance_id`, `provider`, `plan`/premium, `lifecycle`, `started_at` y enlace público configurado, sin almacenar nombres ni tokens privados en el menú.
2. **Telemetría por proceso:** el proveedor WhatsApp-Rust reportará RSS real y monotonic uptime autenticados hacia el controlador; Telegram, Discord y futuros proveedores usarán el mismo contrato. No transmitir PIDs, IPs o secretos a chats públicos.
3. **Conteos seguros:** sub-bots y premium-bots se obtendrán de instancias activas y autorizadas, no de sesiones SQLite externas ni de directorios locales; las caídas y restauraciones deben descontarse automáticamente.
4. **Metadatos por socket:** versión del bot, nombre de instancia, prefijo de comandos y URL deben provenir de configuración validada; el owner visible es un alias configurado y no prueba rango.
5. **Formato y pruebas:** mantener menú clásico y `style none`, validar UTF-8, longitud WhatsApp (3800 bytes), idiomas ES/EN, ocultar categorías vacías y mantener la ayuda separada del menú.
6. **Privacidad:** host y métricas de carga solo propietario autenticado, sin Vault owner implícito; lectores públicos reciben solo datos generales. Nunca dar acceso a comandos por contener información en la presentación.
7. **Frecuencia:** memoria y uso se capturan al consultar menú, no requieren cron ni consulta a API externa. Campos futuros solo se consideran automáticos cuando se integren al registro y estén probados.
8. **Despliegue:** la rama `alpha` no es producción; ejecutar Rustfmt, `cargo test --test menu_catalog` y CI del SHA final, luego actualizar exclusivamente los binarios de prueba vinculados al socket autorizado. No tocar servicios, PM2, Vault ni sesiones del servidor compartido `idkbot` sin autorización y respaldo.

## Situación del desarrollo

- Añadido `src/menu/telemetry.rs` para capturas locales de fecha/host/OS/RAM/uptime del proceso controlador, con fallbacks «No disponible».
- `src/menu.rs` integra los bloques de información en el menú raíz clásico y plano.
- `tests/menu_catalog.rs` verifica etiquetas y que información privada no aparezca en la vista pública.
- Pendiente real: validar compilación Rust y CI Linux/Windows; conectar fuentes fiables para sub-bots, premium, URL y proceso proveedor; desplegar y probar en WhatsApp.
- No agregar `Aura Reed`, `Jeriel B.`, `NODE-AKIRAX`, el dominio externo ni cifras ajenas.
- Mantener esta especificación, `docs/DEVELOPMENT_STATUS.md`, `docs/WORK_CONTINUITY.md` y ambos README sincronizados cuando avance la implementación.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
