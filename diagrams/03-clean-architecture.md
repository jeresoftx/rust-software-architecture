# 03. Clean Architecture

```mermaid
flowchart TB
    INPUT["Entrada externa"]
    REQUEST["ConfirmReservationRequest"]

    subgraph "Application"
        UC["ConfirmReservation"]
        REPO["ReservationRepository"]
    end

    subgraph "Domain"
        ENTITY["Reservation"]
        IDS["ReservationId / OfferId / CustomerId"]
    end

    subgraph "Adapters"
        MEM["InMemoryReservationRepository"]
        FAIL["FailingReservationRepository"]
    end

    INPUT --> REQUEST
    REQUEST --> UC
    UC --> IDS
    UC --> ENTITY
    UC --> REPO
    MEM -. implementa .-> REPO
    FAIL -. implementa .-> REPO
```

La dirección importante es hacia las reglas estables. El caso de uso transforma
datos primitivos en tipos del dominio, la entidad conserva sus invariantes y los
adaptadores implementan el contrato de persistencia desde afuera.
