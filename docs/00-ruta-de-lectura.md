# Ruta de lectura

Este curso se lee como un libro de ingeniería: cada capítulo explica una forma
de organizar software, pero la comparación entre capítulos es parte del
aprendizaje. La pregunta constante no es "qué arquitectura está de moda", sino
"qué problema resuelve, qué costo introduce y qué límite protege".

## Antes de los capítulos

Lee esta sección de inicio antes de entrar a los estilos arquitectónicos:

1. **Introducción:** ubica el caso común y la progresión general.
2. **Manifiesto del curso:** declara el tono, la frontera y las promesas que no
   se deben romper.
3. **Glosario base:** normaliza el lenguaje para discutir límites, contratos y
   tradeoffs sin ambigüedad.
4. **Plantilla de capítulo:** muestra la forma esperada de cada entrega.
5. **Flujo autónomo:** documenta cómo se trabaja con issues, PRs, milestones y
   revisión diferida.

Antes de iniciar un bloque autónomo, confirma que el GitHub Project del curso
esté asociado al repositorio, contenga todos los issues accionables y conserve
su vista principal agrupada por `Milestone`.

## Bloque 1: límites internos

Empieza por los límites que todavía caben dentro de un solo despliegue:

1. **Monolito modular:** aprende a separar responsabilidades sin distribuir el
   sistema.
2. **Arquitectura hexagonal:** protege el dominio mediante puertos y
   adaptadores.
3. **Clean Architecture:** organiza dependencias para que los casos de uso no
   dependan de detalles externos.

Este bloque debe dejar una intuición clara: distribuir no arregla un sistema
que todavía no sabe dónde están sus límites.

## Bloque 2: negocio y comportamiento

Después de ordenar las dependencias internas, estudia cómo el negocio cambia la
forma del software:

1. **Domain-Driven Design:** modela lenguaje, invariantes y límites de negocio.
2. **CQRS:** separa intenciones de escritura y necesidades de lectura cuando el
   modelo único empieza a estorbar.
3. **Event sourcing:** representa el estado como consecuencia de eventos cuando
   la historia importa.

Este bloque exige leer despacio: muchas decisiones parecen técnicas, pero en
realidad son decisiones sobre lenguaje, auditoría, consistencia y operación.

## Bloque 3: distribución organizacional

Al final aparecen los estilos que cruzan límites de proceso, equipo y operación:

1. **Arquitectura orientada a eventos:** coordina cambios mediante hechos
   publicados, no mediante llamadas directas en cadena.
2. **Microservicios:** separa despliegues cuando existe una razón de negocio,
   operativa u organizacional suficiente.

Este bloque debe leerse con escepticismo sano. La distribución puede dar
autonomía, pero también agrega fallas parciales, observabilidad, versionado,
costos y trabajo operacional.

## Forma recomendada por capítulo

En cada capítulo:

1. Lee el concepto y el problema antes del código.
2. Identifica el límite arquitectónico que el capítulo intenta proteger.
3. Compara el estilo con al menos una alternativa ya estudiada.
4. Ejecuta tests y ejemplos antes de resolver ejercicios.
5. Revisa las soluciones solo después de intentar una propuesta propia.
6. Registra dudas o correcciones como issues nuevos, no como cambios invisibles.

## Cierre del curso

Al terminar los ocho capítulos, el lector debe poder explicar:

- qué cambia entre organizar módulos, capas, casos de uso, dominios, eventos y
  servicios;
- cuándo un límite mejora el sistema y cuándo solo añade ceremonia;
- qué dependencias conviene permitir, invertir o prohibir;
- qué lectura operacional tiene cada arquitectura;
- cómo defender una decisión sin venderla como receta universal.
