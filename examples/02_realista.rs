use rust_software_architecture::hexagonal_architecture::{
    adapters::FailingReservationStore,
    application::{ConfirmBooking, ConfirmBookingCommand},
    HexagonalArchitectureError,
};

fn main() {
    let mut store = FailingReservationStore;
    let result = ConfirmBooking::new(&mut store).execute(ConfirmBookingCommand::new(
        "RSV-HEX-REAL",
        "flight-mx-hex",
        "quote-hex-real",
    ));

    match result {
        Err(HexagonalArchitectureError::OutputPortFailed) => {
            println!("el caso de uso no ocultó la falla del adaptador");
        }
        Ok(reservation) => println!("reserva confirmada: {}", reservation.id().as_str()),
        Err(error) => println!("falló por otra razón: {error:?}"),
    }
}
