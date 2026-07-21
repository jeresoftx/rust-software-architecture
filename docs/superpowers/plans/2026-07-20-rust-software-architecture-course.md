# Plan de trabajo: rust-software-architecture

> Estado: cierre editorial en progreso.
> Curso: Arquitectura de software en Rust.  
> RFC: RFC-0001 §10, §13, §14, §15, §17 y §20.

## Regla de ejecución

Antes de tocar código de curso, este plan debe vivir como milestones e issues
en GitHub. A partir de ese tablero, cada paso se trabaja con la regla:

```text
milestone + issue asignado → 1 commit → PR asignado → revisión humana o revisión diferida autorizada → merge a main
```

Cada issue debe estar asignado a `jeresoftx`, tener labels coherentes y
pertenecer al milestone del capítulo o fase correspondiente. Cada PR debe
conservar la misma trazabilidad: assignee, milestone, labels y `Closes #N`.

El GitHub Project asociado al repositorio debe contener todos los issues del
plan y su vista principal debe agrupar por `Milestone`. Si la vista se pierde o
GitHub la reinicia, se debe restaurar antes de comenzar un bloque autónomo.

## Milestones del plan

| Milestone | Alcance | Issues |
|-----------|---------|--------|
| 00. Gobernanza y planificación | Frontera, plantilla, glosario y flujo autónomo | 4 |
| 01. Monolito modular | Límites internos en un solo despliegue | 4 |
| 02. Arquitectura hexagonal | Puertos, adaptadores e infraestructura sustituible | 4 |
| 03. Clean Architecture | Dirección de dependencias y casos de uso | 4 |
| 04. Domain-Driven Design | Lenguaje ubicuo, agregados e invariantes | 4 |
| 05. CQRS | Separación de comandos y consultas | 4 |
| 06. Event sourcing | Estado reconstruido desde eventos | 4 |
| 07. Arquitectura orientada a eventos | Integración mediante eventos y contratos | 4 |
| 08. Microservicios | Distribución con costo organizacional y operativo | 4 |
| 09. Cierre editorial y publicación | Navegación, verificación y revisión candidata | 3 |

## Checklist

### 00. Gobernanza y planificación

- [x] Documentar manifiesto, frontera y caso común del curso.
- [x] Preparar plantilla de capítulo de arquitectura.
- [x] Documentar flujo autónomo y trazabilidad GitHub.
- [x] Verificar que el GitHub Project esté asociado al repo y agrupado por
      `Milestone`.
- [x] Completar glosario base y ruta de lectura.

### 01. Monolito modular

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para módulos y límites internos.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 02. Arquitectura hexagonal

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para puertos y adaptadores.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 03. Clean Architecture

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para casos de uso y dirección de dependencias.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 04. Domain-Driven Design

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para entidades, value objects y agregados.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 05. CQRS

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para comandos, consultas y proyecciones.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 06. Event sourcing

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para eventos, stream y rehidratación.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 07. Arquitectura orientada a eventos

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para productores, consumidores y contratos.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 08. Microservicios

- [x] Especificar concepto, problema, alternativas e invariantes.
- [x] Implementar modelo Rust mínimo para límites de servicio y comunicación.
- [x] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [x] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 09. Cierre editorial y publicación

- [x] Alinear README, ROADMAP, manifiesto y estados de capítulos.
- [x] Preparar navegación mdBook y verificación de enlaces.
- [ ] Documentar publicación candidata, riesgos y revisión humana necesaria.

## Siguiente paso natural

Documentar publicación candidata, riesgos y revisión humana necesaria sin
marcar capítulos como `reviewed` ni `published`.
