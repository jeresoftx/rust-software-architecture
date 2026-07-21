#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModularMonolithError {
    EmptyIdentifier,
    InvalidMoney,
    InvalidQuoteDuration,
    InventoryUnavailable,
    QuoteExpired,
}

pub mod inventory {
    use super::ModularMonolithError;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Capacity {
        units: u32,
    }

    impl Capacity {
        pub fn new(units: u32) -> Result<Self, ModularMonolithError> {
            Ok(Self { units })
        }

        pub fn units(self) -> u32 {
            self.units
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Inventory {
        available_units: u32,
    }

    impl Inventory {
        pub fn new(capacity: Capacity) -> Self {
            Self {
                available_units: capacity.units(),
            }
        }

        pub fn available_units(&self) -> u32 {
            self.available_units
        }

        pub(in crate::modular_monolith) fn reserve_one(
            &mut self,
        ) -> Result<(), ModularMonolithError> {
            if self.available_units == 0 {
                return Err(ModularMonolithError::InventoryUnavailable);
            }

            self.available_units -= 1;
            Ok(())
        }
    }
}

pub mod pricing {
    use super::ModularMonolithError;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Clock {
        epoch_seconds: u64,
    }

    impl Clock {
        pub fn fixed(epoch_seconds: u64) -> Self {
            Self { epoch_seconds }
        }

        pub fn epoch_seconds(self) -> u64 {
            self.epoch_seconds
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Money {
        cents: u64,
        currency: Currency,
    }

    impl Money {
        pub fn mxn(units: u64) -> Self {
            Self {
                cents: units.saturating_mul(100),
                currency: Currency::Mxn,
            }
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

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Quote {
        offer_id: String,
        price: Money,
        issued_at: Clock,
        expires_at: Clock,
    }

    impl Quote {
        pub fn offer_id(&self) -> &str {
            &self.offer_id
        }

        pub fn price(&self) -> Money {
            self.price
        }

        pub fn is_expired_at(&self, clock: Clock) -> bool {
            clock.epoch_seconds() > self.expires_at.epoch_seconds()
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct PricingService {
        clock: Clock,
    }

    impl PricingService {
        pub fn new(clock: Clock) -> Self {
            Self { clock }
        }

        pub fn quote(
            &self,
            offer_id: impl Into<String>,
            price: Money,
            valid_for_seconds: u64,
        ) -> Result<Quote, ModularMonolithError> {
            let offer_id = offer_id.into();
            if offer_id.trim().is_empty() {
                return Err(ModularMonolithError::EmptyIdentifier);
            }

            if price.cents() == 0 {
                return Err(ModularMonolithError::InvalidMoney);
            }

            if valid_for_seconds == 0 {
                return Err(ModularMonolithError::InvalidQuoteDuration);
            }

            Ok(Quote {
                offer_id,
                price,
                issued_at: self.clock,
                expires_at: Clock::fixed(self.clock.epoch_seconds() + valid_for_seconds),
            })
        }
    }
}

pub mod booking {
    use super::{
        inventory::Inventory,
        pricing::{Clock, Money, Quote},
        ModularMonolithError,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationId(String);

    impl ReservationId {
        pub fn new(value: impl Into<String>) -> Result<Self, ModularMonolithError> {
            let value = value.into();
            if value.trim().is_empty() {
                return Err(ModularMonolithError::EmptyIdentifier);
            }

            Ok(Self(value))
        }

        pub fn as_str(&self) -> &str {
            &self.0
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct Reservation {
        id: ReservationId,
        offer_id: String,
        confirmed_price: Money,
    }

    impl Reservation {
        pub fn id(&self) -> &ReservationId {
            &self.id
        }

        pub fn offer_id(&self) -> &str {
            &self.offer_id
        }

        pub fn confirmed_price(&self) -> Money {
            self.confirmed_price
        }
    }

    pub struct BookingService;

    impl BookingService {
        pub fn confirm(
            id: ReservationId,
            quote: Quote,
            inventory: &mut Inventory,
            clock: Clock,
        ) -> Result<Reservation, ModularMonolithError> {
            if quote.is_expired_at(clock) {
                return Err(ModularMonolithError::QuoteExpired);
            }

            inventory.reserve_one()?;

            Ok(Reservation {
                id,
                offer_id: quote.offer_id().to_owned(),
                confirmed_price: quote.price(),
            })
        }
    }
}
