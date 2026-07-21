use rust_software_architecture::domain_driven_design::{
    domain::{Money, ReservationId},
    DomainDrivenDesignError,
};

fn main() {
    let empty_id = ReservationId::new("");
    let free_price = Money::mxn(0);

    if empty_id == Err(DomainDrivenDesignError::InvalidValue) {
        println!("el value object rechazó una identidad vacía");
    }

    if free_price == Err(DomainDrivenDesignError::InvalidValue) {
        println!("el value object rechazó una cotización sin precio");
    }
}
