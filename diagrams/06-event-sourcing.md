# 06. Event sourcing

```mermaid
flowchart LR
    Command["Comando\nRequest / Confirm / Cancel"] --> Decision["Agregado rehidratado"]
    Stream["ReservationEventStream"] --> Decision
    Decision --> Event["Nuevo evento aceptado"]
    Event --> Stream
    Stream --> Rehydrate["Rehidratación determinista"]
    Rehydrate --> State["Reservation\nestado actual"]
    Stream --> Audit["Auditoría\nhistoria completa"]

    Event -. incrementa .-> Version["Versión del stream"]
    Rehydrate -. valida .-> Invariants["Invariantes históricas"]
```

El stream guarda hechos aceptados en orden. El agregado se rehidrata desde esa
historia antes de decidir si un comando puede producir un nuevo evento. La
auditoría no es una tabla secundaria: es la misma historia que permite explicar
el estado actual.
