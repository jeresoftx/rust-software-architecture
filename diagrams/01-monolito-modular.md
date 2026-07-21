# Diagrama: Monolito modular

```mermaid
flowchart LR
    subgraph "Un solo despliegue"
        subgraph "booking"
            B["BookingService"]
            R["Reservation"]
        end

        subgraph "pricing"
            P["PricingService"]
            Q["Quote"]
        end

        subgraph "inventory"
            I["Inventory"]
            C["Capacity"]
        end
    end

    P --> Q
    B --> Q
    B --> I
    I --> C
    B --> R

    classDef module fill:#f6f8fa,stroke:#57606a,color:#24292f;
    class B,R,P,Q,I,C module;
```

El diagrama muestra un solo despliegue con tres módulos internos. La frontera no
es una red ni un proceso separado: es una regla de dependencia y visibilidad.
`booking` puede pedir una reserva a `inventory` mediante un contrato interno,
pero no modifica sus campos privados.
