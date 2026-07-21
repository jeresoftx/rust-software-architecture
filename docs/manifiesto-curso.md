# Manifiesto del curso

`rust-software-architecture` existe para enseñar arquitectura con criterio de
ingeniería, no con frases de consultoría.

## Principios

- La arquitectura se evalúa por sus consecuencias.
- Un límite existe para proteger una razón de cambio.
- Una abstracción debe pagar renta: claridad, pruebas, evolución o reducción de
  riesgo.
- Los estilos arquitectónicos no son identidad; son herramientas.
- Microservicios son una decisión costosa, no una medalla.
- El código debe hacer visibles las decisiones.

## Caso común

El curso usa un motor de reservas educativo como hilo conductor. Cada capítulo
puede rediseñar el mismo dominio desde otra perspectiva para comparar costos:

- monolito modular para límites internos;
- arquitectura hexagonal para aislar infraestructura;
- Clean Architecture para dirigir dependencias;
- Domain-Driven Design para lenguaje e invariantes;
- CQRS para separar escritura y lectura;
- event sourcing para reconstruir estado desde hechos;
- arquitectura orientada a eventos para integración;
- microservicios para distribución organizacional y operativa.

## Límite editorial

Un capítulo puede estar técnicamente completo y aun así no estar publicado.
`reviewed` y `published` requieren revisión humana explícita de Joel, conforme
a RFC-0001 §20.
