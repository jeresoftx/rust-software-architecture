# 04. Domain-Driven Design

```mermaid
flowchart TB
    LANGUAGE["Lenguaje ubicuo"]

    subgraph "Bounded context: Reservas"
        VO["Value objects\nReservationId / OfferId / CustomerId / Money"]
        AGG["Aggregate root\nReservation"]
        EVT["DomainEvent\nReservationConfirmed / ReservationCancelled"]
        REPO["ReservationRepository"]
    end

    MEM["InMemoryReservationRepository"]

    LANGUAGE --> VO
    VO --> AGG
    AGG --> EVT
    REPO --> AGG
    MEM -. implementa .-> REPO
```

El agregado `Reservation` es la frontera de consistencia. Los value objects
protegen lenguaje local, los eventos nombran hechos ocurridos y el repositorio
guarda el agregado sin decidir sus transiciones.
