use rust_software_architecture::microservices::{
    contracts::ConfirmReservationRequest,
    inventory::InventoryService,
    payments::{PaymentGatewayMode, PaymentService},
    reservations::ReservationService,
    MicroservicesError,
};

fn main() {
    let mut reservations = ReservationService::default();
    let mut inventory = InventoryService::with_capacity("OFR-MICRO-2026", 1);
    let mut payments = PaymentService::new(PaymentGatewayMode::Unavailable);
    let request =
        ConfirmReservationRequest::new("RSV-MICRO-300", "OFR-MICRO-2026", "CUS-MICRO-126", 22_000)
            .expect("contrato válido");

    let result = reservations.confirm(request, &mut inventory, &mut payments);

    match result {
        Err(MicroservicesError::RemoteServiceUnavailable) => {
            println!("Pago falló como servicio remoto; la reserva no se confirmó.");
        }
        Ok(_) => println!("La reserva no debió confirmarse."),
        Err(error) => println!("Error inesperado: {error:?}"),
    }

    println!("Reservas confirmadas: {}", reservations.confirmed_count());
    println!("Autorizaciones de pago: {}", payments.authorized_count());
    println!("Holds de inventario: {}", inventory.held_count());
}
