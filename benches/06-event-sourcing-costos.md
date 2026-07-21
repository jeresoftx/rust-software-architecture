# Costos educativos: Event sourcing

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

Event sourcing no se justifica porque recorrer un `Vec` sea más rápido que leer
un struct con estado actual. Se justifica cuando conservar historia, auditar
decisiones y reconstruir vistas vale más que la simplicidad de guardar solo una
foto del presente.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Evento como hecho aceptado | Logs técnicos que no explican el dominio | Nombrar hechos con precisión |
| Stream append-only | Estado actual sin historia verificable | Versionar y validar orden |
| Rehidratación determinista | Transiciones imposibles ocultas en persistencia | Aplicar historia antes de decidir |
| Auditoría desde el stream | Bitácoras paralelas incompletas | Almacenamiento creciente |
| Reconstrucción de proyecciones | Vistas imposibles de regenerar | Migrar eventos y cuidar compatibilidad |
| No editar eventos pasados | Correcciones silenciosas que destruyen confianza | Compensaciones explícitas |

La métrica pedagógica es esta: si una persona puede reconstruir el estado final
y explicar por qué ocurrió desde la misma historia, event sourcing está
aportando claridad real. Si solo hay más archivos y más tipos sin una pregunta
histórica que responder, el costo no está justificado.
