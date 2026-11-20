# Matter-AI — ChatGPT en consola Ubuntu Server (sin Codex)

**Estado:** agente CLI independiente implementado en `workstreams/02-base`, sin despliegue en la laptop, sin claves en GitHub y sin pruebas de API real. Usa OpenAI **Responses API** mediante la biblioteca estándar Python.

**Uso previsto:** instalar en tu laptop Ubuntu con teclado una sola vez; después hablar desde terminal con `matter-ai` y ordenar trabajos sobre cada repositorio Matter. Se pueden ejecutar órdenes de lectura y editar archivos completos **en clones locales**, con confirmación del operador. No controla remotamente un servidor ni entra a la cuenta ChatGPT del navegador.

## Preparación con el mínimo de escritura manual

En Ubuntu necesitas Python 3 y Git. Con internet puedes introducir estas instrucciones **una sola vez**:

```bash
git clone -b workstreams/02-base https://github.com/Matterssmith/Base-Matter.git
cd Base-Matter
bash tools/install-matter-ai.sh
~/.local/bin/matter-ai setup
```

El instalador no utiliza `sudo`, no ejecuta `curl | bash` y no solicita contraseñas del sistema. Si `~/.local/bin` está en PATH, basta `matter-ai`; si no, usar `~/.local/bin/matter-ai`.

### Evitar copiar la API key con el teclado del servidor

La API key es un secreto largo: créala desde un navegador seguro en [platform.openai.com/api-keys](https://platform.openai.com/api-keys), y transfíerela una vez mediante una sesión SSH confiable desde un dispositivo en el que **sí** puedas pegar. En la sesión SSH ejecuta `~/.local/bin/matter-ai setup` y pega la clave cuando aparezca el prompt oculto. **No escribas la clave en el historial shell, GitHub, un chat, URL, issue ni archivo público**. Si no es posible transferirla, no existe forma segura de usar OpenAI API desde la terminal sin introducir o aprovisionar una credencial.

ChatGPT Plus/Pro y API tienen facturación separada; configura controles de gasto en tu cuenta API. Modelo inicial: `gpt-6.2-sol`; para otro modelo usar `matter-ai model gpt-6-luna`. Disponibilidad según cuenta y saldo.

## Comandos diarios

```bash
matter-ai plan
matter-ai projects
matter-ai sync matter-rust
matter-ai sync msel
matter-ai sync social
matter-ai sync vault
matter-ai
```

El comando `matter-ai plan` imprime el traspaso persistido del ecosistema, que el agente también carga al consultar OpenAI; así basta decir "continúa con Matter-Rust" sin copiar el historial del teléfono. El traspaso es una síntesis de trabajo, no toda la conversación.

En el prompt interactivo escribe, por ejemplo:
`Revisa AGENTS y README de matter-rust, lista los pendientes y corrige el error de compilación.`

La IA puede consultar workstreams, listar y leer archivos no secretos (incluidos README extensos por fragmentos), editar archivos completos con confirmación humana y correr ciertos comandos Git/Rust/Python. Los clones quedan bajo `~/Matter/workspaces/<workstream>`, uno por tarea, en la rama Git correspondiente. `matter-ai sync all` es opcional: puede consumir varios GB y necesita credenciales GitHub para repos privados.

En el chat de terminal, `/auto on` permite aprobar **una vez** la edición de archivos de los workspaces locales; no concede push, borrado de ramas ni privilegios de servidor. `/auto off` lo revoca. Antes de ejecutar tests seguirá solicitando aprobación.

**Limitaciones deliberadas:**
- No auto-push, auto-merge, reseteos ni borrados masivos. Pide revisión y push manual para cambios GitHub.
- No acceder a datos cifrados de Vault, archivos de sesión, API keys, credenciales privadas, ni alterar sockets existentes.
- No ejecutar comandos shell arbitrarios ni servicios del sistema como root.
- `Bot-Matter` permanece bloqueado hasta pasar los gates.
- Una tarea puede requerir confirmaciones interactivas para archivos, tests y commits; no es ejecución desatendida.
- No conserva conversaciones de este ChatGPT ni su contexto privado automáticamente: carga `~/.local/share/matter-ai/MATTER_AI_HANDOFF.md` y debe verificar documentación vigente en los repositorios.

## Verificación

```bash
python3 -m unittest discover -s tests -p 'test_matter_ai_cli.py' -v
python3 tools/matter_ai.py projects
python3 -m py_compile tools/matter_ai.py
```

Los tests de CLI son offline; no validan conexión real OpenAI, clonación privada o ejecución total de todos los workstreams.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
