## 2026-11-21 — Instalación con arranque automático solicitado por el propietario

**Nueva política:** el instalador bajo Sakura habilita Matter-Autopilot por defecto, usando Ollama local y `docs/MATTER_AUTOPILOT_INSTRUCTIONS.md` como archivo de instrucciones. Este apartado sustituye cualquier instrucción antigua que diga que el instalador nunca activa el servicio. Por defecto: `matter-rust`, cada 900 s, 96 ciclos/día, hasta cinco herramientas/ciclo, sin auto-push ni trabajo en Bot-Matter o ramas protegidas. No se requiere API OpenAI.

**Una línea bajo Sakura**, si el clon de instalación todavía no existe o ya está en la rama correcta:

```bash
if [ -d "$HOME/Matter-AI-install/.git" ]; then git -C "$HOME/Matter-AI-install" pull --ff-only origin workstreams/02-base; else git clone --depth 1 --branch workstreams/02-base https://github.com/Matterssmith/Base-Matter.git "$HOME/Matter-AI-install"; fi && bash "$HOME/Matter-AI-install/tools/install-matter-ai.sh" && "$HOME/.local/bin/matter-ai" backend local && "$HOME/.local/bin/matter-autopilot" activate && "$HOME/.local/bin/matter-autopilot" status
```

`activate` es el paso explícito no interactivo que confirma que el servidor emplea el código actualizado y habilita el arranque. Los errores de systemd o la falta del backend se diagnostican con `journalctl --user -u matter-autopilot.service -n 50 --no-pager` y `matter-autopilot doctor`. Si el usuario hizo `stop`, futuras reinstalaciones preservan esa decisión salvo que invoque de nuevo `activate`. Requiere `loginctl enable-linger Sakura` para inicio sin login, laptop encendida y servidor de Ollama local con `qwen3:0.6b`. El modelo pequeño no garantiza calidad equivalente a GPT-6.

## Requisito previo: usuario administrador Sakura realmente creado

Antes de usar `systemctl --user`, verifica `id Sakura`. Si ves `failed to look up user Sakura` la cuenta no existe; **no ejecutes** todavía `loginctl enable-linger Sakura` ni `sudo -iu Sakura`.

Desde el clon `Base-Matter`, un administrador puede ejecutar `sudo bash tools/setup-sakura.sh --admin --password-prompt --linger`: crea o conserva el usuario, configura contraseña, agrega el grupo sudo sin borrar grupos y habilita linger después de verificar la cuenta. Para hacerlo manualmente sigue [la guía de recuperación](MATTER_SAKURA_BRANCH_RECOVERY.md). No se concede `NOPASSWD`, y la IA autónoma continúa con `NoNewPrivileges=yes` en el servicio de usuario.

# Matter-Autopilot — ejecutar siempre «continúa el proyecto matter»

## Alternativa sin factura de OpenAI: modelo local Ollama

**Actualización 2026-11-20:** Matter-AI ahora ofrece `matter-ai backend local`, con el backend `tools/matter_local.py` y modelo Ollama instalado en tu Ubuntu. **No necesitas clave OpenAI API en ese modo.** Los pasos de configuración API de las secciones siguientes son solamente para `matter-ai backend openai`. No es el mismo GPT-6 del servicio ChatGPT, ni tiene su memoria o herramientas automáticamente. Si la laptop tiene RAM limitada, prueba primero `ollama pull qwen3:0.6b`. La guía principal para trabajar sin coste por mensaje está en [Matter-AI con Ollama local](MATTER_AI_OLLAMA_LOCAL.md). El modo continuo, guardado por rama y bloqueos de Vault/Bot siguen aplicando.

**Implementado como prototipo de desarrollo:** `tools/matter_autopilot.py`, `tools/matter-autopilot.service`, `tools/install-matter-ai.sh` y `tests/test_matter_autopilot.py` en `workstreams/02-base`. **No instalado en la laptop.** Necesita OpenAI API y autorización explícita. No usa Codex.

La frase ejecutada en cada ronda es exactamente:

```text
continúa el proyecto matter
```

El supervisor añade automáticamente: workstream seleccionado, rama, nota de la ronda anterior, reglas de seguridad, y los objetivos reales de `MATTER_AI_HANDOFF.md`. El agente lee README/AGENTS/continuidad cuando las herramientas lo permiten; no dispone del historial de este chat.

## Instalación una sola vez bajo usuario Sakura

Desde un terminal Ubuntu con Git y Python 3 instalados:

```bash
git clone -b workstreams/02-base https://github.com/Matterssmith/Base-Matter.git ~/Base-Matter
cd ~/Base-Matter
bash tools/install-matter-ai.sh
~/.local/bin/matter-ai setup
```

Los archivos se instalan en `~/.local/share/matter-ai`. El instalador copia `~/.config/systemd/user/matter-autopilot.service`, **pero no lo activa** ni pide sudo. Si ya está instalado, actualiza primero el checkout fuente con `git pull --ff-only` (solo cuando esté limpio) y repite `bash tools/install-matter-ai.sh` para actualizar el ejecutable.

Descarga únicamente los proyectos a desarrollar, por ejemplo:

```bash
~/.local/bin/matter-ai sync matter-rust
~/.local/bin/matter-ai sync msel
~/.local/bin/matter-ai sync social
~/.local/bin/matter-ai sync base
```

`matter-ai sync all` intentará clonar todos los proyectos habilitados (excepto Bot-Matter). Los privados requieren haber configurado antes `gh auth login` y `gh auth setup-git`; la clonación puede ocupar varios GB. Si ya hay archivos o una rama distinta en el directorio, no los sobrescribe.

## Arranque continuo

Se recomienda probar el inventario **sin API ni cobro**:

```bash
~/.local/bin/matter-autopilot preview
```

Configura intervalos y límites razonables (900 segundos, 12 rondas/día, máximo 5 llamadas de modelo por ronda):

```bash
~/.local/bin/matter-autopilot configure --interval 900 --daily 12 --calls 5
```

Un ciclo requiere varias llamadas API y el número de tokens puede variar enormemente. **12 ciclos no significa 12 solicitudes ni un presupuesto garantizado en dinero.** Configura límites de gasto en OpenAI Platform. La API se factura aparte de ChatGPT Plus.

Ejecuta una ronda de prueba deliberada:

```bash
~/.local/bin/matter-autopilot once
```

Habilita el servicio del usuario Sakura:

```bash
~/.local/bin/matter-autopilot start
```

`start` pide escribir `SI` para autorizar el servicio continuo. El proceso queda administrado por `systemd --user`. Ejecuta una ronda por intervalo y rota entre **clones existentes y limpios**; los 27 workstreams están documentados, pero Bot-Matter permanece bloqueado. Si todos están sucios, el modelo no consume API hasta que se revisen los cambios.

Para que siga arrancando después de reiniciar aunque no inicies sesión como Sakura, un administrador de Ubuntu debe habilitar una vez:

```bash
sudo loginctl enable-linger Sakura
```

No hace falta que Sakura tenga permisos sudo. Para comprobar que su gestor de usuario está disponible:

```bash
systemctl --user status matter-autopilot.service
```

En una sesión abierta mediante `sudo -iu Sakura` puede no existir un `systemd --user` operativo; usa el login SSH real de Sakura con clave pública o comprueba el gestor de usuario con el administrador antes de iniciar. Linger inicia el gestor al arrancar, **no enciende una laptop apagada/suspendida**. El equipo debe permanecer encendido y conectado a internet.

## Seguimiento desde consola

```bash
~/.local/bin/matter-autopilot status
journalctl --user -u matter-autopilot.service -n 50 --no-pager
~/.local/bin/matter-autopilot stop
```

`status` muestra proyecto anterior, rondas consumidas en el día, último resultado, repositorios aptos y commits por publicar. `stop` deshabilita el servicio sin cerrar sesión ni borrar cambios. Los registros privados residen en:

- `~/.local/state/matter-ai/autopilot-state.json`: contador persistente, índice y último resumen por proyecto.
- `~/Matter/checkpoints/ID`: metadatos de commits y progreso.
- `~/Matter/workspaces/ID`: archivos de código y commits Git locales.

## Respaldo en GitHub

**Por defecto no hace push automático.** Los cambios autorizados se confirman en commits locales sobre la rama real `workstreams/NN-id`. Ninguna rama `main`, `matter`, `alpha`, `beta` o `stable` se sobrescribe.

Si quieres además que publique automáticamente esos commits a **su misma rama de trabajo**, habilítalo después de probar `once`:

```bash
~/.local/bin/matter-autopilot configure --interval 900 --daily 12 --calls 5 --push-workstreams
```

Te pedirá escribir exactamente `AUTORIZO_PUSH`. Este permiso persistente significa que las rondas posteriores pueden publicar en GitHub sin otra confirmación. La publicación requiere Git autenticado en Sakura y solo permite push normal, nunca `--force` o merge. La salida de CI posterior y las revisiones siguen pendientes y deben comprobarse separadamente.

Para revocar ese permiso sin detener el agente:

```bash
~/.local/bin/matter-autopilot configure --interval 900 --daily 12 --calls 5 --no-push
```

Para elevar la frecuencia, usa límites explícitos, por ejemplo `--interval 600 --daily 48`; **puede generar un coste considerable** si trabaja continuamente. El tope diario se reinicia cada día UTC. No hay garantía de actividad ilimitada: sin saldo, red, workspaces elegibles, o con error de la API, reduce actividad y conserva registros.

## Qué puede hacer mientras no estás

Puede leer fuentes no secretas, leer el traspaso, editar archivos de código y documentación con extensiones permitidas, registrar el resumen, validar rutas y crear commits locales. Cuando está habilitado, también publica su rama exacta. No ejecuta shell arbitrario, compilaciones ni pruebas que puedan ejecutar scripts, no toca `tools/`, `scripts/`, `.github/`, `deploy/`, `vault/`, `AGENTS.md`, paquetes lock, sesiones ni credenciales. Si se requiere modificar esas zonas o ejecutar pruebas, deja el proyecto pendiente para revisión manual. Esto es un **modo acotado**, no un sustituto de revisión humana.

Los errores de código no se convierten automáticamente en funciones verificadas. Un commit autónomo no constituye evidencia de CI ni autoriza desplegar el bot. Las pruebas completas y las promociones `alpha → beta → stable` siguen por PR y aprobación. `Bot-Matter` continúa bloqueado.

## Recuperación

Ante fallo o reinicio, `systemd` puede reiniciar el proceso, que retoma leyendo `autopilot-state.json`. El contador se escribe **antes** de cada ronda API, para no cobrar vueltas adicionales por una caída. El bloqueo de archivo Linux impide dos instancias simultáneas. Tras 429 se aplaza la siguiente ronda y se evitan reintentos rápidos. Si aparece un proyecto con archivos sin commit, se excluye temporalmente para no sobrescribirlos, y se informa mediante `matter-ai workstatus ID`.

El trabajo local no protege contra pérdida de disco; solo el push completado a GitHub es respaldo remoto de código. No equivale a respaldo de las sesiones privadas de Vault, que necesitan proceso separado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
