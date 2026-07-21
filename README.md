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

El repositorio acaba de entrar en desarrollo. Todos los capítulos están en
estado `planned`.

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

## Alcance del curso

Este repositorio sí cubre:

- límites entre módulos, capas, dominios, casos de uso e infraestructura;
- dependencias dirigidas por intención y no por accidente;
- contratos internos y externos;
- evolución de arquitectura sin reescrituras heroicas;
- costos operativos y organizacionales de cada estilo;
- modelos pequeños en Rust para razonar sobre invariantes arquitectónicos.

Este repositorio no reemplaza:

- `rust-system-design`: allá se diseñan sistemas completos; aquí se estudia la
  organización interna y evolutiva del software;
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
| 01 | Monolito modular | `src/modular_monolith.rs` | planned |
| 02 | Arquitectura hexagonal | `src/hexagonal_architecture.rs` | planned |
| 03 | Clean Architecture | `src/clean_architecture.rs` | planned |
| 04 | Domain-Driven Design | `src/domain_driven_design.rs` | planned |
| 05 | CQRS | `src/cqrs.rs` | planned |
| 06 | Event sourcing | `src/event_sourcing.rs` | planned |
| 07 | Arquitectura orientada a eventos | `src/event_driven_architecture.rs` | planned |
| 08 | Microservicios | `src/microservices.rs` | planned |

Estados posibles: `planned`, `draft`, `implemented`, `tested`,
`benchmarked`, `reviewed`, `published`.

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
