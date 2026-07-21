# Costos educativos: Arquitectura hexagonal

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

La arquitectura hexagonal no se justifica porque un trait sea más rápido que una
llamada directa. Se justifica cuando el costo de depender de infraestructura
concreta supera el costo de nombrar puertos y adaptadores.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Puerto de salida explícito | Núcleo acoplado a base de datos o proveedor | Diseño y mantenimiento del contrato |
| Adaptador en memoria | Pruebas lentas o frágiles con infraestructura real | Riesgo de probar un doble demasiado cómodo |
| Error de puerto visible | Fallas de infraestructura ocultas como éxito | Manejo explícito de errores |
| Entrada validada antes del puerto | Datos inválidos persistidos por accidente | Validaciones repetidas si no hay buena frontera |

La métrica pedagógica es esta: si puedes reemplazar el adaptador sin tocar el
caso de uso y las pruebas del núcleo siguen pasando, la frontera hexagonal está
cumpliendo su propósito.
