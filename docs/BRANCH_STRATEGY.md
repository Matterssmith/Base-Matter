# Ramas de Base-Matter — stable / beta / alpha

## Ramas creadas el 17 de noviembre de 2026

| Estado | Rama | Procedencia | Requisito |
| --- | --- | --- | --- |
| Stable | `stable` | `matter` en `0a33ccdb25bb5bb46fdee482002b395619612f2b` | Solo releases autorizadas |
| Beta | `beta` | `matter-beta` en `8e4bcede9a711ea871a74fadc10eef1baada18cf` | CI + revisión de integración |
| Alpha | `alpha` | `matter-beta` en `8e4bcede9a711ea871a74fadc10eef1baada18cf` | Nuevas capacidades |

La rama predeterminada ya es `stable` (comprobado por GitHub). Solo las
tres ramas oficiales deben permanecer después de retirar las referencias
heredadas. Las ramas auxiliares de seguridad ya tienen 0 commits exclusivos
frente a `stable`; `matter` y `matter-beta` apuntan a los mismos commits
que los puntos iniciales de `stable` y `beta` respectivamente.
La eliminación de referencias no implica borrar archivos ni sesiones.

## Promoción, seguridad y compatibilidad

Desarrollar exclusivamente en `alpha`; antes de cada commit inspeccionar
y actualizar ambos README. Probar `cargo fmt --all -- --check`,
`cargo check --all-targets --locked`, `cargo test --all-targets --locked`,
CI Windows/Linux y el proveedor Rust por separado.

Revisar y seleccionar cambios seguros mediante PR `alpha` -> `beta` y
`beta` -> `stable`, evitando una fusión global que arrastre funciones
experimentales. Las ramas base son divergentes y se deben preservar sus
commits y `Cargo.lock`. Exigir CI verde sobre el SHA exacto, verificar
pairing real, y nunca declarar producción por tests offline.

Retirar `matter`, `matter-beta`, `stable-hardening-20261117` y
`stable-socket-owner-control-20261117` mediante una acción autorizada
de GitHub una vez actualizados los consumidores. El conector de este
chat no ofrece eliminación de refs; no fingir que se borraron hasta
verificar la lista de ramas. GitHub sigue siendo la fuente de verdad.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
