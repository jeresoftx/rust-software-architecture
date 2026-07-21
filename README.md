# Rust Software Architecture

Repositorio del camino troncal de Jeresoft Academy para estudiar arquitectura
de software en Rust. Pertenece al Semestre 5 del plan de estudios junto con
`rust-cloud` (RFC-0001 §10).

El objetivo no es memorizar nombres de estilos arquitectónicos. El objetivo es
aprender a decidir límites, dependencias, contratos, flujos de datos, evolución
técnica y tradeoffs cuando un sistema deja de ser un ejercicio aislado y empieza
a sostener negocio, equipos y cambios.

## Qué contiene

- Capítulos en Markdown compatibles con mdBook.
- Modelos Rust educativos para representar límites arquitectónicos.
- El mismo dominio pequeño rediseñado bajo varios estilos.
- Diagramas Mermaid y plantillas de decisión.
- Tests, ejemplos y benchmarks cuando el capítulo lo justifique.
- Ejercicios graduados con soluciones sugeridas.

## Estado editorial

El repositorio acaba de entrar en desarrollo. Los capítulos `01. Monolito
modular`, `02. Arquitectura hexagonal`, `03. Clean Architecture`,
`04. Domain-Driven Design`, `05. CQRS`, `06. Event sourcing` y `07.
Arquitectura orientada a eventos` y `08. Microservicios` están en estado
`draft`; `03. Clean
Architecture` ya cuenta con modelo Rust mínimo, diagrama, ejemplos progresivos,
ejercicios, solución sugerida y análisis de costos. `04. Domain-Driven Design`
ya cuenta con especificación conceptual, modelo Rust mínimo, diagrama, ejemplos
progresivos, ejercicios, solución sugerida y análisis de costos. `05. CQRS` ya
cuenta con especificación conceptual, modelo Rust mínimo, diagrama Mermaid,
ejemplos progresivos, ejercicios, solución sugerida y análisis de costos. `06.
Event sourcing` ya cuenta con especificación conceptual, modelo Rust mínimo,
diagrama Mermaid, ejemplos progresivos, ejercicios, solución sugerida y análisis
de costos. `07. Arquitectura orientada a eventos` ya cuenta con especificación
conceptual, modelo Rust mínimo, diagrama Mermaid, ejemplos progresivos,
ejercicios, solución sugerida y análisis de costos. El resto de capítulos sigue
en estado `planned`. `08. Microservicios` ya cuenta con especificación
conceptual de límites de servicio, datos propios, operación, costos,
alternativas, invariantes y modos de falla.

Ningún capítulo está marcado como `reviewed` ni `published`: la revisión humana
de Joel sigue siendo obligatoria antes de considerar el curso listo para
publicación o ingestión en `academy-web`.

## Lugar en el camino

Este curso vive en el Semestre 5. Recibe ideas de `rust-system-design`,
`rust-distributed-systems`, `rust-database-internals`, `rust-concurrency` y
`rust-design-patterns`. Alimenta `rust-cloud`, `rust-ai-engineering`,
`rust-travel` y los proyectos integradores.

`rust-software-architecture` es canónico para monolito modular, arquitectura
hexagonal, Clean Architecture, Domain-Driven Design, CQRS, event sourcing,
arquitectura orientada a eventos y microservicios.

El caso común es un motor de reservas educativo. Se usa como laboratorio para
comparar límites y costos arquitectónicos; no convierte este repositorio en
`rust-travel` ni en una aplicación productiva de viajes.

## Alcance del curso

Este repositorio sí cubre:

- límites entre módulos, capas, dominios, casos de uso e infraestructura;
- dependencias dirigidas por intención y no por accidente;
- contratos internos y externos;
- evolución de arquitectura sin reescrituras heroicas;
- costos operativos y organizacionales de cada estilo;
- modelos pequeños en Rust para razonar sobre invariantes arquitectónicos.

Este repositorio no reemplaza:

- `rust-system-design`: allá se diseñan sistemas completos, tráfico, capacidad,
  APIs y escenarios de producto; aquí se estudia la organización interna y
  evolutiva del software;
- `rust-distributed-systems`: allá viven consenso, relojes lógicos y
  tolerancia a fallas;
- `rust-database-internals`: allá viven almacenamiento, transacciones, MVCC y
  query optimizer;
- `rust-design-patterns`: allá viven principios y patrones idiomáticos;
- `rust-cloud`: allá viven plataformas, cómputo, redes, IAM, costos y servicios
  manejados.

## Capítulos

| # | Capítulo | Módulo inicial | Estado |
|---|----------|----------------|--------|
| 01 | Monolito modular | `src/modular_monolith.rs` | draft |
| 02 | Arquitectura hexagonal | `src/hexagonal_architecture.rs` | draft |
| 03 | Clean Architecture | `src/clean_architecture.rs` | draft |
| 04 | Domain-Driven Design | `src/domain_driven_design.rs` | draft |
| 05 | CQRS | `src/cqrs.rs` | draft |
| 06 | Event sourcing | `src/event_sourcing.rs` | draft |
| 07 | Arquitectura orientada a eventos | `src/event_driven_architecture.rs` | draft |
| 08 | Microservicios | `src/microservices.rs` | draft |

Estados posibles: `planned`, `draft`, `implemented`, `tested`, `benchmarked`,
`reviewed`, `published`.

## Estructura esperada

```text
AGENTS.md
ROADMAP.md
LICENSE.md
LICENSE-MIT
LICENSE-APACHE
LICENSE-CC-BY-SA-4.0.md
course.manifest.json
docs/
  SUMMARY.md
  00-introduccion.md
  00-ruta-de-lectura.md
  00-glosario.md
  manifiesto-curso.md
  00-caso-comun-y-frontera.md
  plantilla-capitulo-arquitectura.md
  flujo-autonomo.md
  superpowers/plans/
src/
examples/
  soluciones/
tests/
benches/
diagrams/
assets/
```

## Cómo usarlo

Leer primero:

1. [`docs/00-introduccion.md`](./docs/00-introduccion.md)
2. [`docs/00-ruta-de-lectura.md`](./docs/00-ruta-de-lectura.md)
3. [`docs/00-glosario.md`](./docs/00-glosario.md)
4. [`docs/manifiesto-curso.md`](./docs/manifiesto-curso.md)
5. [`docs/00-caso-comun-y-frontera.md`](./docs/00-caso-comun-y-frontera.md)
6. [`docs/plantilla-capitulo-arquitectura.md`](./docs/plantilla-capitulo-arquitectura.md)

Ejecutar tests:

```bash
cargo test
```

Formatear:

```bash
cargo fmt
```

Verificación completa:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo test --doc
```

## Gobernanza

- `AGENTS.md` es la guía de arranque para humanos e IA en este repositorio.
- `ROADMAP.md` registra el avance del curso sin convertirlo en una fecha
  límite.
- Los planes de implementación viven en `docs/superpowers/plans/`.
- `LICENSE.md` resume la doble licencia: código bajo `MIT OR Apache-2.0`;
  contenido educativo bajo `CC BY-SA 4.0`.

## Filosofía

Este repositorio debe poder leerse como un libro de ingeniería. La claridad
gana sobre el ingenio, la calidad gana sobre la velocidad, y ningún capítulo se
considera publicable hasta cumplir la anatomía completa de RFC-0001 §14.
