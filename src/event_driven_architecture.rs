#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventDrivenArchitectureError {
    ConsumerFailed,
    InvalidContract,
}

pub mod contracts {
    use super::EventDrivenArchitectureError;

    pub trait IntegrationEvent {
        fn contract_name(&self) -> &str;
        fn contract_version(&self) -> u16;
        fn event_id(&self) -> &str;
        fn reservation_id(&self) -> &str;
        fn customer_id(&self) -> &str;
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationConfirmed {
        event_id: String,
        reservation_id: String,
        offer_id: String,
        customer_id: String,
        contract_version: u16,
    }

    impl ReservationConfirmed {
        pub fn new(
            event_id: impl Into<String>,
            reservation_id: impl Into<String>,
            customer_id: impl Into<String>,
        ) -> Result<Self, EventDrivenArchitectureError> {
            Self::with_offer(event_id, reservation_id, "", customer_id)
        }

        pub fn with_offer(
            event_id: impl Into<String>,
            reservation_id: impl Into<String>,
            offer_id: impl Into<String>,
            customer_id: impl Into<String>,
        ) -> Result<Self, EventDrivenArchitectureError> {
            let event_id = event_id.into();
            let reservation_id = reservation_id.into();
            let offer_id = offer_id.into();
            let customer_id = customer_id.into();

            if event_id.trim().is_empty()
                || reservation_id.trim().is_empty()
                || customer_id.trim().is_empty()
            {
                return Err(EventDrivenArchitectureError::InvalidContract);
            }

            Ok(Self {
                event_id,
                reservation_id,
                offer_id,
                customer_id,
                contract_version: 1,
            })
        }

        pub fn offer_id(&self) -> &str {
            &self.offer_id
        }
    }

    impl IntegrationEvent for ReservationConfirmed {
        fn contract_name(&self) -> &str {
            "ReservationConfirmed"
        }

        fn contract_version(&self) -> u16 {
            self.contract_version
        }

        fn event_id(&self) -> &str {
            &self.event_id
        }

        fn reservation_id(&self) -> &str {
            &self.reservation_id
        }

        fn customer_id(&self) -> &str {
            &self.customer_id
        }
    }
}

pub mod consumers {
    use super::{
        contracts::{IntegrationEvent, ReservationConfirmed},
        EventDrivenArchitectureError,
    };

    pub trait EventConsumer {
        fn handle(
            &mut self,
            event: &ReservationConfirmed,
        ) -> Result<(), EventDrivenArchitectureError>;
    }

    #[derive(Default)]
    pub struct NotificationConsumer {
        processed_event_ids: Vec<String>,
    }

    impl NotificationConsumer {
        pub fn sent_count(&self) -> usize {
            self.processed_event_ids.len()
        }
    }

    impl EventConsumer for NotificationConsumer {
        fn handle(
            &mut self,
            event: &ReservationConfirmed,
        ) -> Result<(), EventDrivenArchitectureError> {
            if !self
                .processed_event_ids
                .iter()
                .any(|processed| processed == event.event_id())
            {
                self.processed_event_ids.push(event.event_id().to_owned());
            }

            Ok(())
        }
    }

    #[derive(Default)]
    pub struct ReservationAnalyticsConsumer {
        processed_event_ids: Vec<String>,
    }

    impl ReservationAnalyticsConsumer {
        pub fn confirmed_count(&self) -> usize {
            self.processed_event_ids.len()
        }
    }

    impl EventConsumer for ReservationAnalyticsConsumer {
        fn handle(
            &mut self,
            event: &ReservationConfirmed,
        ) -> Result<(), EventDrivenArchitectureError> {
            if !self
                .processed_event_ids
                .iter()
                .any(|processed| processed == event.event_id())
            {
                self.processed_event_ids.push(event.event_id().to_owned());
            }

            Ok(())
        }
    }

    #[derive(Default)]
    pub struct FailingConsumer;

    impl EventConsumer for FailingConsumer {
        fn handle(
            &mut self,
            _event: &ReservationConfirmed,
        ) -> Result<(), EventDrivenArchitectureError> {
            Err(EventDrivenArchitectureError::ConsumerFailed)
        }
    }
}

pub mod bus {
    use super::{
        consumers::EventConsumer, contracts::ReservationConfirmed, EventDrivenArchitectureError,
    };

    #[derive(Default)]
    pub struct InMemoryEventBus {
        published: Vec<ReservationConfirmed>,
    }

    impl InMemoryEventBus {
        pub fn publish(&mut self, event: ReservationConfirmed) -> ReservationConfirmed {
            self.published.push(event.clone());
            event
        }

        pub fn published_count(&self) -> usize {
            self.published.len()
        }

        pub fn published_events(&self) -> &[ReservationConfirmed] {
            &self.published
        }

        pub fn dispatch<'a, I>(
            event: &ReservationConfirmed,
            consumers: I,
        ) -> Result<(), EventDrivenArchitectureError>
        where
            I: IntoIterator<Item = &'a mut dyn EventConsumer>,
        {
            for consumer in consumers {
                consumer.handle(event)?;
            }

            Ok(())
        }
    }
}

pub mod producers {
    use super::{
        bus::InMemoryEventBus, contracts::ReservationConfirmed, EventDrivenArchitectureError,
    };

    pub struct ReservationProducer;

    impl ReservationProducer {
        pub fn confirm_reservation(
            reservation_id: impl Into<String>,
            offer_id: impl Into<String>,
            customer_id: impl Into<String>,
            bus: &mut InMemoryEventBus,
        ) -> Result<ReservationConfirmed, EventDrivenArchitectureError> {
            let reservation_id = reservation_id.into();
            let event_id = format!("reservation-confirmed:{reservation_id}");
            let event =
                ReservationConfirmed::with_offer(event_id, reservation_id, offer_id, customer_id)?;

            Ok(bus.publish(event))
        }
    }
}
