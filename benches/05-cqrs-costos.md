# Costos educativos: CQRS

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

CQRS no se justifica porque crear dos structs sea más rápido que usar uno. Se
justifica cuando el costo de mezclar escritura y lectura empieza a romper
invariantes, claridad o evolución. Por eso la medición de este capítulo es
conceptual: compara qué costo se vuelve visible cuando una operación de cambio
y una operación de lectura dejan de compartir el mismo modelo.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Comando explícito | Cambios implícitos escondidos en métodos de consulta | Más tipos y validación de intención |
| Modelo de escritura separado | Consultas deformando invariantes de negocio | Traducir hechos hacia lectura |
| Evento como hecho observable | Cambios difíciles de proyectar o auditar | Definir significado y versión del hecho |
| Proyección de lectura | Reportes consultando directamente el agregado | Reconstrucción y posible retraso |
| Consulta sin mutación | Lecturas con efectos secundarios accidentales | Mantener vistas preparadas para leer |
| Reconocer retraso de proyección | Tratar una vista vieja como verdad absoluta | Observabilidad y manejo de consistencia eventual |

La métrica pedagógica es esta: si una persona puede señalar qué tipo cambia
estado, qué tipo comunica el hecho y qué tipo lee sin mutar, CQRS está
aportando claridad real. Si no puede hacerlo, la separación solo agregó ruido.
