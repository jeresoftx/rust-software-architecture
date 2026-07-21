#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CleanArchitectureError {
    InvalidInput,
    RepositoryFailed,
}

pub mod domain {
    use super::CleanArchitectureError;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationId(String);

    impl ReservationId {
        pub fn new(value: impl Into<String>) -> Result<Self, CleanArchitectureError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(CleanArchitectureError::InvalidInput);
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
        pub fn new(value: impl Into<String>) -> Result<Self, CleanArchitectureError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(CleanArchitectureError::InvalidInput);
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct CustomerId(String);

    impl CustomerId {
        pub fn new(value: impl Into<String>) -> Result<Self, CleanArchitectureError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(CleanArchitectureError::InvalidInput);
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
        customer_id: CustomerId,
        status: ReservationStatus,
    }

    impl Reservation {
        pub fn confirm(id: ReservationId, offer_id: OfferId, customer_id: CustomerId) -> Self {
            Self {
                id,
                offer_id,
                customer_id,
                status: ReservationStatus::Confirmed,
            }
        }

        pub fn id(&self) -> &ReservationId {
            &self.id
        }

        pub fn offer_id(&self) -> &OfferId {
            &self.offer_id
        }

        pub fn customer_id(&self) -> &CustomerId {
            &self.customer_id
        }

        pub fn status(&self) -> ReservationStatus {
            self.status
        }
    }
}

pub mod application {
    use super::{
        domain::{CustomerId, OfferId, Reservation, ReservationId},
        CleanArchitectureError,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ConfirmReservationRequest {
        reservation_id: String,
        offer_id: String,
        customer_id: String,
    }

    impl ConfirmReservationRequest {
        pub fn new(
            reservation_id: impl Into<String>,
            offer_id: impl Into<String>,
            customer_id: impl Into<String>,
        ) -> Self {
            Self {
                reservation_id: reservation_id.into(),
                offer_id: offer_id.into(),
                customer_id: customer_id.into(),
            }
        }
    }

    pub trait ReservationRepository {
        fn save(&mut self, reservation: Reservation) -> Result<(), CleanArchitectureError>;
    }

    pub struct ConfirmReservation<'a, Repository>
    where
        Repository: ReservationRepository,
    {
        repository: &'a mut Repository,
    }

    impl<'a, Repository> ConfirmReservation<'a, Repository>
    where
        Repository: ReservationRepository,
    {
        pub fn new(repository: &'a mut Repository) -> Self {
            Self { repository }
        }

        pub fn execute(
            self,
            request: ConfirmReservationRequest,
        ) -> Result<Reservation, CleanArchitectureError> {
            let reservation = Reservation::confirm(
                ReservationId::new(request.reservation_id)?,
                OfferId::new(request.offer_id)?,
                CustomerId::new(request.customer_id)?,
            );

            self.repository.save(reservation.clone())?;

            Ok(reservation)
        }
    }
}

pub mod adapters {
    use super::{application::ReservationRepository, domain::Reservation, CleanArchitectureError};

    #[derive(Default)]
    pub struct InMemoryReservationRepository {
        saved: Vec<Reservation>,
    }

    impl InMemoryReservationRepository {
        pub fn saved_count(&self) -> usize {
            self.saved.len()
        }

        pub fn reservations(&self) -> &[Reservation] {
            &self.saved
        }
    }

    impl ReservationRepository for InMemoryReservationRepository {
        fn save(&mut self, reservation: Reservation) -> Result<(), CleanArchitectureError> {
            self.saved.push(reservation);
            Ok(())
        }
    }

    pub struct FailingReservationRepository;

    impl ReservationRepository for FailingReservationRepository {
        fn save(&mut self, _reservation: Reservation) -> Result<(), CleanArchitectureError> {
            Err(CleanArchitectureError::RepositoryFailed)
        }
    }
}
