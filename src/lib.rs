pub const COURSE_NAME: &str = "Arquitectura de software en Rust";

pub mod clean_architecture;
pub mod cqrs;
pub mod domain_driven_design;
pub mod event_driven_architecture;
pub mod event_sourcing;
pub mod hexagonal_architecture;
pub mod modular_monolith;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlannedChapter {
    pub number: u8,
    pub slug: &'static str,
    pub title: &'static str,
    pub status: &'static str,
}

pub const PLANNED_CHAPTERS: [PlannedChapter; 8] = [
    PlannedChapter {
        number: 1,
        slug: "monolito-modular",
        title: "Monolito modular",
        status: "planned",
    },
    PlannedChapter {
        number: 2,
        slug: "arquitectura-hexagonal",
        title: "Arquitectura hexagonal",
        status: "planned",
    },
    PlannedChapter {
        number: 3,
        slug: "clean-architecture",
        title: "Clean Architecture",
        status: "planned",
    },
    PlannedChapter {
        number: 4,
        slug: "domain-driven-design",
        title: "Domain-Driven Design",
        status: "planned",
    },
    PlannedChapter {
        number: 5,
        slug: "cqrs",
        title: "CQRS",
        status: "planned",
    },
    PlannedChapter {
        number: 6,
        slug: "event-sourcing",
        title: "Event sourcing",
        status: "planned",
    },
    PlannedChapter {
        number: 7,
        slug: "arquitectura-orientada-a-eventos",
        title: "Arquitectura orientada a eventos",
        status: "planned",
    },
    PlannedChapter {
        number: 8,
        slug: "microservicios",
        title: "Microservicios",
        status: "planned",
    },
];

pub fn chapter_count() -> usize {
    PLANNED_CHAPTERS.len()
}

#[cfg(test)]
mod tests {
    use super::clean_architecture::{
        adapters::{FailingReservationRepository, InMemoryReservationRepository},
        application::{ConfirmReservation, ConfirmReservationRequest},
        domain::ReservationStatus as CleanReservationStatus,
        CleanArchitectureError,
    };
    use super::cqrs::{
        commands::{ConfirmReservation as CqrsConfirmReservation, ReservationWriteModel},
        events::ReservationConfirmed,
        queries::FindConfirmedReservations,
        read_model::{ProjectionLag, ReservationSummaryProjection},
        CqrsError,
    };
    use super::domain_driven_design::{
        domain::{
            CustomerId as DddCustomerId, Money as DddMoney, OfferId as DddOfferId,
            Reservation as DddReservation, ReservationId as DddReservationId,
            ReservationStatus as DddReservationStatus,
        },
        events::DomainEvent,
        repositories::{
            InMemoryReservationRepository as DddReservationRepository, ReservationRepository,
        },
        DomainDrivenDesignError,
    };
    use super::event_driven_architecture::{
        bus::InMemoryEventBus,
        consumers::{
            EventConsumer, FailingConsumer, NotificationConsumer, ReservationAnalyticsConsumer,
        },
        contracts::{IntegrationEvent, ReservationConfirmed as EdaReservationConfirmed},
        producers::ReservationProducer,
        EventDrivenArchitectureError,
    };
    use super::event_sourcing::{
        commands::{
            CancelReservation as EsCancelReservation, ConfirmReservation as EsConfirmReservation,
            RequestReservation,
        },
        events::{ReservationEvent, ReservationEventKind},
        stream::ReservationEventStream,
        EventSourcingError, Reservation, ReservationStatus as EventSourcedReservationStatus,
    };
    use super::hexagonal_architecture::{
        adapters::{FailingReservationStore, InMemoryReservationStore},
        application::{ConfirmBooking, ConfirmBookingCommand},
        domain::ReservationStatus,
        HexagonalArchitectureError,
    };
    use super::modular_monolith::{
        booking::{BookingService, ReservationId},
        inventory::{Capacity, Inventory},
        pricing::{Clock, Money, PricingService},
        ModularMonolithError,
    };

    #[test]
    fn exposes_course_name() {
        assert_eq!(super::COURSE_NAME, "Arquitectura de software en Rust");
    }

    #[test]
    fn exposes_eight_planned_chapters() {
        assert_eq!(super::chapter_count(), 8);
        assert!(super::PLANNED_CHAPTERS
            .iter()
            .all(|chapter| chapter.status == "planned"));
    }

    #[test]
    fn modular_monolith_confirms_reservation_through_internal_contracts() {
        let clock = Clock::fixed(1_000);
        let mut inventory = Inventory::new(Capacity::new(2).expect("valid capacity"));
        let pricing = PricingService::new(clock);
        let quote = pricing
            .quote("flight-mx-001", Money::mxn(1_500), 30)
            .expect("valid quote");

        let reservation = BookingService::confirm(
            ReservationId::new("RSV-001").expect("valid reservation id"),
            quote,
            &mut inventory,
            clock,
        )
        .expect("confirmed reservation");

        assert_eq!(reservation.id().as_str(), "RSV-001");
        assert_eq!(reservation.offer_id(), "flight-mx-001");
        assert_eq!(inventory.available_units(), 1);
    }

    #[test]
    fn modular_monolith_rejects_expired_quote_without_consuming_inventory() {
        let quoted_at = Clock::fixed(1_000);
        let confirmed_at = Clock::fixed(1_031);
        let mut inventory = Inventory::new(Capacity::new(1).expect("valid capacity"));
        let pricing = PricingService::new(quoted_at);
        let quote = pricing
            .quote("flight-mx-002", Money::mxn(900), 30)
            .expect("valid quote");

        let result = BookingService::confirm(
            ReservationId::new("RSV-002").expect("valid reservation id"),
            quote,
            &mut inventory,
            confirmed_at,
        );

        assert_eq!(result, Err(ModularMonolithError::QuoteExpired));
        assert_eq!(inventory.available_units(), 1);
    }

    #[test]
    fn modular_monolith_rejects_overbooking() {
        let clock = Clock::fixed(1_000);
        let mut inventory = Inventory::new(Capacity::new(0).expect("valid capacity"));
        let pricing = PricingService::new(clock);
        let quote = pricing
            .quote("flight-mx-003", Money::mxn(1_200), 30)
            .expect("valid quote");

        let result = BookingService::confirm(
            ReservationId::new("RSV-003").expect("valid reservation id"),
            quote,
            &mut inventory,
            clock,
        );

        assert_eq!(result, Err(ModularMonolithError::InventoryUnavailable));
        assert_eq!(inventory.available_units(), 0);
    }

    #[test]
    fn hexagonal_use_case_confirms_booking_through_output_port() {
        let mut store = InMemoryReservationStore::default();
        let use_case = ConfirmBooking::new(&mut store);

        let reservation = use_case
            .execute(ConfirmBookingCommand::new(
                "RSV-HEX-001",
                "flight-mx-hex",
                "quote-hex-001",
            ))
            .expect("confirmed reservation");

        assert_eq!(reservation.id().as_str(), "RSV-HEX-001");
        assert_eq!(reservation.status(), ReservationStatus::Confirmed);
        assert_eq!(store.saved_count(), 1);
    }

    #[test]
    fn hexagonal_use_case_rejects_invalid_input_before_touching_adapter() {
        let mut store = InMemoryReservationStore::default();
        let use_case = ConfirmBooking::new(&mut store);

        let result = use_case.execute(ConfirmBookingCommand::new(
            "",
            "flight-mx-hex",
            "quote-hex-001",
        ));

        assert_eq!(result, Err(HexagonalArchitectureError::InvalidInput));
        assert_eq!(store.saved_count(), 0);
    }

    #[test]
    fn hexagonal_use_case_surfaces_output_port_failure() {
        let mut store = FailingReservationStore;
        let use_case = ConfirmBooking::new(&mut store);

        let result = use_case.execute(ConfirmBookingCommand::new(
            "RSV-HEX-002",
            "flight-mx-hex",
            "quote-hex-002",
        ));

        assert_eq!(result, Err(HexagonalArchitectureError::OutputPortFailed));
    }

    #[test]
    fn clean_architecture_confirms_reservation_with_entity_and_repository() {
        let mut repository = InMemoryReservationRepository::default();
        let use_case = ConfirmReservation::new(&mut repository);

        let reservation = use_case
            .execute(ConfirmReservationRequest::new(
                "RSV-CLEAN-001",
                "flight-mx-clean",
                "customer-clean-001",
            ))
            .expect("confirmed reservation");

        assert_eq!(reservation.id().as_str(), "RSV-CLEAN-001");
        assert_eq!(reservation.status(), CleanReservationStatus::Confirmed);
        assert_eq!(repository.saved_count(), 1);
    }

    #[test]
    fn clean_architecture_rejects_invalid_request_before_repository() {
        let mut repository = InMemoryReservationRepository::default();
        let use_case = ConfirmReservation::new(&mut repository);

        let result = use_case.execute(ConfirmReservationRequest::new(
            "",
            "flight-mx-clean",
            "customer-clean-001",
        ));

        assert_eq!(result, Err(CleanArchitectureError::InvalidInput));
        assert_eq!(repository.saved_count(), 0);
    }

    #[test]
    fn clean_architecture_surfaces_repository_failure() {
        let mut repository = FailingReservationRepository;
        let use_case = ConfirmReservation::new(&mut repository);

        let result = use_case.execute(ConfirmReservationRequest::new(
            "RSV-CLEAN-002",
            "flight-mx-clean",
            "customer-clean-002",
        ));

        assert_eq!(result, Err(CleanArchitectureError::RepositoryFailed));
    }

    #[test]
    fn ddd_value_objects_reject_invalid_language() {
        assert_eq!(
            DddReservationId::new(""),
            Err(DomainDrivenDesignError::InvalidValue)
        );
        assert_eq!(DddMoney::mxn(0), Err(DomainDrivenDesignError::InvalidValue));
    }

    #[test]
    fn ddd_aggregate_confirms_reservation_and_records_domain_event() {
        let mut reservation = DddReservation::request(
            DddReservationId::new("RSV-DDD-001").expect("valid reservation id"),
            DddOfferId::new("flight-mx-ddd").expect("valid offer id"),
            DddCustomerId::new("customer-ddd-001").expect("valid customer id"),
            DddMoney::mxn(2_100).expect("valid money"),
        );

        let event = reservation.confirm().expect("confirmed reservation");

        assert_eq!(reservation.status(), DddReservationStatus::Confirmed);
        assert_eq!(
            event,
            DomainEvent::ReservationConfirmed {
                reservation_id: "RSV-DDD-001".to_owned()
            }
        );
    }

    #[test]
    fn ddd_cancelled_reservation_cannot_be_confirmed_again() {
        let mut reservation = DddReservation::request(
            DddReservationId::new("RSV-DDD-002").expect("valid reservation id"),
            DddOfferId::new("flight-mx-ddd").expect("valid offer id"),
            DddCustomerId::new("customer-ddd-002").expect("valid customer id"),
            DddMoney::mxn(1_800).expect("valid money"),
        );

        reservation.confirm().expect("confirmed reservation");
        reservation.cancel().expect("cancelled reservation");

        assert_eq!(
            reservation.confirm(),
            Err(DomainDrivenDesignError::InvalidTransition)
        );
        assert_eq!(reservation.status(), DddReservationStatus::Cancelled);
    }

    #[test]
    fn ddd_repository_stores_aggregate_without_owning_rules() {
        let mut reservation = DddReservation::request(
            DddReservationId::new("RSV-DDD-003").expect("valid reservation id"),
            DddOfferId::new("flight-mx-ddd").expect("valid offer id"),
            DddCustomerId::new("customer-ddd-003").expect("valid customer id"),
            DddMoney::mxn(2_400).expect("valid money"),
        );
        reservation.confirm().expect("confirmed reservation");

        let mut repository = DddReservationRepository::default();
        repository
            .save(reservation.clone())
            .expect("saved aggregate");

        assert_eq!(repository.saved_count(), 1);
        assert_eq!(
            repository.find(reservation.id()).expect("stored aggregate"),
            &reservation
        );
    }

    #[test]
    fn cqrs_command_confirms_reservation_and_emits_event() {
        let mut write_model = ReservationWriteModel::default();

        let event =
            CqrsConfirmReservation::new("RSV-CQRS-001", "flight-mx-cqrs", "customer-cqrs-001")
                .execute(&mut write_model)
                .expect("confirmed reservation");

        assert_eq!(
            event,
            ReservationConfirmed {
                reservation_id: "RSV-CQRS-001".to_owned(),
                offer_id: "flight-mx-cqrs".to_owned(),
                customer_id: "customer-cqrs-001".to_owned(),
            }
        );
        assert_eq!(write_model.confirmed_count(), 1);
    }

    #[test]
    fn cqrs_invalid_command_does_not_change_write_model() {
        let mut write_model = ReservationWriteModel::default();

        let result = CqrsConfirmReservation::new("", "flight-mx-cqrs", "customer-cqrs-001")
            .execute(&mut write_model);

        assert_eq!(result, Err(CqrsError::InvalidCommand));
        assert_eq!(write_model.confirmed_count(), 0);
    }

    #[test]
    fn cqrs_projection_builds_read_model_from_events() {
        let mut projection = ReservationSummaryProjection::default();
        projection.apply(ReservationConfirmed {
            reservation_id: "RSV-CQRS-002".to_owned(),
            offer_id: "flight-mx-cqrs".to_owned(),
            customer_id: "customer-cqrs-002".to_owned(),
        });

        let query = FindConfirmedReservations::new(&projection);
        let summaries = query.execute();

        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].reservation_id(), "RSV-CQRS-002");
        assert_eq!(projection.lag(), ProjectionLag::CaughtUp);
    }

    #[test]
    fn cqrs_query_does_not_mutate_projection() {
        let mut projection = ReservationSummaryProjection::default();
        projection.mark_lagging();

        let before = projection.summary_count();
        let lag_before = projection.lag();
        let query = FindConfirmedReservations::new(&projection);
        let _ = query.execute();

        assert_eq!(projection.summary_count(), before);
        assert_eq!(projection.lag(), lag_before);
    }

    #[test]
    fn event_sourcing_appends_events_and_rehydrates_reservation() {
        let mut stream = ReservationEventStream::default();

        let requested = RequestReservation::new("RSV-ES-001", "flight-mx-es", "customer-es-001")
            .execute(&mut stream)
            .expect("requested reservation");
        let confirmed = EsConfirmReservation::new("RSV-ES-001")
            .execute(&mut stream)
            .expect("confirmed reservation");

        assert_eq!(requested.version(), 1);
        assert_eq!(confirmed.version(), 2);
        assert_eq!(stream.len(), 2);

        let reservation = Reservation::rehydrate(stream.events()).expect("rehydrated reservation");

        assert_eq!(reservation.id(), "RSV-ES-001");
        assert_eq!(
            reservation.status(),
            EventSourcedReservationStatus::Confirmed
        );
    }

    #[test]
    fn event_sourcing_rejects_confirmation_without_request() {
        let mut stream = ReservationEventStream::default();

        let result = EsConfirmReservation::new("RSV-ES-002").execute(&mut stream);

        assert_eq!(result, Err(EventSourcingError::ReservationNotRequested));
        assert_eq!(stream.len(), 0);
    }

    #[test]
    fn event_sourcing_rejects_invalid_history_during_rehydration() {
        let invalid_history = vec![ReservationEvent::new(
            1,
            "RSV-ES-003",
            ReservationEventKind::ReservationConfirmed,
        )];

        let result = Reservation::rehydrate(&invalid_history);

        assert_eq!(result, Err(EventSourcingError::InvalidHistory));
    }

    #[test]
    fn event_sourcing_stream_keeps_audit_history_append_only() {
        let mut stream = ReservationEventStream::default();

        RequestReservation::new("RSV-ES-004", "flight-mx-es", "customer-es-004")
            .execute(&mut stream)
            .expect("requested reservation");
        EsCancelReservation::new("RSV-ES-004")
            .execute(&mut stream)
            .expect("cancelled reservation");

        let audit = stream.events();

        assert_eq!(audit.len(), 2);
        assert_eq!(audit[0].kind(), ReservationEventKind::ReservationRequested);
        assert_eq!(audit[1].kind(), ReservationEventKind::ReservationCancelled);
    }

    #[test]
    fn event_driven_producer_publishes_stable_contract() {
        let mut bus = InMemoryEventBus::default();

        let event = ReservationProducer::confirm_reservation(
            "RSV-EDA-001",
            "flight-mx-eda",
            "customer-eda-001",
            &mut bus,
        )
        .expect("published integration event");

        assert_eq!(event.contract_name(), "ReservationConfirmed");
        assert_eq!(event.contract_version(), 1);
        assert_eq!(event.reservation_id(), "RSV-EDA-001");
        assert_eq!(bus.published_count(), 1);
    }

    #[test]
    fn event_driven_bus_fans_out_to_consumers() {
        let event = EdaReservationConfirmed::new("EVT-EDA-002", "RSV-EDA-002", "customer-eda-002")
            .expect("valid event");
        let mut notifications = NotificationConsumer::default();
        let mut analytics = ReservationAnalyticsConsumer::default();

        InMemoryEventBus::dispatch(
            &event,
            [
                &mut notifications as &mut dyn EventConsumer,
                &mut analytics as &mut dyn EventConsumer,
            ],
        )
        .expect("fan-out succeeds");

        assert_eq!(notifications.sent_count(), 1);
        assert_eq!(analytics.confirmed_count(), 1);
    }

    #[test]
    fn event_driven_consumer_is_idempotent() {
        let event = EdaReservationConfirmed::new("EVT-EDA-003", "RSV-EDA-003", "customer-eda-003")
            .expect("valid event");
        let mut notifications = NotificationConsumer::default();

        notifications.handle(&event).expect("first delivery");
        notifications.handle(&event).expect("duplicate delivery");

        assert_eq!(notifications.sent_count(), 1);
    }

    #[test]
    fn event_driven_failing_consumer_does_not_delete_published_event() {
        let mut bus = InMemoryEventBus::default();
        let event = ReservationProducer::confirm_reservation(
            "RSV-EDA-004",
            "flight-mx-eda",
            "customer-eda-004",
            &mut bus,
        )
        .expect("published integration event");
        let mut failing = FailingConsumer;

        let result = InMemoryEventBus::dispatch(&event, [&mut failing as &mut dyn EventConsumer]);

        assert_eq!(result, Err(EventDrivenArchitectureError::ConsumerFailed));
        assert_eq!(bus.published_count(), 1);
    }
}
