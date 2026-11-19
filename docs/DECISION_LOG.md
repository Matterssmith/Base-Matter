# Historial de decisiones de implementación — Base-Matter

## 2026-11-18 (MX) — BASE-D002: permiso informado de Jeriel
- **Origen:** titular del ecosistema comunica autorización de Jeriel,
  creador de Aura Reed, para usar su clave de IA según lo planeado.
- **Límites:** no se informaron restricciones numéricas. NO confundir
  eso con cuota infinita, ausencia de términos del proveedor ni
  autorización para otras capacidades.
- **Estado:** consentimiento comunicado; evidencia/titularidad
  operativa pendiente de verificar por Vault, sin clave ni gateway.
- **Referencia autoritativa:** Vault-Matter privado, rama matter,
  docs/AI_PERMISSION_AURA_REED.md, commit
  8872f9c3dc3d6aafcf4268c531e7c07f571cbcf4.
- **Arquitectura:** Base-Matter continúa solicitando sólo resultados
  al broker autenticado de Vault. No extrae claves ni ejecuta el
  código sugerido por la IA. Capacidades previstas: guía de menús,
  revisión/propuestas de código con control independiente.
- **Archivos:** AGENTS, docs/MATTER_AI.md, documentos de continuidad
  y ambos README. No cambia transporte ni claves.

## 2026-11-18 (MX) — BASE-D001: registro de cada idea/cambio
- **Origen:** petición expresa de guardar instrucciones y punto
  exacto de avance en caso de perder la comunicación.
- **Regla:** AGENTS, este historial, docs/WORK_CONTINUITY.md
  y docs/DEVELOPMENT_STATUS.md se actualizan en cada commit.
  Auditar README rastreados (2/2).
- **Control:** scripts/check_continuity.py integrado a CI para
  impedir commits futuros que omitan estos registros.
- **Problema corregido:** test de Linux esperaba encabezado de help
  dentro de menu antis 2. Se separa menu de help. En Windows,
  tests/command_audit.rs chocaba en nombres temporales por PID/reloj.
  Se reemplaza la carpeta de prueba por nonce criptográfico.
- **Estado:** cambios incorporados en alpha, revalidación CI pendiente.
- **Siguiente:** validar Linux, Windows y proveedor sobre SHA exacto,
  revisar manualmente menú/help y selector en WhatsApp.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
