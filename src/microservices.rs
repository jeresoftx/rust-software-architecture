#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MicroservicesError {
    InvalidContract,
    InventoryUnavailable,
    RemoteServiceUnavailable,
    SharedDataOwnership,
}

pub mod contracts {
    use super::MicroservicesError;

    pub trait ServiceContract {
        fn contract_name(&self) -> &str;
        fn contract_version(&self) -> u16;
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ConfirmReservationRequest {
        reservation_id: String,
        offer_id: String,
        customer_id: String,
        amount_cents: u32,
        contract_version: u16,
    }

    impl ConfirmReservationRequest {
        pub fn new(
            reservation_id: impl Into<String>,
            offer_id: impl Into<String>,
            customer_id: impl Into<String>,
            amount_cents: u32,
        ) -> Result<Self, MicroservicesError> {
            let reservation_id = reservation_id.into();
            let offer_id = offer_id.into();
            let customer_id = customer_id.into();

            if reservation_id.trim().is_empty()
                || offer_id.trim().is_empty()
                || customer_id.trim().is_empty()
                || amount_cents == 0
            {
                return Err(MicroservicesError::InvalidContract);
            }

            Ok(Self {
                reservation_id,
                offer_id,
                customer_id,
                amount_cents,
                contract_version: 1,
            })
        }

        pub fn reservation_id(&self) -> &str {
            &self.reservation_id
        }

        pub fn offer_id(&self) -> &str {
            &self.offer_id
        }

        pub fn customer_id(&self) -> &str {
            &self.customer_id
        }

        pub fn amount_cents(&self) -> u32 {
            self.amount_cents
        }
    }

    impl ServiceContract for ConfirmReservationRequest {
        fn contract_name(&self) -> &str {
            "ConfirmReservationRequest"
        }

        fn contract_version(&self) -> u16 {
            self.contract_version
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationConfirmation {
        reservation_id: String,
        payment_authorization_id: String,
        inventory_hold_id: String,
        contract_version: u16,
    }

    impl ReservationConfirmation {
        pub fn new(
            reservation_id: impl Into<String>,
            payment_authorization_id: impl Into<String>,
            inventory_hold_id: impl Into<String>,
        ) -> Result<Self, MicroservicesError> {
            let reservation_id = reservation_id.into();
            let payment_authorization_id = payment_authorization_id.into();
            let inventory_hold_id = inventory_hold_id.into();

            if reservation_id.trim().is_empty()
                || payment_authorization_id.trim().is_empty()
                || inventory_hold_id.trim().is_empty()
            {
                return Err(MicroservicesError::InvalidContract);
            }

            Ok(Self {
                reservation_id,
                payment_authorization_id,
                inventory_hold_id,
                contract_version: 1,
            })
        }

        pub fn reservation_id(&self) -> &str {
            &self.reservation_id
        }

        pub fn payment_authorization_id(&self) -> &str {
            &self.payment_authorization_id
        }

        pub fn inventory_hold_id(&self) -> &str {
            &self.inventory_hold_id
        }
    }

    impl ServiceContract for ReservationConfirmation {
        fn contract_name(&self) -> &str {
            "ReservationConfirmation"
        }

        fn contract_version(&self) -> u16 {
            self.contract_version
        }
    }
}

pub mod data_ownership {
    use super::MicroservicesError;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ServiceName {
        Reservations,
        Payments,
        Inventory,
    }

    #[derive(Default)]
    pub struct DataOwnershipCatalog {
        claims: Vec<(ServiceName, String)>,
    }

    impl DataOwnershipCatalog {
        pub fn claim_table(
            &mut self,
            owner: ServiceName,
            table: impl Into<String>,
        ) -> Result<(), MicroservicesError> {
            let table = table.into();

            if self.claims.iter().any(|(claimed_owner, claimed_table)| {
                claimed_owner != &owner && claimed_table == &table
            }) {
                return Err(MicroservicesError::SharedDataOwnership);
            }

            if !self.claims.iter().any(|(claimed_owner, claimed_table)| {
                claimed_owner == &owner && claimed_table == &table
            }) {
                self.claims.push((owner, table));
            }

            Ok(())
        }

        pub fn claim_count(&self) -> usize {
            self.claims.len()
        }
    }
}

pub mod payments {
    use super::{contracts::ConfirmReservationRequest, MicroservicesError};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum PaymentGatewayMode {
        Available,
        Unavailable,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct PaymentAuthorization {
        authorization_id: String,
        reservation_id: String,
    }

    impl PaymentAuthorization {
        pub fn authorization_id(&self) -> &str {
            &self.authorization_id
        }

        pub fn reservation_id(&self) -> &str {
            &self.reservation_id
        }
    }

    pub struct PaymentService {
        gateway_mode: PaymentGatewayMode,
        authorizations: Vec<PaymentAuthorization>,
    }

    impl PaymentService {
        pub fn new(gateway_mode: PaymentGatewayMode) -> Self {
            Self {
                gateway_mode,
                authorizations: Vec::new(),
            }
        }

        pub fn authorize(
            &mut self,
            request: &ConfirmReservationRequest,
        ) -> Result<PaymentAuthorization, MicroservicesError> {
            if self.gateway_mode == PaymentGatewayMode::Unavailable {
                return Err(MicroservicesError::RemoteServiceUnavailable);
            }

            let authorization = PaymentAuthorization {
                authorization_id: format!("AUTH-{}", request.reservation_id()),
                reservation_id: request.reservation_id().to_owned(),
            };

            self.authorizations.push(authorization.clone());

            Ok(authorization)
        }

        pub fn authorized_count(&self) -> usize {
            self.authorizations.len()
        }
    }
}

pub mod inventory {
    use super::MicroservicesError;

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct InventoryHold {
        hold_id: String,
        offer_id: String,
    }

    impl InventoryHold {
        pub fn hold_id(&self) -> &str {
            &self.hold_id
        }

        pub fn offer_id(&self) -> &str {
            &self.offer_id
        }
    }

    pub struct InventoryService {
        offer_id: String,
        available_units: u32,
        holds: Vec<InventoryHold>,
    }

    impl InventoryService {
        pub fn with_capacity(offer_id: impl Into<String>, available_units: u32) -> Self {
            Self {
                offer_id: offer_id.into(),
                available_units,
                holds: Vec::new(),
            }
        }

        pub fn hold(
            &mut self,
            offer_id: &str,
            reservation_id: &str,
        ) -> Result<InventoryHold, MicroservicesError> {
            if self.offer_id != offer_id || self.available_units == 0 {
                return Err(MicroservicesError::InventoryUnavailable);
            }

            self.available_units -= 1;
            let hold = InventoryHold {
                hold_id: format!("HOLD-{reservation_id}"),
                offer_id: offer_id.to_owned(),
            };

            self.holds.push(hold.clone());

            Ok(hold)
        }

        pub fn held_count(&self) -> usize {
            self.holds.len()
        }
    }
}

pub mod reservations {
    use super::{
        contracts::{ConfirmReservationRequest, ReservationConfirmation},
        inventory::InventoryService,
        payments::PaymentService,
        MicroservicesError,
    };

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ConfirmedReservation {
        reservation_id: String,
        customer_id: String,
    }

    #[derive(Default)]
    pub struct ReservationService {
        confirmed: Vec<ConfirmedReservation>,
    }

    impl ReservationService {
        pub fn confirm(
            &mut self,
            request: ConfirmReservationRequest,
            inventory: &mut InventoryService,
            payments: &mut PaymentService,
        ) -> Result<ReservationConfirmation, MicroservicesError> {
            let authorization = payments.authorize(&request)?;
            let hold = inventory.hold(request.offer_id(), request.reservation_id())?;
            let confirmation = ReservationConfirmation::new(
                request.reservation_id(),
                authorization.authorization_id(),
                hold.hold_id(),
            )?;

            self.confirmed.push(ConfirmedReservation {
                reservation_id: request.reservation_id().to_owned(),
                customer_id: request.customer_id().to_owned(),
            });

            Ok(confirmation)
        }

        pub fn confirmed_count(&self) -> usize {
            self.confirmed.len()
        }
    }
}
