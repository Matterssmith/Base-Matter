# Base-Matter — migración de responsabilidades al Engine-Matter

**Decisión del propietario:** 2026-11-23. **Estado:** plan publicado, código ejecutable sin modificar.

## Responsabilidades definitivas

1. **MSEL** (privado) sigue siendo el lenguaje y toolchain oficial P0, con lexer/parser/IR/ABI propios. No se traslada ni se publica su implementación privada.
2. **Engine-Matter** (futuro nombre del actual WhatsApp-Matter) es el motor central de sistemas Matter, incluida la capacidad de hospedar Social-Matter y bots como extensiones. La carpeta `Matter-MSEL/` vivirá en Engine-Matter, con interfaces para consumir MSEL sin duplicar el compilador privado.
3. **Base-Matter** permanece como biblioteca reusable, contratos, servicios fundamentales y políticas desacopladas de plataforma. La relación preferida es Engine → Base; nunca Base → Engine si produce un ciclo.
4. **Engine-Extensions-Matter** (futuro Telegram-Matter) implementa todos los adaptadores de redes sociales ajenas como WhatsApp, Telegram y Discord. No se importan sus protocolos en Engine-Matter. La interfaz de transporte usa eventos de datos no confiables, identidad autenticada, ACL, consentimiento, anti-replay y permisos Vault fail-closed.
5. **Social-Matter** es sistema propio Matter, distinto de proveedores de redes externas. **Extensions-Matter** (sin prefijo Engine) es el proyecto privado existente de módulos generales/Anti, distinto del singular Engine-Extensions-Matter.
6. **Discord-Matter** se eliminará solamente después de inventario, respaldo, migración de referencias/código y verificación. **Bot-Matter** sigue bloqueado hasta cumplir criterios reales.

## Inventario actual pendiente de extraer

- `providers/whatsapp-rust/` contiene código proveedor y README. Conservarlo durante la preparación: el sistema activo no debe quedar sin soporte.
- `src/msel_bridge.rs`, `tests/msel_bridge.rs`, `docs/MATTER_MSEL_MASTER_SPEC.md` y `docs/MSEL_WHATSAPP_INTEGRATION.md` requieren refactor a contratos de Engine/Matter-MSEL desacoplados del transporte; no borrarlos mientras no existan pruebas de equivalencia.
- Gateway y comandos `=/> `, sesiones, identidad, permisos, políticas, menú y cifrado deben permanecer funcionales durante la transición. El motor deberá poder ejecutarse sin WhatsApp por defecto.

## Fases de entrega

1. Conservar SHAs, estado CI, remotos, workflows, inventario de referencias y respaldo antes de editar Rust.
2. Especificar contrato Engine↔Base y Engine↔Extension versionado, mínimo y sin ciclos. Separar núcleo del protocolo WhatsApp y preparar feature flags/despliegues sin proveedor externo.
3. Copiar (no borrar todavía) código transport-specific desde WhatsApp-Matter a Engine-Extensions-Matter y completar pruebas de Linux/Windows, autenticación, consentimiento, replay y emisión.
4. Integrar Matter-MSEL en Engine como host del lenguaje MSEL original; probar ABI y capacidades con aplicaciones Matter internas (Social-Matter). En árboles Git alpha inspeccionados no apareció una carpeta previa `Matter-MSEL`; por ahora solo está preparado el destino.
5. Verificar bajo CI que Engine compila y arranca sin WhatsApp/Telegram/Discord, que los conectores pueden desconectarse sin afectar servicios internos y que Vault bloquea cualquier operación no autorizada.
6. Cutover controlado de WhatsApp con una única sesión, sin duplicar pairing o leer/copiar secretos, manteniendo rollback. Solo tras pruebas reales eliminar duplicaciones y renombrar repositorios desde GitHub admin; archivar/eliminar Discord-Matter tras respaldo.

## Criterios y pendientes

**No hechos**: traslado de código, renombrados GitHub, tests de arquitectura, CI o implementación de adaptadores. **No cambiar** nombre Cargo, `src/`, worktrees, instancias ni sesiones en producción por este plan. Conservar documentos de continuidad y revisar ambos README en cada commit. MSEL P0, Bot-Matter bloqueado.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
