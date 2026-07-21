use rust_software_architecture::event_sourcing::{
    commands::{CancelReservation, RequestReservation},
    stream::ReservationEventStream,
    Reservation,
};

fn main() {
    let mut stream = ReservationEventStream::default();

    RequestReservation::new("RSV-ES-300", "OFR-ES-2026", "CUS-84")
        .execute(&mut stream)
        .expect("reserva solicitada");
    CancelReservation::new("RSV-ES-300")
        .execute(&mut stream)
        .expect("reserva cancelada");

    for event in stream.events() {
        println!(
            "v{} {:?} para {}",
            event.version(),
            event.kind(),
            event.reservation_id()
        );
    }

    let reservation = Reservation::rehydrate(stream.events()).expect("historia válida");

    println!(
        "Estado reconstruido: {:?} para cliente {}.",
        reservation.status(),
        reservation.customer_id()
    );
}
