### Ejecución web integrada al bot (sin GitHub Pages)

El proveedor `whatsapp-rust` arranca el servidor estático embebido junto a la sesión existente, sin crear otro socket de WhatsApp. Alpha escucha en `127.0.0.1:39831` y sirve `index.html` y `app.js` desde su propio binario. No habilita POST ni captura datos. El servicio local continúa activo mientras el proveedor está en ejecución; una falla HTTP no detiene WhatsApp.

Para poder usar `=/>register` como botón público es necesario configurar primero un dominio HTTPS real y un reverse proxy que dirija `/register/` a dicho puerto, y luego exportar `MATTER_REGISTER_PUBLIC_URL` con la URL verificada. Sin dominio no existe enlace público seguro; no reutilizar el antiguo URL de GitHub Pages. Instrucciones y pruebas: [MATTER_REGISTER_BOT_HOST.md](MATTER_REGISTER_BOT_HOST.md).

**Estado:** código en rama de desarrollo; todavía no instalado en `instabox-se`, URL pública HTTPS no confirmada y registro civil deliberadamente deshabilitado. Se mantiene el acceso invitado, no se obtiene rango owner mediante el formulario.

# Matter Register — Pantalla interactiva y transporte multiplataforma

**Estado:** vista previa de interfaz en rama de funcionalidad. Registro real deshabilitado por defecto. NO desplegar como KYC operativo.

## Ubicación y propósito

- Arquitectura estricta **JavaScript-first**: `web/matter-register/index.html` es exclusivamente un cargador (`<script src="./app.js" defer>`), sin DOM visual, CSS, formularios ni código inline. `web/matter-register/app.js` genera por JavaScript el DOM completo, hoja CSS insertada por JS, componentes, temas, animaciones, sonido, flujo de registro, pantalla completa, respuesta de errores e interfaz con el backend. Esta es una regla permanente de Matter Register; no volver a incrustar visuales en HTML.
- UI estática: `web/matter-register/app.js`. Vista local con cinco skins inspiradas por las referencias aportadas por el propietario: Ideas verde, Kids dorado, SFW cian, Testers naranja y Kids Channel neón.
- Interacciones reales de navegador: pasos por audiencia y país, vista de confirmación, fullscreen mediante gesto del usuario, sonido original opcional WebAudio y fallback para navegadores que restringen fullscreen. No usa rastreadores ni almacenamiento de identidad.
- Transporte JS/TS Baileys alternativo: `providers/baileys-compat/register-card.mjs`. No se carga en el runtime Rust; mantiene a Rust como proveedor principal y deja Baileys como adaptador opt-in.
- En WhatsApp el botón `cta_url` puede abrir el portal web; `open_webview` es un modo experimental opt-in (`preferInAppWebview: true`) cuando el SDK y el cliente realmente lo permiten; `quick_reply` devuelve el comando público de invitado si existe en el dispatcher autorizado. Ni `cta_url` ni `open_webview` significan que WhatsApp ejecute libremente HTML, CSS, audio y JS dentro del mensaje. Verificar comportamiento real en dispositivo Android, iPhone y Web.
- Los ejemplos previos TS18/TS19 y de RichResponse siguen siendo solo ensayos de transporte. No falsificar `botForwardedMessage`, `0@bot` ni procedencia Meta AI.
- El contrato de transporte opcional Go está en `providers/whatsapp-go-compat/register_card.go` y su prueba sin SDK de red en `register_card_test.go`; TypeScript dispone de `providers/baileys-compat/register-card.d.ts`. Ambos construyen tarjetas, pero NO se conectan automáticamente al socket WhatsApp. Para Go, JS/TS o futuros proveedores se define el mismo evento abstracto `registration.open` con URL HTTPS aprobada. Los SDK deben mapearlo a su mecanismo de botones; nunca admitir un enlace definido por el remitente.

## Contrato seguro

1. `GET /api/register/capabilities` sin PII. La UI exige simultáneamente `ready=true`, `verification_provider_ready=true`, `authenticated_session=true` y `legal_documents_published=true` para permitir intentar iniciar la sesión. Esto es una pista UI, **no** una comprobación de seguridad.
2. `POST /api/register/sessions` solo después de que un backend privado valide autenticación de origen (nonce de un solo uso, vinculación plataforma + ID firmado, TTL, audience, origen permitido), token CSRF, rate limiting, consentimientos legales versionados y aprobación de país/edad/capacidad. El backend debe responder con una URL HTTPS de redirección de corta duración, nunca con datos civiles. El frontend por sí solo no comprueba nada.
3. Invitado: no crea sesión civil, no envía datos y no recibe rangos. Los comandos públicos de Base son el único acceso en el estado actual.
4. Adulto: paso de documento oficial válido por país, anverso y reverso cuando corresponda, validación de documento por emisor/proveedor autorizado, autenticación de presencia real y coincidencia cuando sea necesaria y proporcional. La biometría no debe almacenarse ni entregarse a Base. Incorporar opt-out y alternativas cuando las reglas lo exijan.
5. Menor: no pedir biometría. Reglas por jurisdicción y servicio; en México puede utilizarse CURP como dato o comprobante sujeto a fuente de validación oficial, pero una CURP introducida o fotografiada no acredita identidad por sí sola. Validar autorización verificable de representante legal cuando proceda.
6. Vault privado es autoridad sobre identidad, decisión legal, nivel de verificación y atributos derivados mínimos. Ninguna asignación global de roles durante registration. Las aserciones para Base/Bot deben tener firma, alcance, audiencia, expiración y protección antirreplay.
7. **No** enviar fotos, CURP, vídeos, nombres de menores, teléfonos, plantillas biométricas, secretos ni referencias reversibles a GitHub. Las copias remotas de Vault admiten únicamente ciphertext autenticado. Controlar también backups, logs, tasas de retención, eliminación y revocación.
8. No incluir tokens de registro en URL de WhatsApp: una URL pública no tiene privilegios. La sesión se inicia después de autenticación y se ata a una identidad real. CORS, CSP, cookies HttpOnly/Secure/SameSite y CSRF del gateway privado son obligatorios.

## Estado de implementación

✅ Pantalla web animada, responsive, temática, fullscreen por gesto, sonido opcional y flujos visuales.
✅ Estructura opcional de tarjeta de WhatsApp Baileys con botón URL y fallback de texto, sin cambiar proveedor Rust ni sockets existentes.
⛔ Gateway privado, identidad vinculada, documentos legales finales, verificación oficial, prueba de vida, autorización de tutor y persistencia final en Vault pendientes. La pantalla bloquea estas capacidades.
⛔ URL pública HTTPS y pruebas E2E en WhatsApp/Android/iOS pendientes: **no** anunciar el registro como operativo.

## Pruebas

```bash
node --check web/matter-register/app.js
python3 -m unittest discover -s test -p 'test_register_web.py'
node --test providers/baileys-compat/register-card.test.mjs
GO111MODULE=off go test ./providers/whatsapp-go-compat
```

La página puede abrirse localmente para probar animaciones, pero no debe procesar datos personales hasta contar con backend habilitado y auditoría.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
