## 2026-11-19 — Rustfmt final de selector de categorías

- Base alpha: 605ae3bf3da68de49276f1ef9d68d6e1b77f8161; Linux Rust completó correctamente sus pruebas de formato, compilación, tests, Clippy y documentación.
- El proveedor Rust falló sólo en formato de la expresión que valida el manifiesto. Se aplicó el diff exacto del CI.
- Windows y proveedor del nuevo SHA requieren nueva confirmación. Sin cambios en permisos, sesiones, Vault o plantilla visual.
- README revisados: 2/2. Continuidad: validar CI para este SHA y probar =/>menu en WhatsApp real.

## 2026-11-19 — Rustfmt y tipado del cambio de menú

- Base alpha previa: 83cc0f993e24b1ea70659c6e11d5bf8277015302.
- GitHub Actions de la revisión inicial: formato FAIL en proveedor y Rust; se aplicaron exactamente las diferencias de rustfmt de ambos trabajos.
- Se hace explícita la comparación `&str` frente a `String` del manifiesto de categorías activas, y el índice de los alfabetos JSON.
- No cambian categorías, alcance, comandos, botones, permisos ni datos de Vault.
- Siguiente: comprobar CI Linux/Windows/proveedor y después pantalla real en WhatsApp.
- README revisados: 2/2. CI del siguiente SHA: pendiente.

## BASE-D009 — UX Leito y alfabetos reutilizables (2026-11-19)

- Propuesta recibida: hacer perceptible la diferencia entre adorno y contenido mediante formatos de WhatsApp.
- Decisión: ocultar categorías sin comandos implementados en menú y botones; mantener 21 identificadores solo en contrato interno y routing histórico de lectura.
- Usar manifiesto compartido en Rust y catálogo JSON de cinco alfabetos, marco reutilizable y módulo de formato. No conceder permisos ni añadir comandos.
- Base: 8639b9e4beab5217f344766d74c1b85578b3daba. CI pendiente; READMEs 2/2.

# Historial de decisiones de implementación — Base-Matter

## BASE-D008 — corrección de casos positivos/negativos de grupo
- Incidencia en  ce9465a8: reemplazo inicial mal ubicado.
- Ahora `=/>menu group` aparece solo en el conjunto autorizado
  y `=/>menu unknown` solo en el conjunto rechazado.
- El código de seguridad no cambió; solo la prueba.
- README revisados: 2/2; CI pendiente.


## BASE-D007 — recuperación del checkpoint y test de grupo
- SHA anterior: 97de8ac040148925baa194710a0d5dab82d90aff.
- Incidencia: el commit previo omitió WORK_CONTINUITY.
- Prueba: `menu group` es una categoría permitida, no debe
  figurar en la lista de denegaciones del signer.
- Corrección: sustituida por `menu unknown`.
- Resultado: revalidación nueva pendiente; README 2/2.


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
