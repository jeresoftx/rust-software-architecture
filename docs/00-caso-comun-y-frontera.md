# Caso común y frontera del curso

El caso común de `rust-software-architecture` es un motor de reservas
educativo. No existe para construir un producto de viajes completo, sino para
darle al curso un dominio estable donde se puedan comparar decisiones
arquitectónicas sin cambiar de problema en cada capítulo.

## Por qué un motor de reservas

Un motor de reservas tiene suficiente tensión arquitectónica para enseñar:

- disponibilidad e inventario;
- cotizaciones con vigencia;
- confirmaciones que cambian estado;
- cancelaciones y compensaciones;
- reglas de negocio visibles;
- lecturas optimizadas;
- eventos de dominio;
- integración con infraestructura;
- límites entre módulos, dominios, casos de uso y servicios.

También es lo bastante pequeño para que cada capítulo pueda explicar una idea
sin esconderla detrás de un sistema enorme.

## Vocabulario del caso común

El curso usará estos términos de forma consistente:

- **Oferta:** opción reservable que puede tener reglas, precio e inventario.
- **Inventario:** capacidad disponible para aceptar reservas.
- **Cotización:** respuesta temporal que estima condiciones y precio.
- **Reserva:** intención confirmada que ocupa inventario bajo reglas de negocio.
- **Confirmación:** operación que convierte una cotización válida en reserva.
- **Cancelación:** operación que libera, compensa o registra una reserva que ya
  no debe mantenerse activa.
- **Disponibilidad:** lectura que responde qué se puede reservar bajo ciertas
  condiciones.
- **Evento de dominio:** hecho relevante como `ReservaConfirmada` o
  `ReservaCancelada`.

Los nombres concretos en código podrán escribirse en inglés cuando convenga por
idioma técnico de Rust, pero la explicación pedagógica debe conservar español
es-MX claro.

## Qué sí pertenece a este repositorio

Este repositorio cubre el diseño arquitectónico del caso común:

- cómo partir el sistema en módulos;
- dónde viven reglas e invariantes;
- cómo se dirigen dependencias;
- qué contratos aparecen entre dominio, aplicación e infraestructura;
- cómo cambia el mismo caso común bajo monolito modular, arquitectura hexagonal,
  Clean Architecture, Domain-Driven Design, CQRS, event sourcing, arquitectura
  orientada a eventos y microservicios;
- qué costos de operación, evolución y coordinación introduce cada estilo.

El código debe ser pequeño, verificable y didáctico. Si una pieza no ayuda a
entender una decisión arquitectónica, probablemente no pertenece aquí.

## Qué no pertenece a este repositorio

Este curso no debe convertirse en:

- una aplicación completa de travel tech;
- un backend productivo con usuarios reales, pagos, proveedores o facturación;
- un catálogo de frameworks;
- una colección de diagramas sin código verificable;
- una defensa de microservicios como destino inevitable;
- un sustituto de cursos especializados de sistemas distribuidos, bases de
  datos, cloud, patrones o diseño de sistemas.

Cuando una idea crezca más allá de la frontera arquitectónica, debe moverse o
referenciarse desde el repositorio adecuado.

## Relación con cursos vecinos

`rust-system-design` decide sistemas completos, tráfico, capacidad, APIs,
almacenamiento y escenarios de producto. Este curso toma una porción pequeña de
ese razonamiento y se concentra en la forma interna del software.

`rust-distributed-systems` estudia consenso, relojes, replicación, tolerancia a
fallas y coordinación distribuida. Este curso puede nombrar esos costos, pero no
los implementa como tema central.

`rust-database-internals` explica almacenamiento, índices, transacciones, MVCC,
planes de consulta y motores. Este curso usa persistencia como frontera
arquitectónica, no como objeto de estudio interno.

`rust-design-patterns` enseña patrones y principios reutilizables. Este curso
usa patrones cuando ayudan a sostener una arquitectura, pero no los enumera como
catálogo.

`rust-cloud` estudia plataformas, redes, IAM, servicios manejados, despliegue y
costos. Este curso prepara límites para poder desplegar con criterio, pero no
reemplaza la operación cloud.

`rust-travel` puede convertirse en un proyecto integrador de producto. Este
curso solo usa el dominio de reservas como laboratorio arquitectónico.

## Decisión canónica

El motor de reservas es un **laboratorio de arquitectura**, no el producto final.
Su valor está en permitir comparación: mismo dominio, distintos límites,
distintas dependencias, distintos costos.
