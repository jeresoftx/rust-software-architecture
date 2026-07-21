use rust_software_architecture::cqrs::commands::{ConfirmReservation, ReservationWriteModel};

fn main() {
    let mut write_model = ReservationWriteModel::default();

    let event = ConfirmReservation::new("RSV-100", "OFR-2026", "CUS-42")
        .execute(&mut write_model)
        .expect("la reserva debe confirmarse");

    println!(
        "Reserva confirmada: {} para cliente {}",
        event.reservation_id, event.customer_id
    );
    println!(
        "Reservas confirmadas en escritura: {}",
        write_model.confirmed_count()
    );
}
