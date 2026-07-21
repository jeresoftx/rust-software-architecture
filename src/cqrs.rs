#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CqrsError {
    InvalidCommand,
}

pub mod events {
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationConfirmed {
        pub reservation_id: String,
        pub offer_id: String,
        pub customer_id: String,
    }
}

pub mod commands {
    use super::{events::ReservationConfirmed, CqrsError};

    #[derive(Clone, Debug, Eq, PartialEq)]
    struct ConfirmedReservation {
        reservation_id: String,
        offer_id: String,
        customer_id: String,
    }

    #[derive(Default)]
    pub struct ReservationWriteModel {
        confirmed: Vec<ConfirmedReservation>,
    }

    impl ReservationWriteModel {
        pub fn confirmed_count(&self) -> usize {
            self.confirmed.len()
        }

        fn confirm(
            &mut self,
            command: ConfirmReservation,
        ) -> Result<ReservationConfirmed, CqrsError> {
            if command.reservation_id.trim().is_empty()
                || command.offer_id.trim().is_empty()
                || command.customer_id.trim().is_empty()
            {
                return Err(CqrsError::InvalidCommand);
            }

            let event = ReservationConfirmed {
                reservation_id: command.reservation_id,
                offer_id: command.offer_id,
                customer_id: command.customer_id,
            };

            self.confirmed.push(ConfirmedReservation {
                reservation_id: event.reservation_id.clone(),
                offer_id: event.offer_id.clone(),
                customer_id: event.customer_id.clone(),
            });

            Ok(event)
        }
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ConfirmReservation {
        reservation_id: String,
        offer_id: String,
        customer_id: String,
    }

    impl ConfirmReservation {
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
            write_model: &mut ReservationWriteModel,
        ) -> Result<ReservationConfirmed, CqrsError> {
            write_model.confirm(self)
        }
    }
}

pub mod read_model {
    use super::events::ReservationConfirmed;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum ProjectionLag {
        CaughtUp,
        Lagging,
    }

    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct ReservationSummary {
        reservation_id: String,
        offer_id: String,
        customer_id: String,
    }

    impl ReservationSummary {
        pub fn reservation_id(&self) -> &str {
            &self.reservation_id
        }

        pub fn offer_id(&self) -> &str {
            &self.offer_id
        }

        pub fn customer_id(&self) -> &str {
            &self.customer_id
        }
    }

    pub struct ReservationSummaryProjection {
        summaries: Vec<ReservationSummary>,
        lag: ProjectionLag,
    }

    impl Default for ReservationSummaryProjection {
        fn default() -> Self {
            Self {
                summaries: Vec::new(),
                lag: ProjectionLag::CaughtUp,
            }
        }
    }

    impl ReservationSummaryProjection {
        pub fn apply(&mut self, event: ReservationConfirmed) {
            self.summaries.push(ReservationSummary {
                reservation_id: event.reservation_id,
                offer_id: event.offer_id,
                customer_id: event.customer_id,
            });
            self.lag = ProjectionLag::CaughtUp;
        }

        pub fn mark_lagging(&mut self) {
            self.lag = ProjectionLag::Lagging;
        }

        pub fn lag(&self) -> ProjectionLag {
            self.lag
        }

        pub fn summary_count(&self) -> usize {
            self.summaries.len()
        }

        pub fn summaries(&self) -> &[ReservationSummary] {
            &self.summaries
        }
    }
}

pub mod queries {
    use super::read_model::{ReservationSummary, ReservationSummaryProjection};

    pub struct FindConfirmedReservations<'a> {
        projection: &'a ReservationSummaryProjection,
    }

    impl<'a> FindConfirmedReservations<'a> {
        pub fn new(projection: &'a ReservationSummaryProjection) -> Self {
            Self { projection }
        }

        pub fn execute(&self) -> Vec<ReservationSummary> {
            self.projection.summaries().to_vec()
        }
    }
}
