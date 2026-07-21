use rust_software_architecture::domain_driven_design::{
    domain::{CustomerId, Money, OfferId, Reservation, ReservationId},
    events::DomainEvent,
    repositories::{InMemoryReservationRepository, ReservationRepository},
};

#[derive(Default)]
struct DomainEventRecorder {
    recorded: Vec<DomainEvent>,
}

impl DomainEventRecorder {
    fn record(&mut self, event: DomainEvent) {
        self.recorded.push(event);
    }

    fn recorded_count(&self) -> usize {
        self.recorded.len()
    }
}

fn main() {
    let mut reservation = Reservation::request(
        ReservationId::new("RSV-DDD-SOL").expect("identidad válida"),
        OfferId::new("flight-mx-sol").expect("oferta válida"),
        CustomerId::new("customer-ddd-sol").expect("cliente válido"),
        Money::mxn(2_700).expect("precio válido"),
    );

    let mut recorder = DomainEventRecorder::default();
    let event = reservation.confirm().expect("reserva confirmada");
    recorder.record(event);

    let mut repository = InMemoryReservationRepository::default();
    repository
        .save(reservation.clone())
        .expect("agregado guardado");

    println!(
        "{} guardada con {} evento de dominio",
        reservation.id().as_str(),
        recorder.recorded_count()
    );
}
