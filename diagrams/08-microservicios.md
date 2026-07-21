# 08. Microservicios

```mermaid
flowchart LR
    Client["Cliente / caso de uso"] --> Contract["ConfirmReservationRequest\ncontrato v1"]
    Contract --> Reservations["ReservationService\nowner: reservations"]
    Reservations --> Payments["PaymentService\nowner: payments"]
    Reservations --> Inventory["InventoryService\nowner: inventory"]
    Payments --> Authorization["PaymentAuthorization"]
    Inventory --> Hold["InventoryHold"]
    Authorization --> Confirmation["ReservationConfirmation\ncontrato v1"]
    Hold --> Confirmation

    Catalog["DataOwnershipCatalog"] -. evita .-> Shared["tabla compartida como API"]
    Payments -. falla remota .-> Partial["falla parcial visible"]
```

El servicio de reservas no lee tablas internas de pagos ni inventario. Colabora
mediante contratos explícitos y trata una falla remota como falla parcial
visible. El catálogo de ownership enseña que compartir tablas entre servicios
rompe autonomía aunque el código esté separado.
