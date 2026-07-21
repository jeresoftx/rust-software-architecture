# ROADMAP

Estado de avance de `rust-software-architecture`, repositorio del camino
troncal de Jeresoft Academy para arquitectura de software en Rust.

No hay fechas límite: este es un proyecto de legado (RFC-0001 §1). Este archivo
orienta el avance, pero no convierte el curso en una carrera por terminar.

## Estado actual

El repositorio acaba de entrar en desarrollo. La estructura inicial ya declara
la frontera del curso, la ruta de lectura, el glosario base, la plantilla de
capítulo, el flujo de trabajo con GitHub, el modo autónomo con revisión diferida
y el plan maestro en forma de checklist.

El capítulo `01. Monolito modular` está en estado `draft`: ya cuenta con
especificación conceptual, modelo Rust mínimo, diagrama Mermaid y ejemplos
progresivos, ejercicios, solución sugerida y análisis de costos. Todavía
requiere revisión humana antes de subir de estado editorial.

El capítulo `02. Arquitectura hexagonal` está en estado `draft`: ya cuenta con
especificación conceptual de puertos, adaptadores, problema, alternativas e
invariantes, modelo Rust mínimo, diagrama Mermaid, ejemplos progresivos,
ejercicios, solución sugerida y análisis de costos. Todavía requiere revisión
humana antes de subir de estado editorial.

El capítulo `03. Clean Architecture` está en estado `draft`: ya cuenta con
especificación conceptual de entidades, casos de uso, dirección de dependencias,
problema, alternativas e invariantes, además de un modelo Rust mínimo con
pruebas para entidad, caso de uso y repositorio, diagrama Mermaid y ejemplos
progresivos, ejercicios, solución sugerida y análisis de costos. Todavía
requiere revisión humana antes de subir de estado editorial.

El capítulo `04. Domain-Driven Design` está en estado `draft`: ya cuenta con
especificación conceptual de lenguaje ubicuo, value objects, entidades,
agregados, bounded contexts, problema, alternativas, invariantes, costos y modos
de falla, además de un modelo Rust mínimo con pruebas para value objects,
agregado, eventos de dominio y repositorio, diagrama Mermaid y ejemplos
progresivos, ejercicios, solución sugerida y análisis de costos. Todavía
requiere revisión humana antes de subir de estado editorial.

El capítulo `05. CQRS` está en estado `draft`: ya cuenta con especificación
conceptual de comandos, consultas, proyecciones, modelos de lectura, problema,
alternativas, invariantes, costos y modos de falla, además de un modelo Rust
mínimo con pruebas para comando, evento, proyección y consulta, diagrama Mermaid
y ejemplos progresivos, ejercicios, solución sugerida y análisis de costos.
Todavía requiere revisión humana antes de subir de estado editorial.

El capítulo `06. Event sourcing` está en estado `draft`: ya cuenta con
especificación conceptual de eventos, streams, rehidratación, auditoría,
alternativas, invariantes, costos y modos de falla, además de un modelo Rust
mínimo con pruebas para stream append-only, rehidratación determinista, rechazo
de historia inválida y auditoría, diagrama Mermaid y ejemplos progresivos.
También cuenta con ejercicios, solución sugerida y análisis de costos. Todavía
requiere revisión humana antes de subir de estado editorial.

El capítulo `07. Arquitectura orientada a eventos` está en estado `draft`: ya
cuenta con especificación conceptual de productores, consumidores, contratos,
integración, alternativas, invariantes, costos y modos de falla, además de un
modelo Rust mínimo con pruebas para contrato estable, publicación, fan-out,
idempotencia y aislamiento de fallas. También cuenta con diagrama Mermaid y
ejemplos progresivos, ejercicios, solución sugerida y análisis de costos.
Todavía requiere revisión humana antes de subir de estado editorial.

El capítulo `08. Microservicios` está en estado `draft`: ya cuenta con
especificación conceptual de límites de servicio, datos propios, contratos,
operación, alternativas, invariantes, costos y modos de falla, además de un
modelo Rust mínimo con pruebas para contratos explícitos, ownership de datos y
falla remota visible. También cuenta con diagrama Mermaid y ejemplos
progresivos, ejercicios, solución sugerida y análisis de costos. Todavía
requiere revisión humana antes de subir de estado editorial.

El plan de trabajo debe vivir en GitHub como milestones e issues antes de tocar
código de curso. Cada paso queda asignado a `jeresoftx`, asociado al milestone
correspondiente y etiquetado para mantener la regla del repositorio: un issue,
un commit y un PR.

El GitHub Project del curso vive en
`https://github.com/users/jeresoftx/projects/9`. Debe permanecer asociado al
repositorio, contener todos los issues accionables y usar una vista principal
agrupada por `Milestone`, porque el avance se revisa por fase o capítulo.

Ningún capítulo está marcado como `reviewed` ni `published`, porque la revisión
humana de Joel sigue siendo obligatoria según RFC-0001 §20.

## Progresión del Semestre 5

El curso abre el Semestre 5 con arquitectura antes de pasar a plataformas cloud.
La progresión esperada es:

1. **Límites internos sostenibles:** monolito modular, arquitectura hexagonal y
   Clean Architecture.
2. **Lenguaje y comportamiento de negocio:** Domain-Driven Design, CQRS y event
   sourcing.
3. **Distribución organizacional y operativa:** arquitectura orientada a eventos
   y microservicios.
4. **Puente hacia Cloud:** usar estos límites para desplegar, operar y escalar
   sistemas con decisiones explícitas.

## Capítulos

| # | Capítulo | Estado |
|---|----------|--------|
| 01 | Monolito modular | draft |
| 02 | Arquitectura hexagonal | draft |
| 03 | Clean Architecture | draft |
| 04 | Domain-Driven Design | draft |
| 05 | CQRS | draft |
| 06 | Event sourcing | draft |
| 07 | Arquitectura orientada a eventos | draft |
| 08 | Microservicios | draft |

## Alineación RFC-0001

- Este repositorio sigue la plantilla de repositorio de RFC-0001 §15.
- Cada capítulo debe cumplir la anatomía de RFC-0001 §14.
- Cada ejercicio debe seguir los niveles de RFC-0001 §17.
- El uso de IA se rige por RFC-0001 §20: la IA acelera, el criterio humano
  decide.

## Fuera de alcance por ahora

- Presentar microservicios como destino inevitable.
- Repetir diagramas genéricos sin código verificable.
- Reemplazar discusión arquitectónica por marcas, frameworks o modas.
- Usar dependencias externas para esconder el concepto.
- Publicar capítulos parciales como si estuvieran completos.

## Siguiente paso natural

Cerrar el milestone `08. Microservicios` si todos sus issues quedaron cerrados
y continuar con `09. Cierre editorial y publicación`.
