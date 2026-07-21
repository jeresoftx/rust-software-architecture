#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DomainDrivenDesignError {
    InvalidValue,
    InvalidTransition,
}

pub mod events {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub enum DomainEvent {
        ReservationConfirmed { reservation_id: String },
        ReservationCancelled { reservation_id: String },
    }
}

pub mod domain {
    use super::{events::DomainEvent, DomainDrivenDesignError};

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationId(String);

    impl ReservationId {
        pub fn new(value: impl Into<String>) -> Result<Self, DomainDrivenDesignError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(DomainDrivenDesignError::InvalidValue);
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
        pub fn new(value: impl Into<String>) -> Result<Self, DomainDrivenDesignError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(DomainDrivenDesignError::InvalidValue);
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
        pub fn new(value: impl Into<String>) -> Result<Self, DomainDrivenDesignError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(DomainDrivenDesignError::InvalidValue);
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Money {
        cents: u64,
        currency: Currency,
    }

    impl Money {
        pub fn mxn(units: u64) -> Result<Self, DomainDrivenDesignError> {
            if units == 0 {
                return Err(DomainDrivenDesignError::InvalidValue);
            }

            Ok(Self {
                cents: units.saturating_mul(100),
                currency: Currency::Mxn,
            })
        }

        pub fn cents(self) -> u64 {
            self.cents
        }

        pub fn currency(self) -> Currency {
            self.currency
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum Currency {
        Mxn,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReservationStatus {
        Pending,
        Confirmed,
        Cancelled,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Reservation {
        id: ReservationId,
        offer_id: OfferId,
        customer_id: CustomerId,
        quoted_price: Money,
        status: ReservationStatus,
    }

    impl Reservation {
        pub fn request(
            id: ReservationId,
            offer_id: OfferId,
            customer_id: CustomerId,
            quoted_price: Money,
        ) -> Self {
            Self {
                id,
                offer_id,
                customer_id,
                quoted_price,
                status: ReservationStatus::Pending,
            }
        }

        pub fn confirm(&mut self) -> Result<DomainEvent, DomainDrivenDesignError> {
            if self.status != ReservationStatus::Pending {
                return Err(DomainDrivenDesignError::InvalidTransition);
            }

            self.status = ReservationStatus::Confirmed;

            Ok(DomainEvent::ReservationConfirmed {
                reservation_id: self.id.as_str().to_owned(),
            })
        }

        pub fn cancel(&mut self) -> Result<DomainEvent, DomainDrivenDesignError> {
            if self.status == ReservationStatus::Cancelled {
                return Err(DomainDrivenDesignError::InvalidTransition);
            }

            self.status = ReservationStatus::Cancelled;

            Ok(DomainEvent::ReservationCancelled {
                reservation_id: self.id.as_str().to_owned(),
            })
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

        pub fn quoted_price(&self) -> Money {
            self.quoted_price
        }

        pub fn status(&self) -> ReservationStatus {
            self.status
        }
    }
}

pub mod repositories {
    use super::{
        domain::{Reservation, ReservationId},
        DomainDrivenDesignError,
    };

    pub trait ReservationRepository {
        fn save(&mut self, reservation: Reservation) -> Result<(), DomainDrivenDesignError>;
        fn find(&self, id: &ReservationId) -> Option<&Reservation>;
    }

    #[derive(Default)]
    pub struct InMemoryReservationRepository {
        saved: Vec<Reservation>,
    }

    impl InMemoryReservationRepository {
        pub fn saved_count(&self) -> usize {
            self.saved.len()
        }
    }

    impl ReservationRepository for InMemoryReservationRepository {
        fn save(&mut self, reservation: Reservation) -> Result<(), DomainDrivenDesignError> {
            if let Some(stored) = self
                .saved
                .iter_mut()
                .find(|stored| stored.id() == reservation.id())
            {
                *stored = reservation;
            } else {
                self.saved.push(reservation);
            }

            Ok(())
        }

        fn find(&self, id: &ReservationId) -> Option<&Reservation> {
            self.saved.iter().find(|reservation| reservation.id() == id)
        }
    }
}
