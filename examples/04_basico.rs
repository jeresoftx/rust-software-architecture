use rust_software_architecture::domain_driven_design::{
    domain::{CustomerId, Money, OfferId, Reservation, ReservationId},
    events::DomainEvent,
};

fn main() {
    let mut reservation = Reservation::request(
        ReservationId::new("RSV-DDD-BASIC").expect("identidad válida"),
        OfferId::new("flight-mx-ddd").expect("oferta válida"),
        CustomerId::new("customer-ddd-basic").expect("cliente válido"),
        Money::mxn(2_100).expect("precio válido"),
    );

    match reservation.confirm().expect("reserva confirmada") {
        DomainEvent::ReservationConfirmed { reservation_id } => {
            println!("hecho de dominio: {reservation_id} confirmada");
        }
        DomainEvent::ReservationCancelled { reservation_id } => {
            println!("hecho inesperado en este ejemplo: {reservation_id} cancelada");
        }
    }
}
