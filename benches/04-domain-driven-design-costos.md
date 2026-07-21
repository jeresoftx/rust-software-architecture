# Costos educativos: Domain-Driven Design

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

DDD no se justifica porque un value object sea más rápido que un `String`.
Se justifica cuando el costo de perder lenguaje de negocio supera el costo de
nombrar reglas, identidades, transiciones y fronteras.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Value objects con invariantes | Datos primitivos inválidos viajando por el sistema | Más constructores y errores de dominio |
| Aggregate root explícito | Reglas de transición duplicadas en servicios o adaptadores | Decidir una frontera de consistencia |
| Evento de dominio como hecho | Auditoría implícita o lenguaje ambiguo sobre lo ocurrido | Versionar significado de eventos |
| Repositorio de agregado | Persistencia decidiendo reglas del dominio | Mapear el agregado sin romper invariantes |
| Bounded context nombrado | Palabras con significados mezclados entre equipos | Mantener traducciones entre contextos |

La métrica pedagógica es esta: si una persona puede leer el código y reconocer
las palabras del dominio sin adivinar en qué servicio vive cada regla, DDD está
aportando claridad real.
