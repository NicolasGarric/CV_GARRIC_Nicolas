// src/data/cv_en.rs
use crate::data::cv::{
    Certification, Contact, ContactKind, CvData, Education,
    Experience, Header, HobbyLink, IconKind, Language, Project, Skills,
    Hobby,
};

pub fn load() -> CvData {
    CvData {
        header:         header(),
        resume:         resume(),
        experiences:    experiences(),
        education:      education(),
        skills:         skills(),
        languages:      languages(),
        certifications: certifications(),
        hobbies:        hobbies(),
    }
}

fn header() -> Header {
    Header {
        name:  "NICOLAS GARRIC".into(),
        title: "Full Stack Developer | Available in Tokyo from September 2026".into(),
        photo: Some("assets/Pdp_GARRIC_Nicolas.jpg".into()),
        contacts: vec![
            Contact { kind: ContactKind::Phone,    label: "(+33) 6 26 84 94 12".into(),          href: Some("tel:+33626849412".into()) },
            Contact { kind: ContactKind::Email,    label: "nicolasdfgarric@gmail.com".into(),     href: Some("mailto:nicolasdfgarric@gmail.com".into()) },
            Contact { kind: ContactKind::Location, label: "Paris → Tokyo (Sept. 2026)".into(),    href: None },
            Contact { kind: ContactKind::Linkedin, label: "linkedin.com/in/nicolasgarric".into(), href: Some("https://www.linkedin.com/in/nicolasgarric".into()) },
            Contact { kind: ContactKind::Github,   label: "github.com/NicolasGarric".into(),      href: Some("https://github.com/NicolasGarric".into()) },
        ],
    }
}

fn resume() -> String {
    "Full Stack Developer trained at <strong>Le Wagon Paris</strong> and Castor&Pollux, \
     with an atypical background combining <strong>web development</strong> and \
     <strong>international film production</strong> (Cannes Film Festival, Japan–France co-productions). \
     Autonomous, rigorous, and experienced in high-pressure environments with strict deadlines. \
     Available as <strong>freelance (French company)</strong> or on a <strong>permanent contract</strong>. \
     Currently based in France, then <strong>in Tokyo from September 2026</strong> for 4 years.".into()
}

fn experiences() -> Vec<Experience> {
    vec![
        Experience {
            title:       "Full Stack Developer — WordPress".into(),
            company:     "Castor & Pollux".into(),
            contract:    Some("Full Time Freelance".into()),
            dates:       "04/2025 – 10/2025".into(),
            location:    "Montreuil, France".into(),
            description: Some("Web agency — autonomous development within a team of 5 developers".into()),
            bullets: vec![
                "Front-end and back-end development of 4 client websites in production".into(),
                "Advanced integration: Twig, ACF, JS ES6+, Swiper, creative animations".into(),
                "Continuous optimisation of performance, UX and responsive design".into(),
            ],
            projects: vec![
                Project { label: "Fraîcheur de Paris".into(), href: Some("https://www.fraicheurdeparis.fr/".into()) },
                Project { label: "Azura".into(),              href: Some("https://www.azura-snacking.com/".into()) },
                Project { label: "Butagaz".into(),            href: Some("https://www.butagaz.fr/".into()) },
                Project { label: "Bjorg (WIP)".into(),        href: Some("https://www.bjorg.fr/".into()) },
            ],
            icon_kind: IconKind::Web,
        },
        Experience {
            title:       "Full Stack Developer Training".into(),
            company:     "Le Wagon Paris".into(),
            contract:    None,
            dates:       "10/2024 – 12/2024".into(),
            location:    "Paris, France".into(),
            description: Some("Intensive bootcamp — two team projects built in groups of 4".into()),
            bullets: vec![
                "FrogLab — industrial data management mobile app, AGILE methodology, 2 weeks".into(),
                "Full stack Airbnb clone — Ruby on Rails, PostgreSQL, deployed to production".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Train,
        },
        Experience {
            title:       "Production & Post-Production Manager".into(),
            company:     "Urban Factory".into(),
            contract:    Some("Permanent Contract".into()),
            dates:       "2022 – 2023".into(),
            location:    "Paris, France".into(),
            description: Some("International film production company — 4 films selected at the Cannes Film Festival".into()),
            bullets: vec![
                "Led technical workflows from production through post-production".into(),
                "Coordinated cross-functional teams and managed budgets under strict constraints".into(),
                "Handled international co-productions (Japan, Mongolia, Switzerland, Spain)".into(),
            ],
            projects: vec![
                Project { label: "Plan 75 — Chie Hayakawa · Japon/France · Mention spéciale Caméra d'Or, Cannes 2022".into(), href: Some("https://www.imdb.com/title/tt19719904/".into()) },
                Project { label: "Goya, Carrière & The Ghost of Buñuel — J.L. López-Linares · Cannes 2022".into(),            href: Some("https://www.imdb.com/title/tt14018794/".into()) },
                Project { label: "Si seulement je pouvais hiberner — Z. Purevdash · Cannes 2023".into(),                      href: Some("https://www.imdb.com/title/tt21605226/".into()) },
                Project { label: "The Shameless — Konstantin Bojanov · Cannes 2023".into(),                                    href: Some("https://www.imdb.com/title/tt15437986/".into()) },
            ],
            icon_kind: IconKind::Film,
        },
        Experience {
            title:       "Technical Manager".into(),
            company:     "Urban Group".into(),
            contract:    Some("Permanent Contract".into()),
            dates:       "2022 – 2023".into(),
            location:    "Paris, France".into(),
            description: Some("Film group (Urban Factory, Reservoir Docs, Urban Sales, Urban Distribution)".into()),
            bullets: vec![
                "Administered servers and FTP infrastructure for the group".into(),
                "Managed complete video file pipelines (acquisition, processing, archiving, distribution)".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Post,
        },
    ]
}

fn education() -> Vec<Education> {
    vec![
        Education { degree: "Full Stack Developer".into(),                                              school: "Le Wagon Paris".into(),                           dates: "10/2024 – 12/2024".into() },
        Education { degree: "Professional Drone Pilot (CATT)".into(),                                  school: "TELEPILOTE SAS".into(),                           dates: "11/2022 – 12/2022".into() },
        Education { degree: "Master 2 — Cinema & Audiovisual, Cultural Documentary".into(),            school: "Université Lumière Lyon 2".into(),                dates: "2020 – 2022".into() },
        // Education { degree: "Master 1 — Cinema & Audiovisual".into(),                               school: "Université Paul Valéry — Montpellier III".into(), dates: "2019 – 2020".into() },
        // Education { degree: "Bachelor — Audiovisual Director, Documentary specialisation".into(),   school: "ESRA Nice".into(),                                dates: "2016 – 2019".into() },
        // Education { degree: "Bachelor 1 — Management, Accounting, Law".into(),                     school: "TSM Toulouse".into(),                             dates: "2015 – 2016".into() },
        // Education { degree: "Baccalauréat STI2D ITEC".into(),                                      school: "Lycée Riquet, St Orens de Gameville".into(),      dates: "2014 – 2015".into() },
    ]
}

fn skills() -> Skills {
    Skills {
        frontend:    vec!["HTML5".into(), "Twig".into(), "CSS3".into(), "SCSS".into(), "JS ES6+".into(), "Figma".into(), "Bootstrap".into(), "React".into(), "Angular".into()],
        backend:     vec!["Ruby".into(), "Ruby on Rails".into(), "Java".into(), "PHP".into(), "Rust".into(), "AI".into(), "GitLab / GitHub".into(), "PostgreSQL".into(), "Testing".into(), "REST API".into(), "DB Architecture".into()],
        soft_skills: vec!["Problem Solving".into(), "Adaptability".into(), "Communication".into(), "Organisation".into(), "Continuous Learning".into(), "Teamwork".into(), "Collaboration".into()],
    }
}

fn languages() -> Vec<Language> {
    vec![
        Language { name: "French".into(),   level: "Native".into(),    bars: 5 },
        Language { name: "English".into(),  level: "C1".into(),        bars: 4 },
        Language { name: "Spanish".into(),  level: "B1".into(),        bars: 2 },
        Language { name: "Japanese".into(), level: "Beginner".into(),  bars: 1 },
    ]
}

fn certifications() -> Vec<Certification> {
    vec![
        Certification {
            name:   "Web Application Designer & Developer".into(),
            issuer: "RNCP Level 6 (Bachelor+) — France Compétences".into(),
        },
        Certification {
            name:   "Theoretical Aptitude Certificate — Drone Pilot".into(),
            issuer: "French Civil Aviation Authority (DGAC)".into(),
        },
    ]
}

fn hobbies() -> Vec<Hobby> {
    vec![
        Hobby { name: "Photography".into(),        detail: Some("Street · Wildlife".into()),              links: vec![] },
        Hobby { name: "Outdoor sports".into(),     detail: Some("Climbing · Hiking".into()),              links: vec![] },
        Hobby { name: "Gastronomy".into(),         detail: Some("Japanese cuisine · Fermentation".into()), links: vec![] },
        Hobby { name: "Japanese language".into(),  detail: Some("Active learning since Jan. 2026 — hiragana, katakana, daily vocabulary".into()), links: vec![] },
        Hobby {
            name:   "Video game development".into(),
            detail: Some("GDScript (Godot) · Rust — ongoing projects".into()),
            links:  vec![
                HobbyLink { label: "godot-survivor".into(),      href: "https://github.com/NicolasGarric/godot-survivor".into() },
                HobbyLink { label: "godot-knight-jumper".into(), href: "https://github.com/NicolasGarric/godot-knight-jumper".into() },
            ],
        },
    ]
}
