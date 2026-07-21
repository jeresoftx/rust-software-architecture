# Navegación y enlaces

Esta página registra la forma de navegación preparada para mdBook antes de la
publicación candidata. No cambia el estado editorial de los capítulos: todos
siguen en `draft` hasta revisión humana explícita de Joel.

## Orden pedagógico

La navegación conserva la progresión del curso:

1. **Inicio:** introducción, ruta de lectura, glosario, manifiesto, caso común,
   plantilla y flujo autónomo.
2. **Límites internos:** monolito modular, arquitectura hexagonal y Clean
   Architecture.
3. **Negocio y comportamiento:** Domain-Driven Design, CQRS y event sourcing.
4. **Distribución organizacional:** arquitectura orientada a eventos y
   microservicios.
5. **Cierre:** navegación, enlaces y publicación candidata.

## Enlaces revisados

La verificación de enlaces internos cubre:

- archivos enlazados desde `docs/SUMMARY.md`;
- enlaces relativos entre páginas de `docs/`;
- referencias desde capítulos hacia `diagrams/`, `examples/` y `benches/`;
- configuración `book.toml` con `src = "docs"`;
- construcción local con `mdbook build`.

Si un enlace apunta a GitHub, no se valida como archivo local. Si apunta a un
comando de ejemplo, se valida mediante las pruebas y ejemplos de Rust del
repositorio.

## Resultado actual

La navegación está lista para revisión candidata. La publicación sigue
bloqueada por revisión humana: preparar el libro no equivale a publicarlo.

Verificaciones realizadas:

- `mdbook build -d /tmp/rust-software-architecture-book`;
- verificación local de enlaces relativos en `docs/` y `README.md`;
- `cargo fmt --check`;
- `git diff --check`.

El documento de cierre vive en
[`10-publicacion-candidata.md`](./10-publicacion-candidata.md).
