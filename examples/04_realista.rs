use rust_software_architecture::domain_driven_design::{
    domain::{CustomerId, Money, OfferId, Reservation, ReservationId},
    repositories::{InMemoryReservationRepository, ReservationRepository},
};

fn main() {
    let mut reservation = Reservation::request(
        ReservationId::new("RSV-DDD-REAL").expect("identidad válida"),
        OfferId::new("flight-mx-ddd").expect("oferta válida"),
        CustomerId::new("customer-ddd-real").expect("cliente válido"),
        Money::mxn(2_400).expect("precio válido"),
    );
    reservation.confirm().expect("reserva confirmada");

    let mut repository = InMemoryReservationRepository::default();
    repository
        .save(reservation.clone())
        .expect("agregado guardado");

    let stored = repository
        .find(reservation.id())
        .expect("agregado recuperado");

    println!(
        "{} guardada con estado {:?}",
        stored.id().as_str(),
        stored.status()
    );
}
