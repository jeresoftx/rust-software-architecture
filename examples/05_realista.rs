use rust_software_architecture::cqrs::{
    commands::{ConfirmReservation, ReservationWriteModel},
    queries::FindConfirmedReservations,
    read_model::{ProjectionLag, ReservationSummaryProjection},
};

fn main() {
    let mut write_model = ReservationWriteModel::default();
    let mut projection = ReservationSummaryProjection::default();

    let event = ConfirmReservation::new("RSV-100", "OFR-2026", "CUS-42")
        .execute(&mut write_model)
        .expect("la reserva debe confirmarse");

    projection.apply(event);

    let query = FindConfirmedReservations::new(&projection);
    let summaries = query.execute();

    for summary in summaries {
        println!(
            "Lectura: reserva {} oferta {} cliente {}",
            summary.reservation_id(),
            summary.offer_id(),
            summary.customer_id()
        );
    }

    if projection.lag() == ProjectionLag::CaughtUp {
        println!("La proyección está al día para esta lectura.");
    }
}
