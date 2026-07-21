use rust_software_architecture::clean_architecture::{
    application::{ConfirmReservation, ConfirmReservationRequest, ReservationRepository},
    domain::Reservation,
    CleanArchitectureError,
};

#[derive(Default)]
struct AuditedReservationRepository {
    saved: Vec<Reservation>,
    audit_log: Vec<String>,
}

impl AuditedReservationRepository {
    fn audit_count(&self) -> usize {
        self.audit_log.len()
    }

    fn saved_count(&self) -> usize {
        self.saved.len()
    }
}

impl ReservationRepository for AuditedReservationRepository {
    fn save(&mut self, reservation: Reservation) -> Result<(), CleanArchitectureError> {
        self.audit_log
            .push(format!("confirmed:{}", reservation.id().as_str()));
        self.saved.push(reservation);
        Ok(())
    }
}

fn main() {
    let mut repository = AuditedReservationRepository::default();
    let reservation = ConfirmReservation::new(&mut repository)
        .execute(ConfirmReservationRequest::new(
            "RSV-CLEAN-SOL",
            "flight-mx-sol",
            "customer-clean-sol",
        ))
        .expect("reserva confirmada");

    println!(
        "{} guardada {} vez y auditada {} vez",
        reservation.id().as_str(),
        repository.saved_count(),
        repository.audit_count()
    );
}
