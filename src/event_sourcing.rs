#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EventSourcingError {
    EmptyStream,
    InvalidCommand,
    InvalidHistory,
    ReservationAlreadyRequested,
    ReservationNotRequested,
    ReservationAlreadyClosed,
    StreamReservationMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReservationStatus {
    Requested,
    Confirmed,
    Cancelled,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Reservation {
    id: String,
    offer_id: String,
    customer_id: String,
    status: ReservationStatus,
}

impl Reservation {
    pub fn rehydrate(events: &[events::ReservationEvent]) -> Result<Self, EventSourcingError> {
        let mut reservation: Option<Self> = None;

        for (index, event) in events.iter().enumerate() {
            if event.version() != index + 1 {
                return Err(EventSourcingError::InvalidHistory);
            }

            match event.kind() {
                events::ReservationEventKind::ReservationRequested => {
                    if reservation.is_some() {
                        return Err(EventSourcingError::ReservationAlreadyRequested);
                    }

                    reservation = Some(Self {
                        id: event.reservation_id().to_owned(),
                        offer_id: event
                            .offer_id()
                            .ok_or(EventSourcingError::InvalidHistory)?
                            .to_owned(),
                        customer_id: event
                            .customer_id()
                            .ok_or(EventSourcingError::InvalidHistory)?
                            .to_owned(),
                        status: ReservationStatus::Requested,
                    });
                }
                events::ReservationEventKind::ReservationConfirmed => {
                    let current = reservation
                        .as_mut()
                        .ok_or(EventSourcingError::InvalidHistory)?;
                    if current.id != event.reservation_id() {
                        return Err(EventSourcingError::StreamReservationMismatch);
                    }
                    if current.status == ReservationStatus::Cancelled {
                        return Err(EventSourcingError::ReservationAlreadyClosed);
                    }
                    current.status = ReservationStatus::Confirmed;
                }
                events::ReservationEventKind::ReservationCancelled => {
                    let current = reservation
                        .as_mut()
                        .ok_or(EventSourcingError::InvalidHistory)?;
                    if current.id != event.reservation_id() {
                        return Err(EventSourcingError::StreamReservationMismatch);
                    }
                    if current.status == ReservationStatus::Cancelled {
                        return Err(EventSourcingError::ReservationAlreadyClosed);
                    }
                    current.status = ReservationStatus::Cancelled;
                }
            }
        }

        reservation.ok_or(EventSourcingError::EmptyStream)
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn offer_id(&self) -> &str {
        &self.offer_id
    }

    pub fn customer_id(&self) -> &str {
        &self.customer_id
    }

    pub fn status(&self) -> ReservationStatus {
        self.status
    }
}

pub mod events {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ReservationEventKind {
        ReservationRequested,
        ReservationConfirmed,
        ReservationCancelled,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationEvent {
        version: usize,
        reservation_id: String,
        kind: ReservationEventKind,
        offer_id: Option<String>,
        customer_id: Option<String>,
    }

    impl ReservationEvent {
        pub fn new(
            version: usize,
            reservation_id: impl Into<String>,
            kind: ReservationEventKind,
        ) -> Self {
            Self {
                version,
                reservation_id: reservation_id.into(),
                kind,
                offer_id: None,
                customer_id: None,
            }
        }

        pub fn requested(
            version: usize,
            reservation_id: impl Into<String>,
            offer_id: impl Into<String>,
            customer_id: impl Into<String>,
        ) -> Self {
            Self {
                version,
                reservation_id: reservation_id.into(),
                kind: ReservationEventKind::ReservationRequested,
                offer_id: Some(offer_id.into()),
                customer_id: Some(customer_id.into()),
            }
        }

        pub fn version(&self) -> usize {
            self.version
        }

        pub fn reservation_id(&self) -> &str {
            &self.reservation_id
        }

        pub fn kind(&self) -> ReservationEventKind {
            self.kind
        }

        pub fn offer_id(&self) -> Option<&str> {
            self.offer_id.as_deref()
        }

        pub fn customer_id(&self) -> Option<&str> {
            self.customer_id.as_deref()
        }
    }
}

pub mod stream {
    use super::{events::ReservationEvent, EventSourcingError};

    #[derive(Default)]
    pub struct ReservationEventStream {
        events: Vec<ReservationEvent>,
    }

    impl ReservationEventStream {
        pub fn append(
            &mut self,
            event: ReservationEvent,
        ) -> Result<ReservationEvent, EventSourcingError> {
            if event.version() != self.next_version() {
                return Err(EventSourcingError::InvalidHistory);
            }

            if let Some(first) = self.events.first() {
                if first.reservation_id() != event.reservation_id() {
                    return Err(EventSourcingError::StreamReservationMismatch);
                }
            }

            self.events.push(event.clone());
            Ok(event)
        }

        pub fn len(&self) -> usize {
            self.events.len()
        }

        pub fn is_empty(&self) -> bool {
            self.events.is_empty()
        }

        pub fn events(&self) -> &[ReservationEvent] {
            &self.events
        }

        pub fn next_version(&self) -> usize {
            self.events.len() + 1
        }
    }
}

pub mod commands {
    use super::{
        events::{ReservationEvent, ReservationEventKind},
        stream::ReservationEventStream,
        EventSourcingError, Reservation, ReservationStatus,
    };

    pub struct RequestReservation {
        reservation_id: String,
        offer_id: String,
        customer_id: String,
    }

    impl RequestReservation {
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

        pub fn execute(
            self,
            stream: &mut ReservationEventStream,
        ) -> Result<ReservationEvent, EventSourcingError> {
            if self.reservation_id.trim().is_empty()
                || self.offer_id.trim().is_empty()
                || self.customer_id.trim().is_empty()
            {
                return Err(EventSourcingError::InvalidCommand);
            }

            if !stream.is_empty() {
                return Err(EventSourcingError::ReservationAlreadyRequested);
            }

            stream.append(ReservationEvent::requested(
                stream.next_version(),
                self.reservation_id,
                self.offer_id,
                self.customer_id,
            ))
        }
    }

    pub struct ConfirmReservation {
        reservation_id: String,
    }

    impl ConfirmReservation {
        pub fn new(reservation_id: impl Into<String>) -> Self {
            Self {
                reservation_id: reservation_id.into(),
            }
        }

        pub fn execute(
            self,
            stream: &mut ReservationEventStream,
        ) -> Result<ReservationEvent, EventSourcingError> {
            let reservation = Reservation::rehydrate(stream.events())
                .map_err(|_| EventSourcingError::ReservationNotRequested)?;

            if reservation.id() != self.reservation_id {
                return Err(EventSourcingError::StreamReservationMismatch);
            }

            if reservation.status() == ReservationStatus::Cancelled {
                return Err(EventSourcingError::ReservationAlreadyClosed);
            }

            stream.append(ReservationEvent::new(
                stream.next_version(),
                self.reservation_id,
                ReservationEventKind::ReservationConfirmed,
            ))
        }
    }

    pub struct CancelReservation {
        reservation_id: String,
    }

    impl CancelReservation {
        pub fn new(reservation_id: impl Into<String>) -> Self {
            Self {
                reservation_id: reservation_id.into(),
            }
        }

        pub fn execute(
            self,
            stream: &mut ReservationEventStream,
        ) -> Result<ReservationEvent, EventSourcingError> {
            let reservation = Reservation::rehydrate(stream.events())
                .map_err(|_| EventSourcingError::ReservationNotRequested)?;

            if reservation.id() != self.reservation_id {
                return Err(EventSourcingError::StreamReservationMismatch);
            }

            if reservation.status() == ReservationStatus::Cancelled {
                return Err(EventSourcingError::ReservationAlreadyClosed);
            }

            stream.append(ReservationEvent::new(
                stream.next_version(),
                self.reservation_id,
                ReservationEventKind::ReservationCancelled,
            ))
        }
    }
}
