# Glosario base

## Arquitectura

Conjunto de decisiones difíciles de cambiar que organizan un sistema: límites,
dependencias, contratos, despliegue, datos, operación y evolución.

## Límite

Separación explícita entre responsabilidades. Un límite sano permite cambiar
una parte del sistema sin obligar a reescribir todo lo demás.

## Contrato

Acuerdo entre dos partes del sistema. Puede ser una interfaz Rust, un evento, un
esquema de datos, una API o una expectativa operacional.

## Dominio

Lenguaje, reglas e invariantes del negocio. En este curso, el dominio común es
un motor de reservas educativo.

## Caso de uso

Orquestación de una intención del usuario o del sistema. Un caso de uso no debe
depender accidentalmente de detalles de infraestructura.

## Infraestructura

Detalles técnicos que permiten ejecutar el sistema: almacenamiento, transporte,
colas, proveedores externos, archivos, red y plataforma.

## Acoplamiento

Grado en que una parte necesita conocer detalles de otra para funcionar. No todo
acoplamiento es malo; el problema es el acoplamiento accidental.

## Cohesión

Grado en que las piezas de un módulo pertenecen a la misma razón de cambio.

## Tradeoff

Intercambio consciente entre beneficios y costos. Una arquitectura seria no
vende ventajas sin nombrar lo que sacrifica.
