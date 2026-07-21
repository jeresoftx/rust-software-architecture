# Plan de trabajo: rust-software-architecture

> Estado: checklist operativo inicial.  
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

- [ ] Documentar manifiesto, frontera y caso común del curso.
- [ ] Preparar plantilla de capítulo de arquitectura.
- [ ] Documentar flujo autónomo y trazabilidad GitHub.
- [ ] Completar glosario base y ruta de lectura.

### 01. Monolito modular

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para módulos y límites internos.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 02. Arquitectura hexagonal

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para puertos y adaptadores.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 03. Clean Architecture

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para casos de uso y dirección de dependencias.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 04. Domain-Driven Design

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para entidades, value objects y agregados.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 05. CQRS

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para comandos, consultas y proyecciones.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 06. Event sourcing

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para eventos, stream y rehidratación.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 07. Arquitectura orientada a eventos

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para productores, consumidores y contratos.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 08. Microservicios

- [ ] Especificar concepto, problema, alternativas e invariantes.
- [ ] Implementar modelo Rust mínimo para límites de servicio y comunicación.
- [ ] Escribir capítulo, diagrama Mermaid y ejemplos progresivos.
- [ ] Agregar ejercicios, soluciones y benchmark educativo o nota de no aplicación.

### 09. Cierre editorial y publicación

- [ ] Alinear README, ROADMAP, manifiesto y estados de capítulos.
- [ ] Preparar navegación mdBook y verificación de enlaces.
- [ ] Documentar publicación candidata, riesgos y revisión humana necesaria.

## Siguiente paso natural

Crear los milestones e issues de GitHub con este checklist, todos asignados a
`jeresoftx`, con labels de tipo, capítulo o fase, y estado. Después, comenzar
por el primer issue de gobernanza.
