#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HexagonalArchitectureError {
    InvalidInput,
    OutputPortFailed,
}

pub mod domain {
    use super::HexagonalArchitectureError;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationId(String);

    impl ReservationId {
        pub fn new(value: impl Into<String>) -> Result<Self, HexagonalArchitectureError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(HexagonalArchitectureError::InvalidInput);
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct OfferId(String);

    impl OfferId {
        pub fn new(value: impl Into<String>) -> Result<Self, HexagonalArchitectureError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(HexagonalArchitectureError::InvalidInput);
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct QuoteId(String);

    impl QuoteId {
        pub fn new(value: impl Into<String>) -> Result<Self, HexagonalArchitectureError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(HexagonalArchitectureError::InvalidInput);
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReservationStatus {
        Confirmed,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Reservation {
        id: ReservationId,
        offer_id: OfferId,
        quote_id: QuoteId,
        status: ReservationStatus,
    }

    impl Reservation {
        pub fn confirm(id: ReservationId, offer_id: OfferId, quote_id: QuoteId) -> Self {
            Self {
                id,
                offer_id,
                quote_id,
                status: ReservationStatus::Confirmed,
            }
        }

        pub fn id(&self) -> &ReservationId {
            &self.id
        }

        pub fn offer_id(&self) -> &OfferId {
            &self.offer_id
        }

        pub fn quote_id(&self) -> &QuoteId {
            &self.quote_id
        }

        pub fn status(&self) -> ReservationStatus {
            self.status
        }
    }
}

pub mod ports {
    use super::{domain::Reservation, HexagonalArchitectureError};

    pub trait ReservationStore {
        fn save(&mut self, reservation: Reservation) -> Result<(), HexagonalArchitectureError>;
    }
}

pub mod application {
    use super::{
        domain::{OfferId, QuoteId, Reservation, ReservationId},
        ports::ReservationStore,
        HexagonalArchitectureError,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ConfirmBookingCommand {
        reservation_id: String,
        offer_id: String,
        quote_id: String,
    }

    impl ConfirmBookingCommand {
        pub fn new(
            reservation_id: impl Into<String>,
            offer_id: impl Into<String>,
            quote_id: impl Into<String>,
        ) -> Self {
            Self {
                reservation_id: reservation_id.into(),
                offer_id: offer_id.into(),
                quote_id: quote_id.into(),
            }
        }
    }

    pub struct ConfirmBooking<'a, Store>
    where
        Store: ReservationStore,
    {
        reservation_store: &'a mut Store,
    }

    impl<'a, Store> ConfirmBooking<'a, Store>
    where
        Store: ReservationStore,
    {
        pub fn new(reservation_store: &'a mut Store) -> Self {
            Self { reservation_store }
        }

        pub fn execute(
            self,
            command: ConfirmBookingCommand,
        ) -> Result<Reservation, HexagonalArchitectureError> {
            let reservation = Reservation::confirm(
                ReservationId::new(command.reservation_id)?,
                OfferId::new(command.offer_id)?,
                QuoteId::new(command.quote_id)?,
            );

            self.reservation_store.save(reservation.clone())?;

            Ok(reservation)
        }
    }
}

pub mod adapters {
    use super::{domain::Reservation, ports::ReservationStore, HexagonalArchitectureError};

    #[derive(Default)]
    pub struct InMemoryReservationStore {
        saved: Vec<Reservation>,
    }

    impl InMemoryReservationStore {
        pub fn saved_count(&self) -> usize {
            self.saved.len()
        }

        pub fn reservations(&self) -> &[Reservation] {
            &self.saved
        }
    }

    impl ReservationStore for InMemoryReservationStore {
        fn save(&mut self, reservation: Reservation) -> Result<(), HexagonalArchitectureError> {
            self.saved.push(reservation);
            Ok(())
        }
    }

    pub struct FailingReservationStore;

    impl ReservationStore for FailingReservationStore {
        fn save(&mut self, _reservation: Reservation) -> Result<(), HexagonalArchitectureError> {
            Err(HexagonalArchitectureError::OutputPortFailed)
        }
    }
}
