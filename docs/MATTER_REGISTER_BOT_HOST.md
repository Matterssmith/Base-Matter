# Matter Register — servidor incluido en el bot (sin GitHub Pages)

**Estado:** implementado en código de la rama `feat/matter-register-web`; aún no instalado ni verificado en el servidor temporal `instabox-se`. No se deben pedir documentos hasta desplegar el gateway privado.

## Comportamiento al arrancar

- El binario `whatsapp-matter-rust-provider` llama a `register_web::start_with_controller_port(...)` al iniciar el runtime, después del control de instancia y **antes** de conectar la sesión existente. No necesita otro proceso, QR ni emparejamiento.
- Escucha en `127.0.0.1:(MATTER_WA_BRIDGE_ADDR.port+10)`. Para alpha con controlador en `127.0.0.1:39821`, la página responde en `http://127.0.0.1:39831/`. Beta y stable obtienen puertos separados. **Solo se accede desde el propio servidor o mediante un reverse proxy autorizado.**
- `index.html` solo carga `app.js`. Toda estructura, CSS, animación, audio y flujo se genera en JavaScript. Los dos archivos se incrustan **en compilación** en el binario Rust con `include_str!`, por lo que el runtime no necesita leer GitHub ni archivos del repositorio público.
- El servidor solo admite GET/HEAD para `/`, `/index.html`, `/app.js`, `/api/register/capabilities` y `/healthz`. El endpoint de capacidades devuelve **todo false**. POST de alta responde 405. No puede recopilar documento, rostro, CURP, sesiones o consentimiento; **no hay registro real**.
- El socket HTTP se liga exclusivamente a loopback; no existe conexión con Vault ni llaves en este proceso. No se abre ningún puerto externo desde Rust. Si falla el bind del puerto web, se registra un error no sensible y WhatsApp sigue; nunca se tocan SQLite, sesiones, roles ni Vault.
- Las opciones `--check`, `--help` y verificaciones offline **no** arrancan servidor. `--serve-register-preview` ejecuta solo el servidor estático en `127.0.0.1:39831` sin abrir WhatsApp, para pruebas aisladas cuando el puerto está libre. El subproceso web desaparece cuando finaliza el proveedor.

## URL pública real mediante el servidor

Cuando exista un dominio apuntando al servidor y certificado TLS válido, añadir **solo este bloque** dentro de un virtual host HTTPS ya configurado (ejemplo Nginx):

```nginx
location = /register { return 308 /register/; }
location /register/ {
    proxy_pass http://127.0.0.1:39831/;
    proxy_http_version 1.1;
    proxy_set_header Host $host;
    proxy_set_header X-Forwarded-Proto https;
    proxy_set_header X-Forwarded-For $remote_addr;
    proxy_connect_timeout 3s;
    proxy_read_timeout 10s;
    client_max_body_size 1k;
}
```

Mantener TLS público en el reverse proxy. **Nunca** publicar directamente `39831` en Internet ni proxyar `/run/matter-vault/vault.sock`. Se prohíbe crear una URL pública ficticia o suponer que el nombre interno `instabox-se` es accesible desde un teléfono.

Definir `MATTER_REGISTER_PUBLIC_URL=https://DOMINIO-REAL/register/` como variable de entorno **exportada** del proceso del proveedor después de desplegar el reverse proxy y comprobar el certificado. El valor debe pasar validación estricta HTTPS: nombre DNS y ruta sin queries, fragmentos ni credenciales; usar barra final. El proveedor usa esta URL en el botón `=/>register` solo desde la ruta autenticada por el controlador; no implica ingreso civil.

## Pruebas seguras en instabox-se

Una vez **compilado e instalado el binario verificado** desde GitHub Actions, arrancar el único proveedor existente por su lanzador normal. No generar códigos, ni iniciar otro proveedor sobre la misma sesión. Antes de exponer URL pública:

```bash
curl -fsS --max-time 3 http://127.0.0.1:39831/healthz
curl -fsS --max-time 3 http://127.0.0.1:39831/api/register/capabilities
curl -fsSI --max-time 3 http://127.0.0.1:39831/app.js
```

La primera respuesta esperada es `matter-register-preview-ok`. La segunda indica `ready:false`. La tercera debe responder `200 OK`. Para validar sin ningún proceso WhatsApp, ejecutar **el nuevo binario** con `--serve-register-preview` cuando no esté ya ocupando el puerto; detener esta prueba antes de iniciar el bot.

Cuando el nombre público y certificado estén disponibles, probar `https://DOMINIO-REAL/register/`, abrir desde el navegador del teléfono y después enviar `=/>register` al bot. WhatsApp puede mostrar NativeFlow o solo texto de respaldo; un envío aceptado **no demuestra** que un botón haya renderizado. Para garantizar que no se mezcla la sesión, conservar el lanzamiento `--resume-only` y los grants separados de alpha.

## Estado pendiente real

- Dominio público real, certificado HTTPS, reverse proxy configurado y comprobación desde móvil.
- Actualización segura del binario del proveedor en el host, instalación desde SHA CI verde, reinicio controlado único de la sesión existente; no se ha hecho desde este chat.
- Gateway privado para registro/consentimiento, comprobación documental y prueba de vida autorizada, protección infantil, consentimiento de tutor, decisiones firmadas y persistencia cifrada Vault.
- Hasta entonces, **solo vista previa pública**, no formulario de captura biométrica.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
