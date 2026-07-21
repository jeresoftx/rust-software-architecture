# 05. CQRS

```mermaid
flowchart LR
    Actor["Caso de uso o interfaz"] --> Command["ConfirmReservation"]
    Command --> WriteModel["ReservationWriteModel"]
    WriteModel --> Event["ReservationConfirmed"]
    Event --> Projection["ReservationSummaryProjection"]
    Projection --> ReadModel["ReservationSummary"]
    Query["FindConfirmedReservations"] --> ReadModel

    WriteModel -. protege .-> Invariants["Invariantes de escritura"]
    Projection -. reconoce .-> Lag["Retraso de proyección"]
```

El comando expresa una intención de cambio. El modelo de escritura valida la
intención, protege invariantes y produce un hecho observable. La proyección
convierte ese hecho en una vista de lectura. La consulta lee la vista sin volver
a ejecutar reglas de escritura ni mutar la proyección.
