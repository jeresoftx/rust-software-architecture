use rust_software_architecture::clean_architecture::{
    adapters::InMemoryReservationRepository,
    application::{ConfirmReservation, ConfirmReservationRequest},
};

fn main() {
    let mut repository = InMemoryReservationRepository::default();
    let reservation = ConfirmReservation::new(&mut repository)
        .execute(ConfirmReservationRequest::new(
            "RSV-CLEAN-BASIC",
            "flight-mx-clean",
            "customer-clean-basic",
        ))
        .expect("reserva confirmada");

    println!(
        "{} confirmada por caso de uso limpio",
        reservation.id().as_str()
    );
    println!("reservas guardadas: {}", repository.saved_count());
}
