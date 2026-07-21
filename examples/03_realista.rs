use rust_software_architecture::clean_architecture::{
    adapters::FailingReservationRepository,
    application::{ConfirmReservation, ConfirmReservationRequest},
    CleanArchitectureError,
};

fn main() {
    let mut repository = FailingReservationRepository;
    let result = ConfirmReservation::new(&mut repository).execute(ConfirmReservationRequest::new(
        "RSV-CLEAN-REAL",
        "flight-mx-clean",
        "customer-clean-real",
    ));

    match result {
        Err(CleanArchitectureError::RepositoryFailed) => {
            println!("el caso de uso no ocultó la falla de repositorio");
        }
        Ok(reservation) => println!("reserva confirmada: {}", reservation.id().as_str()),
        Err(error) => println!("falló por otra razón: {error:?}"),
    }
}
