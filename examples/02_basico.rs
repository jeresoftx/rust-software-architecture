use rust_software_architecture::hexagonal_architecture::{
    adapters::InMemoryReservationStore,
    application::{ConfirmBooking, ConfirmBookingCommand},
};

fn main() {
    let mut store = InMemoryReservationStore::default();
    let reservation = ConfirmBooking::new(&mut store)
        .execute(ConfirmBookingCommand::new(
            "RSV-HEX-BASIC",
            "flight-mx-hex",
            "quote-hex-basic",
        ))
        .expect("reserva confirmada");

    println!(
        "{} guardada por adaptador en memoria",
        reservation.id().as_str()
    );
}
