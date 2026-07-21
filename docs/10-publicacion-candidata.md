# Publicación candidata

Este documento prepara una revisión integral del curso como candidato técnico de
publicación. No publica el curso y no cambia ningún capítulo a `reviewed` ni
`published`.

## Estado candidato

Estado editorial global: `draft`.

Los ocho capítulos cuentan con:

- especificación conceptual;
- modelo Rust mínimo;
- pruebas;
- diagrama Mermaid;
- ejemplos progresivos;
- ejercicios graduados;
- solución sugerida;
- análisis educativo de costos o nota de no aplicación.

La navegación mdBook está configurada en `book.toml` y el índice vive en
`docs/SUMMARY.md`.

## Verificaciones requeridas

La publicación candidata debe repetir esta batería antes de revisión humana:

```bash
cargo fmt --check
git diff --check
cargo test --doc
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo bench
mdbook build -d /tmp/rust-software-architecture-book
```

Además, se debe verificar que `course.manifest.json` sea JSON válido y que los
enlaces relativos locales de `docs/` y `README.md` existan.

## Resultado de verificación

Resultado actual de la publicación candidata:

- `cargo fmt --check`: correcto;
- `git diff --check`: correcto;
- `jq empty course.manifest.json`: correcto;
- `cargo test --doc`: correcto;
- `cargo clippy --all-targets --all-features -- -D warnings`: correcto;
- `cargo test --all-targets`: correcto, 30 pruebas de biblioteca pasan y los
  ejemplos compilan;
- `cargo bench`: correcto; no hay benchmarks Rust medidos, solo notas
  educativas de costos por capítulo;
- `mdbook build -d /tmp/rust-software-architecture-book`: correcto;
- verificación local de enlaces relativos en `docs/` y `README.md`: correcta;
- búsqueda de codificación rota y errores comunes visibles: sin hallazgos.

## Revisión editorial básica

Antes de pedir revisión humana, se revisan:

- acentos y `ñ` en español es-MX;
- nombres propios: Jeresoft Academy, Joel, Rust, GitHub, mdBook, Mermaid;
- consistencia de estados editoriales;
- ausencia de texto con codificación rota;
- ausencia de capítulos marcados como `reviewed` o `published`;
- consistencia entre README, ROADMAP, manifiesto, SUMMARY y manifest JSON.

Esta revisión no sustituye lectura humana. Solo reduce ruido antes de que Joel
evalúe criterio, tono, profundidad, secuencia y publicación real.

## Riesgos pendientes

- La revisión humana puede pedir reescrituras de tono, ejemplos o profundidad.
- Los diagramas Mermaid aún deben revisarse visualmente en el sitio final.
- `academy-web` puede requerir metadatos adicionales para ingestión.
- La ortografía automática no reemplaza lectura editorial completa.
- Los capítulos están completos para revisión candidata, pero no aprobados.
- La publicación real debe decidirse fuera del modo autónomo.

## Criterio de salida

El curso puede considerarse candidato técnico cuando:

- la suite completa pasa;
- mdBook construye localmente;
- los enlaces locales no están rotos;
- el repositorio queda limpio en `main`;
- todos los issues del milestone de cierre están cerrados;
- Joel acepta iniciar revisión humana.

Hasta entonces, el estado correcto sigue siendo `draft`.
