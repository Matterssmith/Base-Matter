# Historial de decisiones de implementación — Base-Matter

## BASE-D006 — Fixture dinámico del menú y guías WhatsApp
- Falla CI: tests/whatsapp_public_readonly.rs probaba la nueva
  navegación con un renderer de plantilla estática antigua que sólo
  soporta "menu". No es fallo de permisos de la ruta pública.
- Solución: mantener fixture legado para sus tests y probar
  categorías/páginas en fixture con MenuPreferences real.
- Formato: corrección de rustfmt del match del proveedor.
- Documentos: instrucciones precisas de prueba audio/HTML/TS18/TS19.
- Estado: CI nuevo por verificar; ningún dato secreto modificado.
- README revisados: 2/2.


## BASE-D005 — Rustfmt laboratorio, 18-11-2026
- Problema: CI de provider y Rustcore rechazaron formato de arrays,
  match e imports del laboratorio y los tests de catálogo.
- Decisión: aplicar rustfmt sin variar comportamiento ni permisos.
- Evidencia: workflows de SHA 05ec05c5bba5bb33b63dcb9aec457e2f8fcedda4.
- Resultado: archivos modificados y publicados, CI nueva pendiente.
- README revisados: 2/2.


## BASE-D004 — Ingreso de 21 categorías y páginas, 18-11-2026
- Se evita que el click de un selector ya autorizado sea descartado
  por el signer público, manteniendo lectura sin permisos de modificación.
- Las 21 rutas visibles y las páginas 1–3 son entradas válidas;
  aliases históricos sólo de compatibilidad.
- Pruebas: signer read-only y despachador con las 21 entradas,
  seguridad de botones, vista HTML con audio original.
- Estado: CI pendiente, teléfono todavía no probado. README 2/2.


## BASE-D003 — Menú HTML y sonido TS18/TS19, 2026-11-18
- Origen: prueba visual =/>menu pedida por el propietario.
- Decisión: un selector y 21 categorías primero; tarjetas TS18
  (Audio/MP4), TS19 (Audio/MP4/Enlace), RichResponse y un tono
  de 0.45s como extras sólo en self-chat verificado y opt-in.
- Implementación: proveedor Rust, audio Opus original, sin JavaScript,
  suplantación Meta AI, descargas ni roles; fallback texto.
- Pendiente: CI Linux/Windows/proveedor, móvil real, compatibilidad
  RichResponse y botones. No afirmar éxito visual sin pantalla.


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
