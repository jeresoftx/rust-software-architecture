use rust_software_architecture::event_sourcing::{
    commands::{CancelReservation, ConfirmReservation, RequestReservation},
    events::ReservationEvent,
    stream::ReservationEventStream,
    Reservation,
};

fn audit_labels(events: &[ReservationEvent]) -> Vec<String> {
    events
        .iter()
        .map(|event| {
            format!(
                "v{}:{:?}:{}",
                event.version(),
                event.kind(),
                event.reservation_id()
            )
        })
        .collect()
}

fn main() {
    let mut stream = ReservationEventStream::default();

    RequestReservation::new("RSV-ES-SOL", "OFR-SOL-2026", "CUS-SOL")
        .execute(&mut stream)
        .expect("reserva solicitada");
    ConfirmReservation::new("RSV-ES-SOL")
        .execute(&mut stream)
        .expect("reserva confirmada");
    CancelReservation::new("RSV-ES-SOL")
        .execute(&mut stream)
        .expect("reserva cancelada");

    let reservation = Reservation::rehydrate(stream.events()).expect("historia válida");
    let audit = audit_labels(stream.events());

    println!(
        "{} terminó como {:?} después de {} eventos.",
        reservation.id(),
        reservation.status(),
        audit.len()
    );

    for label in audit {
        println!("{label}");
    }
}
