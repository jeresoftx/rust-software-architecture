use rust_software_architecture::hexagonal_architecture::{
    application::{ConfirmBooking, ConfirmBookingCommand},
    domain::Reservation,
    ports::ReservationStore,
    HexagonalArchitectureError,
};

#[derive(Default)]
struct AuditedReservationStore {
    saved: Vec<Reservation>,
    audit_log: Vec<String>,
}

impl AuditedReservationStore {
    fn audit_count(&self) -> usize {
        self.audit_log.len()
    }
}

impl ReservationStore for AuditedReservationStore {
    fn save(&mut self, reservation: Reservation) -> Result<(), HexagonalArchitectureError> {
        self.audit_log
            .push(format!("confirmed:{}", reservation.id().as_str()));
        self.saved.push(reservation);
        Ok(())
    }
}

fn main() {
    let mut store = AuditedReservationStore::default();
    let reservation = ConfirmBooking::new(&mut store)
        .execute(ConfirmBookingCommand::new(
            "RSV-HEX-SOL",
            "flight-mx-sol",
            "quote-hex-sol",
        ))
        .expect("reserva confirmada");

    println!(
        "{} auditada {} vez",
        reservation.id().as_str(),
        store.audit_count()
    );
}
