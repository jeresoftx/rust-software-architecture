# Diagrama: Arquitectura hexagonal

```mermaid
flowchart LR
    CLI["Adaptador de entrada\nCLI / HTTP / job"]

    subgraph "Núcleo"
        UC["Caso de uso\nConfirmBooking"]
        D["Dominio\nReservation"]
        P["Puerto de salida\nReservationStore"]
    end

    MEM["Adaptador de salida\nInMemoryReservationStore"]
    FAIL["Adaptador de salida\nFailingReservationStore"]

    CLI --> UC
    UC --> D
    UC --> P
    MEM -. implementa .-> P
    FAIL -. implementa .-> P
```

La dirección de dependencia apunta hacia el núcleo. El caso de uso conoce el
puerto `ReservationStore`, no el adaptador concreto. Los adaptadores pueden
cambiar sin reescribir la regla de confirmación.
