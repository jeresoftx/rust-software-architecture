# Introducción

Arquitectura de software no es dibujar cajas. Es decidir qué puede cambiar sin
romper lo demás, qué dependencia es legítima, dónde vive el lenguaje del negocio
y qué costo operativo acepta el sistema.

Este curso usa Rust como vehículo para hacer visibles esas decisiones. La
mayoría de los capítulos son conceptuales, pero cada idea debe aterrizar en
modelos pequeños, tests y ejemplos que permitan discutir invariantes en vez de
solo opiniones.

## Caso común

El caso común del curso será un motor de reservas educativo. Es lo bastante
pequeño para caber en ejemplos, pero lo bastante rico para mostrar:

- inventario;
- disponibilidad;
- cotización;
- confirmación;
- cancelación;
- eventos de dominio;
- consultas separadas de comandos;
- límites entre negocio e infraestructura.

No es el curso `rust-travel`: aquí el foco está en la arquitectura, no en
construir un producto completo de travel tech.

## Regla de lectura

El curso avanza de lo local a lo distribuido:

1. primero se ordena un sistema dentro de un solo despliegue;
2. después se separa dominio, aplicación e infraestructura;
3. luego se modela comportamiento de negocio;
4. finalmente se estudia cuándo conviene distribuir y qué se paga al hacerlo.

Para estudiar sin brincar el razonamiento, empieza por la
[ruta de lectura](./00-ruta-de-lectura.md), consulta el
[glosario base](./00-glosario.md) cada vez que un término cargue demasiado peso
y usa el [manifiesto del curso](./manifiesto-curso.md) como frontera editorial.
Cada capítulo debe leerse en este orden: concepto, problema, alternativas,
justificación, implementación, ejercicios y soluciones.
