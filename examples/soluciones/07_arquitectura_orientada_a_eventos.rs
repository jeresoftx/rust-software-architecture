use rust_software_architecture::event_driven_architecture::{
    bus::InMemoryEventBus,
    consumers::{EventConsumer, FailingConsumer, NotificationConsumer, ReservationAnalyticsConsumer},
    contracts::IntegrationEvent,
    producers::ReservationProducer,
    EventDrivenArchitectureError,
};

fn main() {
    let mut bus = InMemoryEventBus::default();
    let mut notifications = NotificationConsumer::default();
    let mut analytics = ReservationAnalyticsConsumer::default();
    let mut failing = FailingConsumer;

    let event = ReservationProducer::confirm_reservation(
        "RSV-EDA-SOL",
        "OFR-SOL-2026",
        "CUS-SOL",
        &mut bus,
    )
    .expect("evento publicado");

    InMemoryEventBus::dispatch(
        &event,
        [
            &mut notifications as &mut dyn EventConsumer,
            &mut analytics as &mut dyn EventConsumer,
        ],
    )
    .expect("consumidores principales procesados");

    notifications
        .handle(&event)
        .expect("evento duplicado ignorado");

    let failure = InMemoryEventBus::dispatch(&event, [&mut failing as &mut dyn EventConsumer]);

    if failure == Err(EventDrivenArchitectureError::ConsumerFailed) {
        println!("El consumidor fallido no borró el evento {}.", event.event_id());
    }

    println!(
        "{} v{} publicado para {}.",
        event.contract_name(),
        event.contract_version(),
        event.reservation_id()
    );
    println!("Eventos publicados: {}", bus.published_count());
    println!("Notificaciones enviadas: {}", notifications.sent_count());
    println!("Reservas medidas: {}", analytics.confirmed_count());
}
