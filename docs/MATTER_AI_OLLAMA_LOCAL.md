## Corrección de la creación manual de Sakura

Para obtener el usuario Linux exacto `Sakura` (con mayúscula) y permisos de administrador:

```bash
getent passwd Sakura || sudo useradd -m -U -s /bin/bash Sakura
sudo passwd Sakura
sudo usermod -aG sudo Sakura
id Sakura
sudo loginctl enable-linger Sakura
sudo -iu Sakura
```

Si ya existe, `useradd` no se ejecuta. `sudo passwd Sakura` pide contraseña sin mostrarla. Estos pasos deben ejecutarse desde una cuenta que ya tenga permisos sudo. No permite acceso remoto sin autenticación. Como alternativa de un solo comando desde el clon de desarrollo: `sudo bash tools/setup-sakura.sh --admin --password-prompt --linger`. [Guía de recuperación](MATTER_SAKURA_BRANCH_RECOVERY.md).

# Matter-AI local: conversación y ciclos de trabajo sin facturación por mensaje

**Fecha:** 2026-11-20. **Estado:** backend local Ollama implementado en `workstreams/02-base` y pruebas offline incluidas; **no instalado ni probado con un modelo real en la laptop del usuario**.

## Qué es posible y qué no

ChatGPT Plus es un servicio alojado. **No convierte GPT-6 en una aplicación instalable** ni proporciona solicitudes ilimitadas de API desde Ubuntu. La API comercial se factura aparte. Algunos terceros admiten iniciar sesión con ChatGPT y compartir parte de los límites del plan, pero eso no es ilimitado ni ofrece acceso a todas las capacidades de esta conversación.

Alternativa: servidor de inferencia **Ollama**, ejecutado localmente en Ubuntu con modelos descargados. El servidor responde en `http://127.0.0.1:11434/api/chat` y admite llamadas a herramientas. Esto elimina el coste **por solicitud de OpenAI** mientras se usa ese modelo local, pero hay restricciones físicas: RAM, CPU, disco, energía, temperatura y calidad inferior frente a un modelo de gran tamaño. No es el mismo ChatGPT, no comparte memoria de usuario, conectores ni herramientas de la app; Matter-AI carga la documentación persistente del proyecto y sus repositorios.

## Crear Sakura manualmente

Desde la cuenta administradora actual:

```bash
sudo adduser Sakura
```

Ubuntu solicitará una contraseña para Sakura sin mostrar lo que escribes. Si prefieres **sin contraseña establecida**, ejecuta en lugar de lo anterior:

```bash
sudo adduser --disabled-password --gecos "" Sakura
```

La opción bloquea inicio de sesión mediante contraseña: **no es una contraseña vacía**. Para entrar desde la cuenta administradora puedes ejecutar:

```bash
sudo -iu Sakura
whoami
```

Para iniciar SSH directamente como Sakura, habilita y usa una **clave pública SSH** (no se autoriza acceso remoto sin autenticación). No conceder sudo sin contraseña ni modificar `sshd_config` para permitir contraseñas vacías.

## Instalar Ollama

Instálalo siguiendo las instrucciones oficiales: https://docs.ollama.com/linux . La descarga y el instalador remoto deben obtenerse de fuentes oficiales verificadas; evita ejecutar scripts de procedencia desconocida con sudo. Si el instalador de Ollama registró el servicio systemd, comprueba:

```bash
ollama -v
systemctl status ollama
```

Descarga un modelo pequeño para equipos limitados:

```bash
ollama pull qwen3:0.6b
```

El archivo del modelo ocupa unos **523 MB**, sin contar memoria adicional para inferencia/contexto. Alternativa si el equipo tiene holgura de RAM:

```bash
ollama pull qwen3:1.7b
```

Ocupa unos **1.4 GB** descargados, pero la memoria real en ejecución es mayor. En laptops de aproximadamente 4 GB de RAM, 0.6B es un inicio más realista; 1.7B puede ser lento y no hay garantía de que quepa sin swapping. Ambos modelos son muy inferiores para ingeniería compleja, auditando seguridad o escribiendo Rust sin asistencia.

## Instalar Matter-AI para Sakura

Dentro de la sesión `Sakura`:

```bash
git clone --branch workstreams/02-base https://github.com/Matterssmith/Base-Matter.git ~/Base-Matter
cd ~/Base-Matter
bash tools/install-matter-ai.sh
~/.local/bin/matter-ai backend local
~/.local/bin/matter-ai local-model qwen3:0.6b
~/.local/bin/matter-ai
```

Desde este punto, hablarás con el **modelo local**, no GPT-6. Para cambiar al proveedor comercial cuando lo decidas:

```bash
~/.local/bin/matter-ai backend openai
~/.local/bin/matter-ai setup
```

La API requiere clave privada y facturación independiente. Cambiar `backend` no toca WhatsApp, Vault ni las ramas de producción.

## Conversaciones persistentes en el terminal

Con el backend `local`, la conversación se recupera parcialmente al volver a abrir `matter-ai`. Guarda hasta 12 intercambios recientes en `~/.local/state/matter-ai/chat-history.json` con permisos `0600`. El historial y el modelo se quedan en Ubuntu; no comparten la memoria ni las conexiones de esta app de ChatGPT. La herramienta intenta descartar mensajes que contengan patrones de credenciales, pero **no es un gestor de secretos**; nunca pegues claves o sesiones en la conversación.

Para planificación de largo plazo también se carga `MATTER_AI_HANDOFF.md` y se conserva el último resultado por workstream. Es una memoria operativa local, no memoria ilimitada: las conversaciones extensas se resumen o quedan fuera de la ventana de contexto.

## Automatizar el trabajo 24/7 sin cuota OpenAI

Descarga primero los repositorios de trabajo necesarios:

```bash
~/.local/bin/matter-ai sync matter-rust
~/.local/bin/matter-ai sync msel
~/.local/bin/matter-ai sync social
```

Los repos privados requieren autorización GitHub para Sakura. Verifica el servidor y las herramientas antes de activarlo:

```bash
~/.local/bin/matter-autopilot doctor
~/.local/bin/matter-autopilot preview
```

Ejemplo de configuración de trabajo continuo **sin cuota de mensajes de OpenAI**, con un máximo operativo de 288 rondas al día y una separación mínima de 300 segundos. No significa que la máquina pueda procesar cada ronda en cinco minutos ni que los cambios sean siempre correctos:

```bash
~/.local/bin/matter-autopilot configure --interval 300 --daily 288 --calls 5 --no-push
~/.local/bin/matter-autopilot once
~/.local/bin/matter-autopilot start
```

Cada ronda sigue la orden literal **«continúa el proyecto matter»** y avanza un proyecto descargado, con memoria local de la última ronda. En modo local se puede configurar hasta 1,440 rondas/día, pero un modelo pequeño en CPU rara vez alcanzará esa cifra. Los límites de ciclos evitan errores de repetición, sobrecarga y calentamiento; no son cuotas comerciales.

El servicio `systemd --user` puede reiniciar automáticamente si falla. Para ejecutarse después de reiniciar la laptop sin abrir sesión, un administrador debe habilitar:

```bash
sudo loginctl enable-linger Sakura
```

**La laptop debe permanecer encendida y sin suspensión.** No hay forma de trabajar mientras está apagada, sin capacidad eléctrica o con Ollama detenido.

## Control, privacidad, calidad y respaldo

```bash
~/.local/bin/matter-autopilot status
journalctl --user -u matter-autopilot.service -n 50 --no-pager
~/.local/bin/matter-autopilot stop
```

- **Backend local:** sólo `127.0.0.1`, sin solicitar API key y sin URL de Ollama remota. Evita exponer el puerto 11434 a internet.
- **Seguridad:** archivos de código/documentación permitidos en `workstreams/*`; no scripts, workflows, secretos, datos Vault, sesiones ni inicio de Bot-Matter. Un modelo pequeño puede interpretar mal un archivo: los commits automáticos no se dan por probados.
- **Persistencia:** commits locales y registro `~/.local/state/matter-ai/autopilot-state.json`. Auto-push sólo con autorización explícita adicional, únicamente a la rama del workstream. Alpha/beta/stable únicamente mediante PR revisado.
- **Memoria:** documentos `MATTER_AI_HANDOFF.md`, README y último resumen local; no la memoria del ChatGPT de tu teléfono.
- **Sin límites comerciales de mensajes:** sí en modo local, **no** sin límites de recursos, precisión o seguridad.

La integración local usa la API oficial Ollama `/api/chat` con soporte de herramientas y funciona con la biblioteca estándar de Python. No necesita Node ni Codex.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
