use rust_software_architecture::microservices::{
    data_ownership::{DataOwnershipCatalog, ServiceName},
    MicroservicesError,
};

fn main() {
    let mut catalog = DataOwnershipCatalog::default();

    catalog
        .claim_table(ServiceName::Reservations, "reservations")
        .expect("tabla de reservas asignada");
    catalog
        .claim_table(ServiceName::Payments, "payments")
        .expect("tabla de pagos asignada");

    let shared = catalog.claim_table(ServiceName::Inventory, "reservations");

    match shared {
        Err(MicroservicesError::SharedDataOwnership) => {
            println!("Una tabla no puede ser API compartida entre servicios.");
        }
        Ok(()) => println!("El catálogo no debió permitir ownership compartido."),
        Err(error) => println!("Error inesperado: {error:?}"),
    }

    println!("Tablas con owner: {}", catalog.claim_count());
}
