use rust_software_architecture::event_driven_architecture::{
    bus::InMemoryEventBus,
    consumers::{EventConsumer, FailingConsumer, NotificationConsumer},
    producers::ReservationProducer,
    EventDrivenArchitectureError,
};

fn main() {
    let mut bus = InMemoryEventBus::default();
    let mut notifications = NotificationConsumer::default();
    let mut failing = FailingConsumer;

    let event = ReservationProducer::confirm_reservation(
        "RSV-EDA-300",
        "OFR-EDA-2026",
        "CUS-126",
        &mut bus,
    )
    .expect("evento publicado");

    let failure = InMemoryEventBus::dispatch(&event, [&mut failing as &mut dyn EventConsumer]);

    match failure {
        Err(EventDrivenArchitectureError::ConsumerFailed) => {
            println!("Un consumidor falló, pero el evento sigue publicado.");
        }
        Ok(()) => println!("El consumidor de prueba no debió completar."),
        Err(error) => println!("Error inesperado: {error:?}"),
    }

    notifications.handle(&event).expect("notificación enviada");
    notifications
        .handle(&event)
        .expect("evento duplicado ignorado");

    println!("Eventos publicados: {}", bus.published_count());
    println!("Notificaciones enviadas: {}", notifications.sent_count());
}
