# Costos educativos: Clean Architecture

Este capítulo no incluye benchmark de rendimiento con `cargo bench`.

Clean Architecture no se justifica porque una llamada a caso de uso sea más
rápida que una llamada directa. Se justifica cuando el costo de cambiar detalles
externos empieza a amenazar reglas estables de negocio.

## Benchmark conceptual

| Decisión | Costo que evita | Costo que agrega |
|----------|-----------------|------------------|
| Entidad con invariantes propias | Reglas dispersas en servicios o adaptadores | Más tipos pequeños que nombrar y probar |
| Caso de uso como orquestador | Flujo de aplicación mezclado con detalles técnicos | Decidir qué vive en aplicación y qué vive en dominio |
| Repositorio como contrato | Persistencia dictando la forma del núcleo | Mantenimiento de un trait estable |
| Entrada validada antes de persistir | Datos inválidos guardados por accidente | Traducción explícita desde datos primitivos |
| Error de repositorio visible | Confirmaciones falsas cuando falla infraestructura | Manejo de errores en la frontera de aplicación |

La métrica pedagógica es esta: si puedes cambiar el repositorio, agregar
auditoría o rechazar datos inválidos sin tocar la entidad `Reservation`, la
dirección de dependencias está haciendo trabajo real.
