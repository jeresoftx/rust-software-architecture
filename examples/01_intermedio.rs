use rust_software_architecture::modular_monolith::{
    booking::{BookingService, ReservationId},
    inventory::{Capacity, Inventory},
    pricing::{Clock, Money, PricingService},
};

fn main() {
    let clock = Clock::fixed(1_000);
    let mut inventory = Inventory::new(Capacity::new(1).expect("capacidad válida"));
    let pricing = PricingService::new(clock);
    let quote = pricing
        .quote("flight-mx-101", Money::mxn(1_800), 60)
        .expect("cotización válida");

    let reservation = BookingService::confirm(
        ReservationId::new("RSV-101").expect("id válido"),
        quote,
        &mut inventory,
        clock,
    )
    .expect("reserva confirmada");

    println!(
        "{} confirmó {} y quedan {} unidades",
        reservation.id().as_str(),
        reservation.offer_id(),
        inventory.available_units()
    );
}
