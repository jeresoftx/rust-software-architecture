# 08. Microservicios

| Campo | Valor |
|-------|-------|
| Estado | `draft` |
| Issue | [#35](https://github.com/jeresoftx/rust-software-architecture/issues/35) |
| PR | Pendiente |
| Milestone | `08. Microservicios` |
| Módulo Rust | `src/microservices.rs` |
| Ejemplos | Pendiente |
| Soluciones | Pendiente |
| Diagramas | Pendiente |

Microservicios significa partir un sistema en servicios pequeños, autónomos y
desplegables de forma independiente alrededor de límites de negocio. No es una
versión "más profesional" del monolito ni el destino natural de todo sistema.
Es una decisión cara que solo conviene cuando el costo de coordinación interna
supera el costo de operar una red de servicios.

En el motor de reservas educativo, separar reservas, pagos, notificaciones e
inventario puede permitir equipos y despliegues independientes. También puede
crear latencia, fallas parciales, duplicación de contratos, consistencia
eventual y más trabajo operativo. Este capítulo enseña a decidir esa frontera
sin romantizar la distribución.

## 1. Concepto

Un microservicio es una unidad de software con responsabilidad de negocio
delimitada, contrato explícito y ciclo de despliegue propio. Sus piezas
principales son:

- **límite de servicio:** qué problema resuelve y qué decisiones le pertenecen;
- **datos propios:** estado que el servicio gobierna sin compartir tablas como
  si fueran API;
- **contrato:** interfaz estable para colaborar con otros servicios;
- **comunicación:** llamada síncrona, evento, cola o integración por batch;
- **operación:** despliegue, monitoreo, alertas, trazas, reintentos y soporte;
- **propiedad:** equipo o responsable que entiende el servicio y sus costos.

La palabra "micro" no habla de líneas de código. Habla de autonomía real. Un
servicio pequeño que comparte base de datos, despliegue y decisiones con todos
los demás no es autónomo; solo es un módulo distribuido con más red de por
medio.

## 2. Problema

Después de arquitectura orientada a eventos, el curso ya entiende integración
por contratos. El siguiente dolor aparece cuando los límites internos empiezan
a chocar con necesidades de evolución independiente:

- reservas cambia más rápido que inventario;
- pagos requiere controles, auditoría y despliegues con más cuidado;
- notificaciones puede escalar por volumen sin escalar todo el sistema;
- analítica necesita consumir hechos sin bloquear la operación principal;
- equipos distintos pueden necesitar ownership claro.

Si todo vive dentro del mismo despliegue, coordinar cambios puede volverse
lento. Pero distribuir demasiado pronto cambia un problema de diseño interno
por muchos problemas de red, operación y consistencia.

Microservicios intenta comprar autonomía con distribución. El precio se paga en
fallas parciales, contratos, observabilidad, datos duplicados y coordinación
entre equipos.

## 3. Alternativas

### Monolito modular

Debe ser la opción base cuando el equipo aún aprende el dominio o cuando el
producto necesita cambiar rápido sin operación distribuida. Permite límites
claros sin pagar red, despliegues múltiples ni consistencia eventual.

### Arquitectura hexagonal por módulos

Permite aislar dominio e infraestructura sin separar despliegues. Es útil
cuando se quiere preparar fronteras sustituibles antes de decidir si conviene
distribuir.

### Servicios internos grandes

Pueden funcionar cuando una frontera necesita despliegue propio, pero todavía
no justifica partirse en servicios pequeños. Reducen la cantidad de piezas
operativas.

### Microservicios

Ganan cuando hay límites de negocio estables, ownership claro, necesidad de
despliegue independiente y capacidad operativa suficiente. Pierden cuando se
usan para esconder un dominio mal entendido o para perseguir una moda.

## 4. Modelo Rust esperado

El modelo mínimo debe representar:

- límites de servicio explícitos para reservas, pagos e inventario;
- contratos de solicitud y respuesta entre servicios;
- ownership de datos por servicio;
- una llamada síncrona con error de servicio remoto;
- una decisión que no pueda depender de tablas compartidas;
- pruebas que demuestren frontera, contrato, falla parcial y autonomía básica.

El objetivo no es crear un framework HTTP. El objetivo es que el lector vea que
microservicios agrega una frontera operativa real: cada servicio decide sobre
sus datos y solo colabora mediante contratos explícitos.

## 5. Invariantes

El capítulo debe proteger estas reglas:

- un servicio gobierna sus propios datos;
- otros servicios no leen ni escriben sus tablas internas;
- cada colaboración cruza un contrato explícito;
- una falla remota debe ser visible como falla parcial;
- separar despliegues no elimina la necesidad de modelar bien el dominio;
- duplicar datos de lectura requiere trazabilidad y sincronización explícitas;
- microservicios no se presentan como destino inevitable;
- si no hay ownership operativo, no hay autonomía real.

Estas invariantes evitan confundir "muchos repositorios" con arquitectura. La
arquitectura aparece cuando las fronteras reducen coordinación sin destruir la
capacidad de entender y operar el sistema.

## 6. Costos

Microservicios agregan costo:

- más despliegues, configuración y monitoreo;
- fallas parciales y latencia de red;
- contratos versionados entre servicios;
- consistencia eventual y procesos de compensación;
- duplicación de datos para lectura;
- pruebas de integración y ambientes más complejos;
- necesidad de ownership claro por equipo o responsable;
- observabilidad distribuida para explicar incidentes.

Su beneficio principal es la autonomía evolutiva cuando el dominio y la
organización ya justifican esa inversión. Su costo principal es que el sistema
deja de fallar como un proceso único y empieza a fallar como una red viva.

## 7. Modos de falla

Microservicios fallan cuando:

- se separan por capas técnicas en vez de límites de negocio;
- comparten una base de datos como contrato oculto;
- cada llamada síncrona crea una cadena frágil de disponibilidad;
- nadie versiona contratos;
- no hay trazas, métricas ni logs correlacionables;
- no existe responsable operativo por servicio;
- se distribuye el sistema antes de entender el monolito;
- se usa la palabra "microservicio" para justificar complejidad innecesaria.

## 8. Relación con otros cursos

Este capítulo se apoya en `rust-system-design` para capacidad, APIs y
escenarios de producto; en `rust-distributed-systems` para fallas parciales,
timeouts y coordinación; en `rust-database-internals` para ownership de datos;
y en `rust-cloud` para despliegue, redes, observabilidad y costos.

También conversa con `rust-docker`, porque los laboratorios de servicios,
brokers y bases de datos se montarán ahí cuando el curso necesite
infraestructura. En este repositorio, el modelo se mantiene en memoria para que
la frontera conceptual sea visible antes de introducir plataformas.

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

- Microservicios se enseñan después de arquitectura orientada a eventos porque
  primero se necesita entender integración por contratos.
- Este capítulo evita presentar microservicios como destino inevitable.
- La autonomía de un microservicio requiere ownership de datos, contrato,
  despliegue y operación; no basta con partir código.
- El costo central de microservicios es operar fallas parciales y consistencia
  eventual con trazabilidad suficiente.
