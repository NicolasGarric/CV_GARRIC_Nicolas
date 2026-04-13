// src/data/cv.rs
use serde::{Deserialize, Serialize};

// ══════════════════════════════════════════════════════
//  Structures de données
// ══════════════════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentProject {
    pub label:       String,
    pub href:        Option<String>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvData {
    pub header:                   Header,
    pub resume:                   String,
    pub current_projects_label:   String,
    pub current_projects:         Vec<CurrentProject>,
    pub experiences:              Vec<Experience>,
    pub education:                Vec<Education>,
    pub skills:                   Skills,
    pub languages:                Vec<Language>,
    pub certifications:           Vec<Certification>,
    pub hobbies:                  Vec<Hobby>,
}

impl CvData {
    pub fn load() -> Self {
        Self {
            header:                 Header::default(),
            resume:                 resume(),
            current_projects_label: "Projets en cours".into(),
            current_projects:       current_projects(),
            experiences:            experiences(),
            education:              education(),
            skills:                 Skills::default(),
            languages:              languages(),
            certifications:         certifications(),
            hobbies:                hobbies(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub name:     String,
    pub title:    String,
    pub photo:    Option<String>,
    pub contacts: Vec<Contact>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            name:  "NICOLAS GARRIC".into(),
            title: "Développeur Full Stack | Disponible à Tokyo dès septembre 2026".into(),
            photo: Some("assets/Pdp_GARRIC_Nicolas.jpg".into()),
            contacts: vec![
                Contact { kind: ContactKind::Phone,    label: "(+33) 6 26 84 94 12".into(),          href: Some("tel:+33626849412".into()) },
                Contact { kind: ContactKind::Email,    label: "nicolasdfgarric@gmail.com".into(),     href: Some("mailto:nicolasdfgarric@gmail.com".into()) },
                Contact { kind: ContactKind::Location, label: "Paris → Tokyo (sept. 2026)".into(),    href: None },
                Contact { kind: ContactKind::Linkedin, label: "linkedin.com/in/nicolasgarric".into(), href: Some("https://www.linkedin.com/in/nicolasgarric".into()) },
                Contact { kind: ContactKind::Github,   label: "github.com/NicolasGarric".into(),      href: Some("https://github.com/NicolasGarric".into()) },
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub kind:  ContactKind,
    pub label: String,
    pub href:  Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ContactKind { Phone, Email, Location, Linkedin, Github }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub label: String,
    pub href:  Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub title:       String,
    pub company:     String,
    pub contract:    Option<String>,
    pub dates:       String,
    pub location:    String,
    pub description: Option<String>,
    pub bullets:     Vec<String>,
    pub projects:    Vec<Project>,
    pub icon_kind:   IconKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum IconKind { Web, Train, Film, Post, Drone }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub degree: String,
    pub school: String,
    pub dates:  String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skills {
    pub frontend:    Vec<String>,
    pub backend:     Vec<String>,
    pub soft_skills: Vec<String>,
}

impl Default for Skills {
    fn default() -> Self {
        Self {
            frontend:    vec!["HTML5".into(), "CSS3".into(), "SCSS".into(), "JS ES6+".into(), "TypeScript".into(), "React".into(), "Twig".into(), "WordPress".into(), "Figma".into(), "Bootstrap".into()],
            backend:     vec!["Ruby on Rails".into(), "PHP".into(), "Rust".into(), "WebAssembly".into(), "Supabase".into(), "PostgreSQL".into(), "GitLab / GitHub".into(), "Vercel".into(), "API REST".into(), "Testing".into(), "Architecture BDD".into()],
            soft_skills: vec!["Résolution de problèmes".into(), "Adaptabilité".into(), "Communication".into(), "Organisation".into(), "Apprentissage Continu".into(), "Travail en Équipe".into(), "Collaboration".into()],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub name:  String,
    pub level: String,
    pub bars:  usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certification {
    pub name:   String,
    pub issuer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hobby {
    pub name:   String,
    pub detail: Option<String>,
    pub links:  Vec<HobbyLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HobbyLink {
    pub label: String,
    pub href:  String,
}

// ══════════════════════════════════════════════════════
//  Données du CV — modifiez ici uniquement
// ══════════════════════════════════════════════════════

fn resume() -> String {
    "Développeur Full Stack avec un parcours alliant <strong>développement web</strong> et \
     <strong>production cinématographique internationale</strong>. \
     Disponible en <strong>freelance</strong> ou en <strong>CDI</strong> — \
     <strong>à Tokyo dès septembre 2026</strong>.".into()
}

fn current_projects() -> Vec<CurrentProject> {
    vec![
        CurrentProject {
            label:       "portfolio-nicolas-garric.vercel.app/upcoming".into(),
            href:        Some("https://portfolio-nicolas-garric.vercel.app/upcoming".into()),
            description: "Tous mes projets en cours sont disponibles sur mon site portfolio :".into(),
        },
    ]
}

fn experiences() -> Vec<Experience> {
    vec![
        Experience {
            title:       "Portfolio personnel".into(),
            company:     "Projet personnel".into(),
            contract:    None,
            dates:       "04/2026".into(),
            location:    "Paris, France".into(),
            description: Some("Portfolio personnel avec sites réalisés, projets à venir, jeux jouables dans le navigateur et contact mail.".into()),
            bullets: vec![
                "React/TypeScript, backend Supabase, jeux en Rust/WebAssembly, déployé sur Vercel avec internationalisation FR/EN/JP".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Web,
        },
        Experience {
            title:       "Développeur Full Stack — WordPress".into(),
            company:     "Castor & Pollux".into(),
            contract:    Some("Freelance Full Time".into()),
            dates:       "04/2025 – 10/2025".into(),
            location:    "Montreuil, France".into(),
            description: Some("Agence web — développement en autonomie au sein d'une équipe de 5 développeurs".into()),
            bullets: vec![
                "Développement front-end et back-end de 4 sites clients en production".into(),
                "Intégration avancée : Twig, ACF, JS ES6+, Swiper, animations créatives".into(),
                "Optimisation des performances, UX et responsive design en continu".into(),
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
            title:       "Formation — Développeur Full Stack".into(),
            company:     "Le Wagon Paris".into(),
            contract:    None,
            dates:       "10/2024 – 12/2024".into(),
            location:    "Paris, France".into(),
            description: Some("Bootcamp intensif — développement de deux projets en équipe de 4".into()),
            bullets: vec![
                "FrogLab — application mobile de gestion industrielle, méthode AGILE, 2 semaines".into(),
                "Clone Airbnb full stack — Ruby on Rails, PostgreSQL, déployé en production".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Train,
        },
        Experience {
            title:       "Chargé de Production et Post-Production".into(),
            company:     "Urban Factory".into(),
            contract:    Some("CDI".into()),
            dates:       "2022 – 2023".into(),
            location:    "Paris, France".into(),
            description: Some("Société de production internationale — 4 films sélectionnés au Festival de Cannes".into()),
            bullets: vec![
                "Pilotage des workflows techniques de production à post-production".into(),
                "Coordination d'équipes pluridisciplinaires et gestion budgétaire sous contraintes strictes".into(),
                "Gestion de co-productions internationales (Japon, Mongolie, Suisse, Espagne)".into(),
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
            title:       "Responsable Technique".into(),
            company:     "Urban Group".into(),
            contract:    Some("CDI".into()),
            dates:       "2022 – 2023".into(),
            location:    "Paris, France".into(),
            description: Some("Groupe cinématographique (Urban Factory, Reservoir Docs, Urban Sales, Urban Distribution)".into()),
            bullets: vec![
                "Administration des serveurs et infrastructure FTP du groupe".into(),
                "Gestion complète des flux de fichiers vidéo (acquisition, traitement, archivage, diffusion)".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Post,
        },
    ]
}

fn education() -> Vec<Education> {
    vec![
        Education { degree: "Développeur Full Stack".into(),                                                                     school: "Le Wagon Paris".into(),                           dates: "10/2024 – 12/2024".into() },
        Education { degree: "Télépilote de drone professionnel (CATT)".into(),                                                   school: "TELEPILOTE SAS".into(),                           dates: "11/2022 – 12/2022".into() },
        Education { degree: "Master 2 — Cinéma et Audiovisuel, Documentaire Culturel".into(),                                    school: "Université Lumière Lyon 2".into(),                dates: "2020 – 2022".into() },
        // Education { degree: "Master 1 — Cinéma et Audiovisuel".into(),                                                           school: "Université Paul Valéry — Montpellier III".into(), dates: "2019 – 2020".into() },
        // Education { degree: "Licence — Réalisateur Audiovisuel, Spécialisation Documentaire".into(),                             school: "ESRA Nice".into(),                                dates: "2016 – 2019".into() },
        // Education { degree: "Licence 1 — Gestion, Comptabilité, Management, Droit".into(),                                      school: "TSM Toulouse".into(),                             dates: "2015 – 2016".into() },
        // Education { degree: "Baccalauréat STI2D ITEC".into(),                                                                    school: "Lycée Riquet, St Orens de Gameville".into(),      dates: "2014 – 2015".into() },
    ]
}

fn languages() -> Vec<Language> {
    vec![
        Language { name: "Français".into(),  level: "Natif".into(),    bars: 5 },
        Language { name: "Anglais".into(),   level: "C1".into(),       bars: 4 },
        Language { name: "Espagnol".into(),  level: "B1".into(),       bars: 2 },
        Language { name: "Japonais".into(),  level: "Débutant".into(), bars: 1 },
    ]
}

fn certifications() -> Vec<Certification> {
    vec![
        Certification {
            name:   "Concepteur Développeur d'Applications Web".into(),
            issuer: "RNCP (RNCP39261), Niveau 6 — France Compétences".into(),
        },
        Certification {
            name:   "Certificat d'Aptitude Théorique aux fonctions de Télépilote".into(),
            issuer: "Direction Générale de l'Aviation Civile (DGAC)".into(),
        },
    ]
}

fn hobbies() -> Vec<Hobby> {
    vec![
        Hobby { name: "Photographie".into(),          detail: Some("Street · Animalière".into()),              links: vec![] },
        Hobby { name: "Sports de plein air".into(),   detail: Some("Escalade · Randonnée".into()),             links: vec![] },
        Hobby { name: "Gastronomie".into(),           detail: None,                                             links: vec![] },
        Hobby { name: "Apprentissage du japonais".into(), detail: Some("Hiragana & Katakana · Préparation JLPT N5".into()), links: vec![] },
        Hobby { name: "Développement de jeux vidéos".into(), detail: Some("GDScript, Rust".into()),                       links: vec![] },
    ]
}
