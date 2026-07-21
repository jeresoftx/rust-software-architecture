use rust_software_architecture::modular_monolith::{
    booking::{BookingService, ReservationId},
    inventory::{Capacity, Inventory},
    pricing::{Clock, Money, PricingService},
    ModularMonolithError,
};

fn confirmar_reserva(
    reservation_id: &str,
    offer_id: &str,
    inventory: &mut Inventory,
    clock: Clock,
) -> Result<String, ModularMonolithError> {
    let pricing = PricingService::new(clock);
    let quote = pricing.quote(offer_id, Money::mxn(1_500), 30)?;
    let reservation =
        BookingService::confirm(ReservationId::new(reservation_id)?, quote, inventory, clock)?;

    Ok(reservation.id().as_str().to_owned())
}

fn main() {
    let clock = Clock::fixed(1_000);
    let mut inventory = Inventory::new(Capacity::new(1).expect("capacidad válida"));
    let reservation_id =
        confirmar_reserva("RSV-SOL-001", "flight-mx-sol", &mut inventory, clock)
            .expect("reserva confirmada");

    println!("{reservation_id}");
}
