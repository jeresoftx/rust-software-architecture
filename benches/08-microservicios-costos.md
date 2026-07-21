# Costos educativos: Microservicios

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

Microservicios no se justifican porque llamar funciones en módulos separados sea
rápido o lento. Se justifican cuando la autonomía de despliegue, ownership y
evolución compensa el costo de operar una red de servicios. Medir nanosegundos
del modelo en memoria no enseña la decisión arquitectónica: el costo real vive
en fallas parciales, contratos, datos propios, observabilidad y coordinación
entre responsables.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Servicio con datos propios | Tablas compartidas como contrato oculto | Sincronización y duplicación explícita |
| Contrato versionado | Cambios internos que rompen consumidores | Compatibilidad y gobierno de versiones |
| Despliegue independiente | Coordinar cada release de todo el sistema | Pipelines, configuración y rollback por servicio |
| Llamada remota visible | Dependencias internas escondidas | Latencia, timeouts y fallas parciales |
| Ownership operativo | Responsabilidad difusa ante incidentes | On-call, monitoreo y soporte por servicio |
| Observabilidad distribuida | Incidentes imposibles de explicar | Trazas, correlación y costos de plataforma |

La métrica pedagógica es esta: si separar un servicio reduce coordinación real
y el equipo puede explicar contratos, datos, despliegues e incidentes de esa
frontera, microservicios está comprando autonomía. Si solo aumenta repositorios,
red y permisos sin ownership claro, el monolito modular era más honesto.
