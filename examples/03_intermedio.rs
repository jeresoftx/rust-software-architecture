use rust_software_architecture::clean_architecture::{
    adapters::InMemoryReservationRepository,
    application::{ConfirmReservation, ConfirmReservationRequest},
    CleanArchitectureError,
};

fn main() {
    let mut repository = InMemoryReservationRepository::default();
    let result = ConfirmReservation::new(&mut repository).execute(ConfirmReservationRequest::new(
        "",
        "flight-mx-clean",
        "customer-clean-invalid",
    ));

    match result {
        Err(CleanArchitectureError::InvalidInput) => {
            println!("la frontera de entrada rechazó datos inválidos");
        }
        Ok(reservation) => println!("la reserva no debió confirmarse: {reservation:?}"),
        Err(error) => println!("falló por otra razón: {error:?}"),
    }

    println!("reservas guardadas: {}", repository.saved_count());
}
