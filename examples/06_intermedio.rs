use rust_software_architecture::event_sourcing::{
    commands::ConfirmReservation, stream::ReservationEventStream, EventSourcingError,
};

fn main() {
    let mut stream = ReservationEventStream::default();

    let result = ConfirmReservation::new("RSV-ES-200").execute(&mut stream);

    match result {
        Err(EventSourcingError::ReservationNotRequested) => {
            println!("No se puede confirmar una reserva que no fue solicitada.");
        }
        Ok(_) => println!("El comando no debió producir un evento."),
        Err(error) => println!("Error inesperado: {error:?}"),
    }

    println!("Eventos guardados: {}", stream.len());
}
