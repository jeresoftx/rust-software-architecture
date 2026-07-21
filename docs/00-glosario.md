# Glosario base

## Arquitectura

Conjunto de decisiones difíciles de cambiar que organizan un sistema: límites,
dependencias, contratos, despliegue, datos, operación y evolución.

## Arquitectura limpia

Familia de estilos que ordena dependencias hacia reglas de negocio estables. Su
idea central es que los casos de uso no deben depender de frameworks,
almacenamiento, transporte ni detalles de interfaz.

## Arquitectura hexagonal

Estilo que expresa el sistema como un núcleo de aplicación rodeado por puertos
y adaptadores. Los puertos declaran necesidades o capacidades; los adaptadores
conectan esas declaraciones con detalles externos.

## Arquitectura orientada a eventos

Estilo donde partes del sistema coordinan cambios mediante eventos publicados.
Reduce ciertas dependencias directas, pero exige cuidar orden, duplicados,
versionado, observabilidad y consistencia eventual.

## Límite

Separación explícita entre responsabilidades. Un límite sano permite cambiar
una parte del sistema sin obligar a reescribir todo lo demás.

## Módulo

Unidad de organización con una razón de cambio reconocible. En Rust puede
expresarse con `mod`, tipos públicos mínimos, pruebas cercanas y reglas claras
de visibilidad.

## Capa

Agrupación por responsabilidad técnica o conceptual. Una capa puede ayudar a
dirigir dependencias, pero se vuelve peligrosa cuando obliga al dominio a
hablar en términos de infraestructura.

## Contrato

Acuerdo entre dos partes del sistema. Puede ser una interfaz Rust, un evento, un
esquema de datos, una API o una expectativa operacional.

## Puerto

Contrato que expresa lo que el núcleo necesita del exterior o lo que permite que
el exterior le solicite. Un puerto debe nombrarse desde la intención del sistema,
no desde la tecnología concreta.

## Adaptador

Implementación que conecta un puerto con una tecnología o frontera específica:
base de datos, API HTTP, cola, CLI, archivo, proveedor externo o interfaz de
usuario.

## Dominio

Lenguaje, reglas e invariantes del negocio. En este curso, el dominio común es
un motor de reservas educativo.

## Invariante

Regla que debe permanecer verdadera para que el sistema sea correcto. Una buena
arquitectura facilita ubicar, probar y proteger invariantes.

## Entidad

Objeto de dominio con identidad estable a través del tiempo. Su importancia no
está solo en sus atributos, sino en las reglas que preserva durante su ciclo de
vida.

## Value object

Objeto de dominio definido por sus valores y reglas, no por una identidad
propia. Debe ser pequeño, explícito y difícil de construir en estado inválido.

## Agregado

Conjunto de entidades y value objects que se modifica como unidad de
consistencia. Su frontera protege invariantes durante una operación de negocio.

## Bounded context

Frontera lingüística y conceptual donde un modelo tiene significado preciso. Un
término puede significar cosas distintas en contextos distintos sin que eso sea
un error.

## Caso de uso

Orquestación de una intención del usuario o del sistema. Un caso de uso no debe
depender accidentalmente de detalles de infraestructura.

## Comando

Solicitud que expresa intención de cambiar el estado del sistema. Puede fallar
por reglas de negocio, permisos, concurrencia o indisponibilidad de una
dependencia.

## Consulta

Solicitud para leer información sin cambiar el estado observable del sistema.
En CQRS, una consulta puede usar modelos especializados para responder mejor a
las necesidades de lectura.

## Proyección

Modelo de lectura construido a partir de datos fuente o eventos. Optimiza una
pregunta concreta y acepta costos de sincronización, reconstrucción y
consistencia.

## Evento de dominio

Hecho relevante que ya ocurrió dentro del dominio. Se nombra en pasado y debe
representar lenguaje del negocio, no detalles técnicos de implementación.

## Evento de integración

Hecho publicado para comunicar cambios a otros sistemas o contextos. Suele
necesitar versionado, compatibilidad y reglas explícitas de entrega.

## Stream

Secuencia ordenada de eventos relacionados por una clave o agregado. En event
sourcing, un stream permite reconstruir estado y razonar sobre historia.

## Infraestructura

Detalles técnicos que permiten ejecutar el sistema: almacenamiento, transporte,
colas, proveedores externos, archivos, red y plataforma.

## Monolito modular

Sistema desplegado como una unidad, pero organizado internamente por límites
explícitos. No es ausencia de arquitectura: exige contratos internos, visibilidad
controlada y disciplina de dependencias.

## Microservicio

Servicio desplegable de manera independiente y dueño de una frontera clara. No
es una clase remota ni una forma automática de escalar: implica operación,
observabilidad, contratos y fallas parciales.

## Consistencia eventual

Modelo donde distintas lecturas pueden observar el cambio en momentos
diferentes, pero el sistema converge si no hay nuevas escrituras y los procesos
pendientes terminan correctamente.

## Despliegue

Forma en que una parte del sistema se empaqueta, libera y opera. Separar código
no siempre implica separar despliegues; separar despliegues sí aumenta las
responsabilidades operativas.

## Observabilidad

Capacidad de entender el comportamiento interno de un sistema a partir de sus
señales externas: logs, métricas, trazas, eventos y síntomas visibles para quien
opera.

## Acoplamiento

Grado en que una parte necesita conocer detalles de otra para funcionar. No todo
acoplamiento es malo; el problema es el acoplamiento accidental.

## Cohesión

Grado en que las piezas de un módulo pertenecen a la misma razón de cambio.

## Tradeoff

Intercambio consciente entre beneficios y costos. Una arquitectura seria no
vende ventajas sin nombrar lo que sacrifica.
