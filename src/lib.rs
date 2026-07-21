pub const COURSE_NAME: &str = "Arquitectura de software en Rust";

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
}
