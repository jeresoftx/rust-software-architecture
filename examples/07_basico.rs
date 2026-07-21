use rust_software_architecture::event_driven_architecture::{
    bus::InMemoryEventBus, contracts::IntegrationEvent, producers::ReservationProducer,
};

fn main() {
    let mut bus = InMemoryEventBus::default();

    let event =
        ReservationProducer::confirm_reservation("RSV-EDA-100", "OFR-EDA-2026", "CUS-42", &mut bus)
            .expect("evento publicado");

    println!(
        "{} v{} para la reserva {} del cliente {}.",
        event.contract_name(),
        event.contract_version(),
        event.reservation_id(),
        event.customer_id()
    );
    println!("Eventos publicados: {}", bus.published_count());
}
