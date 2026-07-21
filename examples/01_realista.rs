use rust_software_architecture::modular_monolith::{
    booking::{BookingService, ReservationId},
    inventory::{Capacity, Inventory},
    pricing::{Clock, Money, PricingService},
    ModularMonolithError,
};

fn main() {
    let quoted_at = Clock::fixed(1_000);
    let confirmed_at = Clock::fixed(1_031);
    let mut inventory = Inventory::new(Capacity::new(1).expect("capacidad válida"));
    let pricing = PricingService::new(quoted_at);
    let quote = pricing
        .quote("flight-mx-202", Money::mxn(2_100), 30)
        .expect("cotización válida");

    let result = BookingService::confirm(
        ReservationId::new("RSV-202").expect("id válido"),
        quote,
        &mut inventory,
        confirmed_at,
    );

    match result {
        Ok(reservation) => println!("reserva confirmada: {}", reservation.id().as_str()),
        Err(ModularMonolithError::QuoteExpired) => {
            println!(
                "cotización expirada; inventario intacto: {}",
                inventory.available_units()
            );
        }
        Err(error) => println!("reserva rechazada: {error:?}"),
    }
}
