use rust_software_architecture::microservices::{
    contracts::{ConfirmReservationRequest, ServiceContract},
    data_ownership::{DataOwnershipCatalog, ServiceName},
    inventory::InventoryService,
    payments::{PaymentGatewayMode, PaymentService},
    reservations::ReservationService,
    MicroservicesError,
};

fn main() {
    let mut catalog = DataOwnershipCatalog::default();

    catalog
        .claim_table(ServiceName::Reservations, "reservations")
        .expect("reservas gobierna su tabla");
    catalog
        .claim_table(ServiceName::Payments, "payments")
        .expect("pagos gobierna su tabla");
    catalog
        .claim_table(ServiceName::Inventory, "inventory")
        .expect("inventario gobierna su tabla");

    let mut reservations = ReservationService::default();
    let mut inventory = InventoryService::with_capacity("OFR-MICRO-SOL", 1);
    let mut payments = PaymentService::new(PaymentGatewayMode::Available);
    let request = ConfirmReservationRequest::new(
        "RSV-MICRO-SOL",
        "OFR-MICRO-SOL",
        "CUS-MICRO-SOL",
        15_000,
    )
    .expect("contrato válido");

    let confirmation = reservations
        .confirm(request, &mut inventory, &mut payments)
        .expect("confirmación entre servicios");

    println!(
        "{} v{} confirmó {}.",
        confirmation.contract_name(),
        confirmation.contract_version(),
        confirmation.reservation_id()
    );
    println!("Tablas con owner: {}", catalog.claim_count());
    println!("Reservas confirmadas: {}", reservations.confirmed_count());

    let mut failing_payments = PaymentService::new(PaymentGatewayMode::Unavailable);
    let mut second_inventory = InventoryService::with_capacity("OFR-MICRO-SOL", 1);
    let failed_request = ConfirmReservationRequest::new(
        "RSV-MICRO-FAIL",
        "OFR-MICRO-SOL",
        "CUS-MICRO-FAIL",
        15_000,
    )
    .expect("contrato válido");
    let failed = reservations.confirm(failed_request, &mut second_inventory, &mut failing_payments);

    if failed == Err(MicroservicesError::RemoteServiceUnavailable) {
        println!("La falla remota quedó visible y no confirmó una reserva nueva.");
    }
}
