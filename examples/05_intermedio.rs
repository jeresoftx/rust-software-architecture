use rust_software_architecture::cqrs::{
    commands::{ConfirmReservation, ReservationWriteModel},
    CqrsError,
};

fn main() {
    let mut write_model = ReservationWriteModel::default();

    let result = ConfirmReservation::new("", "OFR-2026", "CUS-42").execute(&mut write_model);

    match result {
        Err(CqrsError::InvalidCommand) => {
            println!("El comando inválido fue rechazado sin cambiar escritura.");
        }
        Ok(_) => println!("El comando no debió aceptarse."),
    }

    println!(
        "Reservas confirmadas en escritura: {}",
        write_model.confirmed_count()
    );
}
