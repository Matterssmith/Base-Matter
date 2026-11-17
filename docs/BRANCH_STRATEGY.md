# Ramas de Base-Matter — stable / beta / alpha

## Ramas creadas el 17 de noviembre de 2026

| Estado | Rama | Procedencia | Requisito |
| --- | --- | --- | --- |
| Stable | `stable` | `matter` en `0a33ccdb25bb5bb46fdee482002b395619612f2b` | Solo releases autorizadas |
| Beta | `beta` | `matter-beta` en `8e4bcede9a711ea871a74fadc10eef1baada18cf` | CI + revisión de integración |
| Alpha | `alpha` | `matter-beta` en `8e4bcede9a711ea871a74fadc10eef1baada18cf` | Nuevas capacidades |

Conservar `matter`, `matter-beta` y ramas auxiliares existentes durante
la transición. GitHub mantiene `matter` como rama predeterminada hasta que
un administrador cambie esa preferencia en Settings > General > Default branch
a `stable`. Crear una rama no cambia la rama predeterminada.

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

No eliminar `matter` ni `matter-beta` antes de actualizar referencias y
validar que todas las instalaciones usan la nueva ruta. Repositorio GitHub
permanece fuente de verdad.

Créditos:
Hecho por Lournyx Matterssmith [Lou-Nyx].
