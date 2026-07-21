use rust_software_architecture::hexagonal_architecture::{
    adapters::InMemoryReservationStore,
    application::{ConfirmBooking, ConfirmBookingCommand},
    HexagonalArchitectureError,
};

fn main() {
    let mut store = InMemoryReservationStore::default();
    let result = ConfirmBooking::new(&mut store).execute(ConfirmBookingCommand::new(
        "",
        "flight-mx-hex",
        "quote-hex-invalid",
    ));

    match result {
        Err(HexagonalArchitectureError::InvalidInput) => {
            println!("entrada rechazada antes de tocar el adaptador");
        }
        Ok(_) => println!("la entrada inválida no debió confirmarse"),
        Err(error) => println!("falló por otra razón: {error:?}"),
    }

    println!("reservas guardadas: {}", store.saved_count());
}
