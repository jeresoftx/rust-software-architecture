use rust_software_architecture::cqrs::{
    commands::{ConfirmReservation, ReservationWriteModel},
    queries::FindConfirmedReservations,
    read_model::{ProjectionLag, ReservationSummary, ReservationSummaryProjection},
};

fn summaries_for_customer<'a>(
    summaries: &'a [ReservationSummary],
    customer_id: &str,
) -> Vec<&'a ReservationSummary> {
    summaries
        .iter()
        .filter(|summary| summary.customer_id() == customer_id)
        .collect()
}

fn main() {
    let mut write_model = ReservationWriteModel::default();
    let mut projection = ReservationSummaryProjection::default();

    let first_event = ConfirmReservation::new("RSV-CQRS-1", "OFR-MEX-1", "CUS-42")
        .execute(&mut write_model)
        .expect("primera reserva confirmada");

    let second_event = ConfirmReservation::new("RSV-CQRS-2", "OFR-MEX-2", "CUS-42")
        .execute(&mut write_model)
        .expect("segunda reserva confirmada");

    projection.apply(first_event);
    projection.mark_lagging();

    let visible_summaries = FindConfirmedReservations::new(&projection).execute();
    let customer_summaries = summaries_for_customer(&visible_summaries, "CUS-42");

    println!(
        "La escritura tiene {} reservas confirmadas.",
        write_model.confirmed_count()
    );
    println!(
        "La lectura muestra {} reservas para CUS-42 y está {:?}.",
        customer_summaries.len(),
        projection.lag()
    );

    projection.apply(second_event);
    let caught_up_summaries = FindConfirmedReservations::new(&projection).execute();

    if projection.lag() == ProjectionLag::CaughtUp {
        println!(
            "La proyección alcanzó la escritura con {} reservas visibles.",
            caught_up_summaries.len()
        );
    }
}
