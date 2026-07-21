# 07. Arquitectura orientada a eventos

```mermaid
flowchart LR
    Producer["ReservationProducer"] --> Contract["ReservationConfirmed\ncontrato v1"]
    Contract --> Bus["InMemoryEventBus"]
    Bus --> Notifications["NotificationConsumer"]
    Bus --> Analytics["ReservationAnalyticsConsumer"]
    Bus --> Failure["FailingConsumer"]

    Contract -. nombre y versión .-> Version["Contrato estable"]
    Notifications -. evita duplicados .-> Processed["event_id procesado"]
    Analytics -. conteo independiente .-> Metrics["Métrica de reservas"]
    Failure -. no borra .-> Bus
```

El productor publica un hecho de integración con contrato explícito. El bus en
memoria conserva el evento publicado y lo entrega a consumidores que reaccionan
sin depender del productor. Si un consumidor falla, el evento no desaparece; si
el mismo evento llega dos veces, los consumidores idempotentes no duplican sus
efectos críticos.
