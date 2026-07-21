use rust_software_architecture::event_sourcing::{
    commands::{ConfirmReservation, RequestReservation},
    stream::ReservationEventStream,
    Reservation,
};

fn main() {
    let mut stream = ReservationEventStream::default();

    RequestReservation::new("RSV-ES-100", "OFR-ES-2026", "CUS-42")
        .execute(&mut stream)
        .expect("reserva solicitada");
    ConfirmReservation::new("RSV-ES-100")
        .execute(&mut stream)
        .expect("reserva confirmada");

    let reservation = Reservation::rehydrate(stream.events()).expect("historia válida");

    println!(
        "Reserva {} reconstruida como {:?} desde {} eventos.",
        reservation.id(),
        reservation.status(),
        stream.len()
    );
}
