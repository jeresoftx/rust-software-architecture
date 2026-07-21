use rust_software_architecture::event_driven_architecture::{
    bus::InMemoryEventBus,
    consumers::{EventConsumer, NotificationConsumer, ReservationAnalyticsConsumer},
    producers::ReservationProducer,
};

fn main() {
    let mut bus = InMemoryEventBus::default();
    let mut notifications = NotificationConsumer::default();
    let mut analytics = ReservationAnalyticsConsumer::default();

    let event =
        ReservationProducer::confirm_reservation("RSV-EDA-200", "OFR-EDA-2026", "CUS-84", &mut bus)
            .expect("evento publicado");

    InMemoryEventBus::dispatch(
        &event,
        [
            &mut notifications as &mut dyn EventConsumer,
            &mut analytics as &mut dyn EventConsumer,
        ],
    )
    .expect("fan-out completado");

    println!("Notificaciones enviadas: {}", notifications.sent_count());
    println!("Reservas medidas: {}", analytics.confirmed_count());
}
