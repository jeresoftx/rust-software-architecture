# 04. Domain-Driven Design

| Campo | Valor |
|-------|-------|
| Estado | `draft` |
| Issue | [#19](https://github.com/jeresoftx/rust-software-architecture/issues/19) |
| PR | Pendiente |
| Milestone | `04. Domain-Driven Design` |
| Módulo Rust | `src/domain_driven_design.rs` |
| Ejemplos | Pendiente |
| Soluciones | Pendiente |
| Diagramas | Pendiente |

Domain-Driven Design no empieza con carpetas ni patrones. Empieza con una
pregunta incómoda: ¿el código habla el mismo idioma que las personas que
entienden el negocio?

En el motor de reservas educativo, las palabras `Oferta`, `Cotización`,
`Reserva`, `Inventario`, `Cancelación` y `Disponibilidad` ya existen en el
lenguaje del curso. DDD pide convertir ese lenguaje en límites, reglas e
invariantes, sin disfrazar decisiones de negocio como detalles técnicos.

## 1. Concepto

Domain-Driven Design organiza software alrededor del dominio: el conocimiento,
las reglas y el lenguaje que hacen que el sistema importe. Sus piezas centrales
para este capítulo son:

- **Lenguaje ubicuo:** vocabulario compartido entre código, documentación y
  conversación de negocio.
- **Value objects:** valores con significado propio, igualdad por contenido e
  invariantes locales.
- **Entidades:** objetos con identidad estable a través del tiempo.
- **Agregados:** frontera de consistencia donde una raíz protege cambios
  válidos.
- **Bounded contexts:** límites donde una palabra tiene un significado preciso.

DDD no promete que todo sea más simple. Promete que la complejidad inevitable
del negocio vive en un lugar visible, nombrado y discutible.

## 2. Problema

Después de Clean Architecture, el sistema ya separa entidad, caso de uso y
repositorio. El siguiente dolor aparece cuando el dominio crece:

- una reserva puede confirmarse, cancelarse o expirar;
- una cotización tiene precio, vigencia y condiciones;
- inventario puede reservarse, liberarse o quedar bloqueado;
- una palabra como "disponible" puede significar cosas distintas para ventas,
  operación o atención al cliente;
- una regla escrita en una pantalla puede contradecir la regla escrita en el
  caso de uso.

Si el código solo tiene tipos genéricos como `BookingService`,
`ReservationData` o `Status`, el lenguaje real queda fuera del modelo. La
arquitectura sigue limpia en capas, pero pobre en significado.

## 3. Alternativas

### Continuar con Clean Architecture

Puede bastar cuando el dominio es pequeño. El riesgo es que el caso de uso
conserve buena dirección de dependencias, pero el lenguaje de negocio siga
aplanado en estructuras sin intención.

### Modelo anémico con servicios

Permite avanzar rápido al inicio: datos por un lado, servicios por otro. Su
costo aparece cuando las reglas se duplican y nadie sabe qué objeto protege una
invariante.

### Domain-Driven Design táctico

Introduce value objects, entidades, agregados y repositorios con intención de
dominio. No requiere dividir todo el sistema en microservicios ni inventar un
mapa estratégico completo desde el primer día.

### Domain-Driven Design estratégico completo

Bounded contexts, context maps, relaciones entre equipos y lenguaje de negocio
profundo son valiosos cuando el sistema y la organización ya tienen suficiente
complejidad. Para este capítulo, se nombran y se conectan, pero el modelo Rust
se mantiene táctico y pequeño.

## 4. Modelo Rust esperado

El modelo mínimo debe representar:

- value objects como `ReservationId`, `OfferId`, `CustomerId` y `QuotedPrice`;
- una entidad o aggregate root `Reservation`;
- reglas de transición como confirmar y cancelar;
- un evento de dominio pequeño, por ejemplo `ReservationConfirmed`;
- errores de dominio explícitos para estados inválidos;
- un repositorio que guarde el agregado sin dictar sus reglas;
- pruebas que demuestren que el agregado protege invariantes.

El objetivo no es crear un framework DDD. El objetivo es que el lector vea qué
cambia cuando el lenguaje del dominio deja de ser texto alrededor del código y
se convierte en tipos, métodos y errores.

## 5. Invariantes

El capítulo debe proteger estas reglas:

- una reserva no existe sin identidad válida;
- una cotización no puede tener precio cero;
- una reserva confirmada debe conservar oferta, cliente y precio confirmado;
- una reserva cancelada no puede confirmarse otra vez sin una decisión explícita;
- el agregado `Reservation` decide sus transiciones, no un adaptador externo;
- los value objects rechazan estados inválidos al construirse;
- los eventos de dominio describen hechos que ya ocurrieron, no deseos.

Estas invariantes deben convertirse en pruebas durante la implementación del
modelo Rust mínimo.

## 6. Costos

DDD agrega costo real:

- más conversación antes de nombrar tipos;
- más presión por escribir lenguaje preciso;
- más objetos pequeños y constructores;
- más decisiones sobre frontera de agregado;
- riesgo de convertir cada sustantivo en tipo;
- riesgo de usar vocabulario elegante sin reglas reales;
- necesidad de revisar el modelo cuando cambia el aprendizaje del dominio.

Su beneficio principal es que las reglas importantes tienen dueño y nombre. Su
costo principal es que obliga a pensar en el negocio antes de esconderlo detrás
de servicios genéricos.

## 7. Modos de falla

DDD falla cuando:

- se usa como decoración de carpetas `domain`, `application` e
  `infrastructure`;
- los value objects no protegen ninguna invariante;
- los agregados son demasiado grandes y bloquean cambios independientes;
- los agregados son demasiado pequeños y no protegen consistencia;
- se inventa lenguaje sin hablar con quien entiende el negocio;
- todo se llama "contexto" sin definir una frontera real;
- se confunde DDD con microservicios.

## 8. Relación con otros cursos

Este capítulo se apoya en `rust-design-patterns` para encapsulación,
constructores y composición; en `rust-database-internals` para recordar que
persistir agregados tiene costos transaccionales; en `rust-system-design` para
ubicar bounded contexts dentro de un sistema mayor; y prepara discusiones de
`rust-distributed-systems` cuando una invariante cruza fronteras de contexto.

## 9. Diagrama Mermaid

Pendiente del issue de capítulo, diagrama y ejemplos.

## 10. Ejemplos progresivos

Pendientes del issue de capítulo, diagrama y ejemplos.

## 11. Ejercicios

Pendientes del issue de ejercicios, soluciones y costos.

## 12. Cierre editorial

Estado actual: `draft`.

Este capítulo todavía no está `reviewed` ni `published`. Requiere modelo Rust
mínimo, diagrama, ejemplos, ejercicios, soluciones, costos finales y revisión
humana explícita de Joel antes de avanzar de estado editorial.

### Decisiones registradas

- DDD se enseña después de Clean Architecture porque primero se aprende a
  dirigir dependencias y luego a enriquecer el lenguaje del núcleo.
- Este capítulo empieza con DDD táctico; el DDD estratégico se nombra sin
  convertirlo en el centro del primer modelo.
- El motor de reservas se usa como laboratorio de lenguaje: las palabras del
  dominio deben aparecer en código, pruebas y documentación.
- DDD no se presenta como sinónimo de microservicios.
