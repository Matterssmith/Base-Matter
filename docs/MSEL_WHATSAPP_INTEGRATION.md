## Test opcional extremo a extremo entre repositorios (offline, sin transporte)

Tras compilar la biblioteca MSEL original según el ejemplo anterior, en `Base-Matter/alpha`:

```bash
cd /home/Sakura/Base-Matter
git fetch origin alpha
git switch alpha
git pull --ff-only origin alpha
MATTER_MSEL_CHECKOUT=/home/Sakura/MSEL cargo test --test msel_bridge original_msel_checkout_e2e -- --ignored --nocapture
```

Este test carga el script real `examples/bots/multiplatform.msel` del repo privado, lanza el proceso de Base-Matter y valida la salida `["pong"]` del motor nativo C++ de MSEL. Por defecto está **ignorado en CI público** por requerir el clone privado; su éxito no se presume. Sólo usa identidades inventadas y no abre ninguna sesión, red o proveedor de WhatsApp. CI Linux de la revisión previa `1cb11f4` aprobó rustfmt/check/tests/Clippy; ver [ejecución](https://github.com/Matterssmith/Base-Matter/actions/runs/44954149724).

# Base-Matter + WhatsApp-Matter → MSEL original

**Estado 2026-11-21:** puente de mensajes añadido a `alpha`, sin despliegue WhatsApp ni prueba extremo a extremo en vivo. El proveedor existente permanece en el mismo socket y sesión; la ruta de bot MSEL no se activa automáticamente.

## Responsabilidad de cada repositorio

- **MSEL (privado, alpha):** lenguaje oficial original, scripts `.msel`, motor C++17 `msel_bot_message_v1`, CLI `python3 -m msel bot-event` que procesa un evento JSON por stdin y devuelve hasta 4 respuestas JSON por stdout. Python es solo marshaling/host; **el código MSEL se interpreta en el motor original**, no en Python, JavaScript o un lenguaje paralelo.
- **WhatsApp-Matter (público, matter):** proveedor nativo Rust y conversión de mensajes al evento neutral de MSEL (`NormalizedTextEvent::as_untrusted_msel_message`). La estructura **no** es prueba de identidad ni autorización y no envía respuestas.
- **Base-Matter (público, alpha):** `src/msel_bridge.rs` define `MselCommandRunner` (proceso local restringido, plazo máximo, JSON acotado, respuesta deshabilitada por defecto); `InboundGateway.ingest_msel` exige `InboundAuthenticator`, `InboundAdmission` y `InboundReplayStore` antes de ejecutar.

## Secuencia autorizada de datos

1. El proveedor entrega el mensaje autenticado; el host construye `InboundEvent { platform: "whatsapp", event_id, text }`. El mensaje normalizado de WhatsApp-Matter no se usa como comprobante de identidad.
2. `InboundAuthenticator` verifica la conexión real, deriva `VerifiedIngress` con `socket_id`, `source_id` (remitente), `group_id` y plataforma. **No aceptar esas identidades desde campos escritos por un usuario.**
3. `InboundAdmission` debe permitir explícitamente la cuenta/sesión, consentimiento legal, estado bot ON para ese grupo/socket, política de comandos, permisos `message.read` y, si corresponde, `message.reply`. Toda negativa es fallo cerrado.
4. `InboundReplayStore` reclama el identificador de mensaje contra replay antes de efectos; repetir el evento devuelve `MselInboundOutcome::Duplicate`.
5. `MselCommandRunner` crea un subproceso local sin entorno secreto y envía exclusivamente `{provider,chat,sender,message_id,text}` por stdin. `MSEL` ejecuta el script confiable con la biblioteca nativa original. Los mensajes de control `=/>` se rechazan en esta ruta.
6. `MselInboundOutcome::Replies(Vec<String>)` retorna texto sin enviarlo. **La aplicación host todavía necesita conectar un transporte de respuesta autorizado** con deduplicación de envíos e idempotencia. Jamás enviar automáticamente texto devuelto sin verificar permiso/estado del socket.

La ruta existente `InboundGateway.ingest` para controles continúa separada; no ejecutar las dos rutas para un mismo mensaje porque comparten replay. `MselCommandRunner::new` no concede `message.reply`; únicamente la aplicación de confianza, tras validar la política, puede invocar `allow_message_reply()`.

## Prueba offline sin sesiones WhatsApp

En Linux Debian/Ubuntu, desde un clon **autorizado** del repositorio privado MSEL:

```bash
cd /home/Sakura/MSEL
git fetch origin alpha
git switch alpha
git pull --ff-only origin alpha
mkdir -p build
g++ -std=c++17 -O2 -DMSEL_BUILD_EMBED -fPIC -shared native/cpp/msel_native.cpp -o build/libmsel_embed.so
printf '%s' '{"provider":"whatsapp","chat":"test@g.us","sender":"test@s.whatsapp.net","message_id":"test-001","text":"#ping"}' | python3 -m msel bot-event examples/bots/multiplatform.msel --native-lib build/libmsel_embed.so
```

Salida prevista: `["pong"]`. No conecta WhatsApp. Para tests del host Rust (en otro checkout):

```bash
cd /home/Sakura/Base-Matter
git fetch origin alpha
git switch alpha
git pull --ff-only origin alpha
cargo test --test msel_bridge
cargo test --lib msel_bridge
```

Y en el checkout WhatsApp-Matter:

```bash
cargo test -p matter-whatsapp-contracts msel_event_contract_is_data_only_and_exact
```

En el flujo de CI se validará también el CLI real con `MSEL_NATIVE_TEST_LIB` bajo Linux. Las suites creadas no equivalen a éxito verificado; consultar los SHAs de GitHub Actions. No utilizar la sesión conectada a producción ni crear otro proveedor con su SQLite.

## Límites / próximos pasos

- Integrar una implementación de `InboundAuthenticator` y `InboundAdmission` con el proveedor live y Vault antes de usar `ingest_msel` en el proceso WhatsApp; instalar reglas de bot activo/grupo y autorización del envío.
- Probar el resultado real en un socket WhatsApp **de prueba** autorizado, con telemetría sin contenidos privados; asegurarse de que el bot no conteste si está OFF.
- El puente por proceso no es sandbox ni reemplaza un runtime Rust oficial. Sólo scripts MSEL confiables; usar UID restringido, políticas de filesystem/red y timeouts. El motor nativo y código de lenguaje siguen privados.
- Mantener Base-Matter y Bot-Matter independientes del compilador privado. Bot-Matter permanece bloqueado hasta superar criterios de preparación. No afirmar que las 32 tecnologías del lenguaje están completas.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
