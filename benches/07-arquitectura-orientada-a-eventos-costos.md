# Costos educativos: Arquitectura orientada a eventos

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

La arquitectura orientada a eventos no se justifica porque publicar en un
`Vec` sea rápido. Se justifica cuando varios consumidores necesitan reaccionar
a un hecho sin acoplarse al productor. Medir nanosegundos de un bus en memoria
no enseña la decisión arquitectónica: el costo real vive en contratos,
duplicados, reintentos, observabilidad y gobierno del mapa de eventos.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Evento como hecho de integración | Llamadas directas a cada consumidor | Diseñar contrato y nombre estable |
| Productor sin consumidores concretos | Acoplamiento por cada nueva reacción | Trazabilidad entre productor, evento y consumidor |
| Bus o broker | Cascadas síncronas rígidas | Operación, monitoreo y reintentos |
| Consumidores idempotentes | Efectos duplicados ante reentrega | Memoria o persistencia de eventos procesados |
| Versionado de contrato | Rupturas silenciosas entre equipos | Compatibilidad y migración de consumidores |
| Manejo de fallas por consumidor | Pérdida del hecho publicado | Dead letters, alertas y reprocesamiento |

La métrica pedagógica es esta: si agregar un consumidor nuevo no obliga a
editar el productor y el sistema puede explicar qué evento falló, quién lo
procesó y qué contrato esperaba, la arquitectura está comprando desacoplamiento
real. Si solo hay más mensajes sin trazabilidad, el costo supera el beneficio.
