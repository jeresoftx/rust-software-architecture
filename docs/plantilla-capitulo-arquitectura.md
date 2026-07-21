# Plantilla de capítulo de arquitectura

Cada capítulo debe seguir la anatomía de RFC-0001 §14 y adaptar estas secciones
al estilo arquitectónico correspondiente. La plantilla no es una camisa de
fuerza; es una defensa contra capítulos que solo nombran una arquitectura sin
mostrar sus consecuencias.

Un capítulo puede cambiar el orden interno de algunas subsecciones si mejora la
claridad, pero no debe omitir concepto, problema, alternativas, modelo Rust,
invariantes, costos, modos de falla, ejercicios ni cierre editorial.

## Metadatos editoriales

Cada capítulo debe abrir con una tabla breve:

| Campo | Valor |
|-------|-------|
| Estado | `planned`, `draft`, `implemented`, `tested` o `benchmarked` |
| Issue | Link al issue de GitHub |
| PR | Link al PR cuando exista |
| Milestone | Milestone del capítulo |
| Módulo Rust | Ruta esperada en `src/` |
| Ejemplos | Rutas esperadas en `examples/` |
| Soluciones | Rutas esperadas en `examples/soluciones/` |
| Diagramas | Rutas esperadas en `diagrams/` o sección Mermaid |

`reviewed` y `published` quedan reservados para revisión humana explícita de
Joel. La IA no debe asignar esos estados durante trabajo autónomo.

## 1. Concepto

Qué es el estilo o técnica, qué problema promete resolver y qué no promete.

La explicación debe empezar en lenguaje humano. Antes de dibujar capas,
servicios o flechas, el capítulo debe poder contestar:

- qué tensión real aparece en el sistema;
- qué límite intenta proteger el estilo;
- qué dependencia busca permitir, invertir o prohibir;
- qué parte del motor de reservas se vuelve más clara;
- qué parte se vuelve más costosa.

Evita definiciones circulares como "arquitectura hexagonal es usar puertos y
adaptadores" sin explicar por qué esos puertos existen.

## 2. Problema

Qué dolor aparece en el motor de reservas educativo antes de aplicar la técnica.

Debe incluir un caso narrativo pequeño, por ejemplo:

- una regla de disponibilidad empieza a repetirse en varios lugares;
- confirmar una reserva mezcla dominio, base de datos y transporte;
- una consulta necesita optimizar lectura sin contaminar escritura;
- un evento se publica dos veces o en orden inesperado;
- un equipo necesita cambiar una parte sin coordinar todo el despliegue.

El problema debe sentirse inevitable dentro del caso común, no inventado para
justificar la arquitectura.

## 3. Alternativas

Qué otras decisiones eran posibles y por qué no se eligieron para este capítulo.

Compara al menos dos opciones:

- seguir con el estilo anterior y pagar su costo;
- introducir el estilo del capítulo;
- posponer la decisión;
- usar una variante más simple.

La comparación debe nombrar tradeoffs. No basta decir que una alternativa es
"menos escalable" o "más mantenible"; hay que explicar qué cambia para pruebas,
despliegue, lenguaje, consistencia, operación y coordinación.

## 4. Modelo Rust

Qué módulos, tipos, traits, eventos o puertos representan el límite.

El modelo Rust debe ser pequeño y verificable. Cada capítulo debe declarar:

- módulo principal en `src/`;
- tipos públicos mínimos;
- constructores que impidan estados inválidos cuando sea razonable;
- errores de dominio o aplicación;
- tests cercanos a las invariantes;
- ejemplos progresivos que puedan ejecutarse.

Usa `trait` cuando represente un contrato arquitectónico real, no como
decoración. Usa módulos para visibilidad y frontera. No agregues dependencias
externas no triviales ni `unsafe` sin autorización explícita de Joel.

## 5. Invariantes

Qué reglas deben mantenerse aunque cambie la implementación.

Cada capítulo debe listar invariantes en forma comprobable:

- una reserva confirmada ocupa inventario;
- una cotización expirada no puede confirmarse;
- un comando no debe mutar estado si falla validación;
- una proyección puede estar atrasada, pero no debe inventar hechos;
- un adaptador no debe filtrar detalles técnicos hacia el dominio.

Cuando sea posible, cada invariante debe tener al menos una prueba automatizada.

## 6. Costos

Qué complejidad agrega el estilo y cuándo no conviene usarlo.

Incluye costos de:

- lectura y aprendizaje;
- número de archivos y conceptos;
- pruebas y mocks;
- persistencia;
- versionado de contratos;
- observabilidad;
- despliegue;
- coordinación entre personas o equipos.

Un capítulo serio debe poder decir "no uses esto todavía" cuando el problema no
lo justifica.

## 7. Modos de falla

Qué se rompe cuando se exagera, se aplica tarde o se aplica sin contexto.

Ejemplos de fallas esperadas:

- monolito modular convertido en carpetas sin límites reales;
- arquitectura hexagonal convertida en exceso de interfaces;
- Clean Architecture convertida en capas ceremoniales;
- DDD convertido en vocabulario sofisticado sin reglas de negocio;
- CQRS aplicado antes de tener presión real de lectura;
- event sourcing usado sin entender reconstrucción, versionado o privacidad;
- eventos usados como llamadas remotas difíciles de rastrear;
- microservicios usados para esconder un monolito mal entendido.

## 8. Relación con otros cursos

Qué conceptos se importan de cursos canónicos sin reexplicarlos.

Cada capítulo debe declarar sus dependencias pedagógicas:

- `rust-system-design` para decisiones de sistema completo;
- `rust-distributed-systems` para coordinación, fallas y consistencia
  distribuida;
- `rust-database-internals` para persistencia, transacciones e índices;
- `rust-design-patterns` para patrones y principios;
- `rust-cloud` para despliegue, red, IAM y operación.

El capítulo puede enlazar esos cursos, pero no debe reemplazarlos.

## 9. Diagrama Mermaid

Diagrama versionado en Markdown o en `diagrams/`.

Cada capítulo debe tener al menos un diagrama Mermaid cuando el estilo implique
relaciones entre módulos, capas, eventos, servicios o contratos.

El diagrama debe:

- nombrar fronteras, no solo tecnologías;
- mostrar dirección de dependencias;
- evitar flechas ambiguas;
- mantenerse cerca del texto que lo explica;
- vivir en el capítulo o en `diagrams/NN-nombre.md` si crece demasiado.

## 10. Ejemplos progresivos

Básico, intermedio, avanzado y caso realista.

Estructura sugerida:

| Nivel | Propósito | Ruta sugerida |
|-------|-----------|---------------|
| Básico | Mostrar el concepto sin ruido | `examples/NN_basico.rs` |
| Intermedio | Introducir una segunda regla o dependencia | `examples/NN_intermedio.rs` |
| Avanzado | Mostrar tradeoffs reales | `examples/NN_avanzado.rs` |
| Realista | Conectar varias piezas del caso común | `examples/NN_realista.rs` |

Los ejemplos deben compilar y enseñar una progresión. Si un ejemplo necesita más
explicación que el capítulo, probablemente está intentando cubrir demasiado.

## 11. Ejercicios

Niveles 1 a 3 conforme a RFC-0001 §17.

Cada capítulo debe cerrar con ejercicios graduados:

- **Nivel 1:** identificar conceptos, completar código pequeño o explicar una
  decisión.
- **Nivel 2:** modificar el modelo, agregar una regla o escribir pruebas.
- **Nivel 3:** comparar alternativas, defender un diseño y registrar costos.

Las soluciones sugeridas deben vivir en `examples/soluciones/` o en una sección
claramente separada, según convenga al capítulo. Deben explicar por qué una
solución es aceptable, no solo mostrar código final.

## 12. Cierre editorial

Estado del capítulo, verificaciones y límites pendientes de revisión humana.

Cada capítulo debe cerrar con:

- estado editorial actual;
- verificaciones ejecutadas;
- decisiones registradas;
- límites pendientes;
- referencias a issue, PR y milestone;
- aclaración explícita de que `reviewed` y `published` requieren revisión
  humana.

## Checklist antes de abrir PR

- [ ] El capítulo explica el porqué antes del cómo.
- [ ] El problema nace del motor de reservas educativo.
- [ ] Hay alternativas comparadas con tradeoffs explícitos.
- [ ] El modelo Rust compila y evita estados inválidos cuando sea razonable.
- [ ] Las invariantes están nombradas y probadas cuando aplica.
- [ ] Los costos operativos y organizacionales aparecen en el texto.
- [ ] Los modos de falla no se ocultan.
- [ ] El diagrama Mermaid muestra límites y dirección de dependencias.
- [ ] Los ejemplos progresan de simple a realista.
- [ ] Los ejercicios tienen niveles 1 a 3.
- [ ] Las soluciones no sustituyen el intento del estudiante.
- [ ] El README, ROADMAP y checklist se actualizan cuando el avance lo amerite.
- [ ] El PR usa `Closes #N` además del texto en español.
- [ ] El PR está asignado a `jeresoftx`, asociado al milestone del issue y
      etiquetado.
- [ ] El PR declara si será fusionado en modo de revisión diferida.

## Verificaciones esperadas

Cuando aplique al cambio:

```bash
cargo fmt --check
git diff --check
cargo test --doc
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
```

Un capítulo de pura documentación también debe pasar las verificaciones
técnicas del crate, porque el repositorio completo debe conservarse saludable.
