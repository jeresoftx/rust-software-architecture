use rust_software_architecture::microservices::{
    contracts::{ConfirmReservationRequest, ServiceContract},
    inventory::InventoryService,
    payments::{PaymentGatewayMode, PaymentService},
    reservations::ReservationService,
};

fn main() {
    let mut reservations = ReservationService::default();
    let mut inventory = InventoryService::with_capacity("OFR-MICRO-2026", 2);
    let mut payments = PaymentService::new(PaymentGatewayMode::Available);
    let request =
        ConfirmReservationRequest::new("RSV-MICRO-200", "OFR-MICRO-2026", "CUS-MICRO-84", 18_500)
            .expect("contrato válido");

    let confirmation = reservations
        .confirm(request, &mut inventory, &mut payments)
        .expect("reserva confirmada");

    println!(
        "{} v{} para {}.",
        confirmation.contract_name(),
        confirmation.contract_version(),
        confirmation.reservation_id()
    );
    println!("Reservas confirmadas: {}", reservations.confirmed_count());
    println!("Autorizaciones de pago: {}", payments.authorized_count());
    println!("Holds de inventario: {}", inventory.held_count());
}
