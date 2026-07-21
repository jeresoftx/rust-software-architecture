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
- El mismo dominio se rediseña varias veces para comparar costos, no para
  simular progreso artificial.
- Ninguna explicación debe vender una receta sin nombrar su costo operacional y
  organizacional.

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

El caso común se documenta con más detalle en
[`00-caso-comun-y-frontera.md`](./00-caso-comun-y-frontera.md). La decisión
canónica es que el motor de reservas funciona como laboratorio arquitectónico,
no como producto final.

## Frontera con otros repositorios

Este curso es canónico para la forma interna y evolutiva del software: límites,
dependencias, contratos, casos de uso, módulos, eventos, servicios e impactos de
operación.

No reemplaza:

- `rust-system-design`, donde se decide el diseño de sistemas completos;
- `rust-distributed-systems`, donde viven coordinación, fallas distribuidas,
  relojes, consenso y replicación;
- `rust-database-internals`, donde se estudian motores, transacciones, índices y
  query optimizers;
- `rust-design-patterns`, donde se ordenan patrones y principios reutilizables;
- `rust-cloud`, donde se estudian plataformas, despliegue, IAM, red y costos;
- `rust-travel`, que puede tomar el dominio de reservas como proyecto integrador
  de producto.

La frontera práctica es esta: si una decisión existe para enseñar organización
arquitectónica, pertenece aquí; si existe para construir producto, operar una
plataforma, implementar un motor de datos o resolver teoría distribuida, debe
vivir en el curso vecino correspondiente.

## Límite editorial

Un capítulo puede estar técnicamente completo y aun así no estar publicado.
`reviewed` y `published` requieren revisión humana explícita de Joel, conforme
a RFC-0001 §20.
