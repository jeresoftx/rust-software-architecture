use rust_software_architecture::modular_monolith::inventory::{Capacity, Inventory};

fn main() {
    let inventory = Inventory::new(Capacity::new(3).expect("capacidad válida"));

    println!("unidades disponibles: {}", inventory.available_units());
}
