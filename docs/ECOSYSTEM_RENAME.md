# Convención oficial: Matter al final

A partir del 17 de noviembre de 2026, la familia se denomina
\`<Componente>-Matter\`. El proyecto general conserva la marca
**Matterssmith**, los créditos y la historia Git.

## Renombrados de repositorios planificados

| Repositorio actual | Nombre nuevo |
| --- | --- |
| Matter-Base | Base-Matter |
| Matter-System | System-Matter |
| Matter-Interfaces | Interfaces-Matter |
| Matter-Baileys | WhatsApp-Matter |
| Matter-Telegraf | Telegram-Matter |
| Matter-Discord | Discord-Matter |
| Matter-Extensions | Extensions-Matter |
| Matter-SDK | SDK-Matter |
| Matter-Docs | Docs-Matter |
| Matter-AI | AI-Matter |
| Matter-API | API-Matter |
| Matter-MCP | MCP-Matter |
| Matter-Bot | Bot-Matter |
| Matter-Web | Web-Matter |
| Matter-Automation | Automation-Matter |
| Matter-Cloud | Cloud-Matter |
| Matter-Marketplace | Marketplace-Matter |
| Matter-Server | Server-Matter |
| Matter-App | App-Matter |
| Matter-OS | OS-Matter |
| Matter-Vault | Vault-Matter |
| Matter-Outdated | Outdated-Matter |

Los repositorios \`matterssmith.github.io\` y los repositorios de demostración
no se renombran: no pertenecen a la convención \`Matter-*\`.

## Compatibilidad / límites

**Esta documentación no renombra repositorios en GitHub.** El nombre real
debe cambiarlo una cuenta con permisos administrativos en cada repositorio.
La conexión GitHub de esta sesión permite operar ramas y archivos, pero
no ofrece una función para renombrar los repositorios.

Para ejecutar una migración controlada desde una máquina con GitHub CLI
autenticado y acceso administrativo, usa
\`python3 scripts/rename_ecosystem.py\` para revisar el plan y el comando
\`python3 scripts/rename_ecosystem.py --apply --only Matter-Base\` para
migrar un repositorio cada vez. El modo \`--apply --all\` es explícito y
solo se recomienda después de actualizar las referencias cruzadas.

Antes de renombrar, inspecciona las referencias GitHub Actions,
dependencias Cargo, URLs de aplicaciones, páginas y documentación. GitHub
puede redirigir URLs Git normales, pero **no garantiza redirecciones para
acciones referenciadas desde un repositorio renombrado**. Renombrar no cambia
\`matter\` ni \`matter-beta\` de sitio, pero los remotos locales deben pasar a
la URL nueva. Confirma la nueva URL y permisos después de cada paso.

El paquete Rust \`matter-base\` y el crate \`matter_base\` son identificadores
de compatibilidad actuales. No se renombran automáticamente mientras
existan herramientas, contratos y tests que los importen. Se deprecán o
migran en una versión con pruebas de consumidores, no por reemplazos masivos.

Este plan tampoco habilita Matter-Bot; el nombre de destino es Bot-Matter.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
