# Costos educativos: Monolito modular

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

La decisión arquitectónica del monolito modular no se evalúa por nanosegundos
entre funciones dentro del mismo proceso. En esta etapa, el costo relevante es:

- cuántos contratos internos deben mantenerse;
- cuánta visibilidad pública se expone;
- cuántas pruebas protegen invariantes reales;
- qué tan fácil es cambiar inventario sin romper reservas;
- qué tan fácil es leer el flujo sin abrir todo el repositorio.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Un solo despliegue | Red, fallas parciales, versionado entre servicios | Coordinación de releases dentro del mismo binario |
| Módulos con campos privados | Mutaciones accidentales entre áreas | Diseño explícito de APIs internas |
| Contratos internos pequeños | Dependencias improvisadas | Pruebas y nombres más deliberados |
| Pruebas de invariantes | Regresiones silenciosas | Más casos que mantener |

La métrica pedagógica para este capítulo es cualitativa: si el lector puede
explicar qué módulo protege cada regla y qué dependencia está permitida, el
modelo está cumpliendo su propósito.
