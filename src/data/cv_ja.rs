// src/data/cv_ja.rs
use crate::data::cv::{
    Certification, Contact, ContactKind, CurrentProject, CvData, Education,
    Experience, Header, HobbyLink, IconKind, Language, Project, Skills,
    Hobby,
};

pub fn load() -> CvData {
    CvData {
        header:                 header(),
        resume:                 resume(),
        current_projects_label: "進行中のプロジェクト".into(),
        current_projects:       current_projects(),
        experiences:            experiences(),
        education:              education(),
        skills:                 skills(),
        languages:              languages(),
        certifications:         certifications(),
        hobbies:                hobbies(),
    }
}

fn header() -> Header {
    Header {
        name:  "NICOLAS GARRIC".into(),
        title: "フルスタック開発者 | 2026年9月より東京勤務可能".into(),
        photo: Some("assets/Pdp_GARRIC_Nicolas.jpg".into()),
        contacts: vec![
            Contact { kind: ContactKind::Phone,    label: "(+33) 6 26 84 94 12".into(),          href: Some("tel:+33626849412".into()) },
            Contact { kind: ContactKind::Email,    label: "nicolasdfgarric@gmail.com".into(),     href: Some("mailto:nicolasdfgarric@gmail.com".into()) },
            Contact { kind: ContactKind::Location, label: "パリ → 東京（2026年9月）".into(),      href: None },
            Contact { kind: ContactKind::Linkedin, label: "linkedin.com/in/nicolasgarric".into(), href: Some("https://www.linkedin.com/in/nicolasgarric".into()) },
            Contact { kind: ContactKind::Github,   label: "github.com/NicolasGarric".into(),      href: Some("https://github.com/NicolasGarric".into()) },
        ],
    }
}

fn resume() -> String {
    "フルスタック開発者。<strong>Le Wagon Paris</strong>およびCastor&Polluxで研鑽を積み、\
     <strong>Web開発</strong>と<strong>国際映画制作</strong>を融合したキャリアを持つ。\
     <strong>フリーランス</strong>または<strong>正社員</strong>として対応可能 — \
     <strong>2026年9月より東京</strong>。".into()
}

fn current_projects() -> Vec<CurrentProject> {
    vec![
        CurrentProject {
            label:       "portfolio-nicolas-garric.vercel.app/upcoming".into(),
            href:        Some("https://portfolio-nicolas-garric.vercel.app/upcoming".into()),
            description: "進行中のすべてのプロジェクトは、私のポートフォリオサイトでご覧いただけます：".into(),
        },
    ]
}

fn experiences() -> Vec<Experience> {
    vec![
        Experience {
            title:       "個人ポートフォリオ".into(),
            company:     "個人プロジェクト".into(),
            contract:    None,
            dates:       "2026年4月".into(),
            location:    "フランス、パリ".into(),
            description: Some("制作実績、今後のプロジェクト、ブラウザでプレイできるゲーム、メールお問い合わせを掲載した個人ポートフォリオ。".into()),
            bullets: vec![
                "React/TypeScript・Supabaseバックエンド・Rust/WebAssemblyによるゲーム・Vercelデプロイ・FR/EN/JP多言語対応".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Web,
        },
        Experience {
            title:       "フルスタック開発者 — WordPress".into(),
            company:     "Castor & Pollux".into(),
            contract:    Some("フリーランス（フルタイム）".into()),
            dates:       "2025年4月 – 2025年10月".into(),
            location:    "フランス、モントルイユ".into(),
            description: Some("Webエージェンシー — 5名の開発者チームで自律的に開発".into()),
            bullets: vec![
                "4つのクライアントサイトのフロントエンド・バックエンド開発（本番稼働中）".into(),
                "高度なインテグレーション：Twig、ACF、JS ES6+、Swiper、クリエイティブアニメーション".into(),
                "パフォーマンス・UX・レスポンシブデザインの継続的な最適化".into(),
            ],
            projects: vec![
                Project { label: "Fraîcheur de Paris".into(), href: Some("https://www.fraicheurdeparis.fr/".into()) },
                Project { label: "Azura".into(),              href: Some("https://www.azura-snacking.com/".into()) },
                Project { label: "Butagaz".into(),            href: Some("https://www.butagaz.fr/".into()) },
                Project { label: "Bjorg (制作中)".into(),     href: Some("https://www.bjorg.fr/".into()) },
            ],
            icon_kind: IconKind::Web,
        },
        Experience {
            title:       "フルスタック開発者 研修".into(),
            company:     "Le Wagon Paris".into(),
            contract:    None,
            dates:       "2024年10月 – 2024年12月".into(),
            location:    "フランス、パリ".into(),
            description: Some("集中型ブートキャンプ — 4人チームで2つのプロジェクトを開発".into()),
            bullets: vec![
                "FrogLab — AGILEメソッドによる産業データ管理モバイルアプリ、2週間".into(),
                "Airbnbクローン（フルスタック）— Ruby on Rails、PostgreSQL、本番デプロイ済み".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Train,
        },
        Experience {
            title:       "プロダクション・ポストプロダクションマネージャー".into(),
            company:     "Urban Factory".into(),
            contract:    Some("正社員".into()),
            dates:       "2022年 – 2023年".into(),
            location:    "フランス、パリ".into(),
            description: Some("国際映画制作会社 — カンヌ映画祭出品作品4本".into()),
            bullets: vec![
                "プロダクションからポストプロダクションまでの技術ワークフローを統括".into(),
                "部門横断チームの調整と厳格な制約下での予算管理".into(),
                "国際共同制作の管理（日本、モンゴル、スイス、スペイン）".into(),
            ],
            projects: vec![
                Project { label: "PLAN 75 — 早川千絵監督 · 日仏合作 · カンヌ2022カメラ・ドール特別賞".into(), href: Some("https://www.imdb.com/title/tt19719904/".into()) },
                Project { label: "Goya, Carrière & The Ghost of Buñuel — J.L. López-Linares · カンヌ2022".into(),  href: Some("https://www.imdb.com/title/tt14018794/".into()) },
                Project { label: "眠れない夜に — Z. Purevdash · カンヌ2023".into(),                               href: Some("https://www.imdb.com/title/tt21605226/".into()) },
                Project { label: "The Shameless — Konstantin Bojanov · カンヌ2023".into(),                         href: Some("https://www.imdb.com/title/tt15437986/".into()) },
            ],
            icon_kind: IconKind::Film,
        },
        Experience {
            title:       "テクニカルマネージャー".into(),
            company:     "Urban Group".into(),
            contract:    Some("正社員".into()),
            dates:       "2022年 – 2023年".into(),
            location:    "フランス、パリ".into(),
            description: Some("映画グループ（Urban Factory、Reservoir Docs、Urban Sales、Urban Distribution）".into()),
            bullets: vec![
                "グループのサーバーおよびFTPインフラの管理".into(),
                "映像ファイルの完全なパイプライン管理（取得、処理、アーカイブ、配布）".into(),
            ],
            projects: vec![],
            icon_kind: IconKind::Post,
        },
    ]
}

fn education() -> Vec<Education> {
    vec![
        Education { degree: "フルスタック開発者".into(),                                    school: "Le Wagon Paris".into(),             dates: "2024年10月 – 2024年12月".into() },
        Education { degree: "プロフェッショナルドローンパイロット（CATT）".into(),          school: "TELEPILOTE SAS".into(),             dates: "2022年11月 – 2022年12月".into() },
        Education { degree: "修士2号 — 映画・映像学、文化ドキュメンタリー専攻".into(),     school: "リヨン第2大学（Lumière Lyon 2）".into(), dates: "2020年 – 2022年".into() },
    ]
}

fn skills() -> Skills {
    Skills {
        frontend:    vec!["HTML5".into(), "CSS3".into(), "SCSS".into(), "JS ES6+".into(), "TypeScript".into(), "React".into(), "Twig".into(), "WordPress".into(), "Figma".into(), "Bootstrap".into()],
        backend:     vec!["Ruby on Rails".into(), "PHP".into(), "Rust".into(), "WebAssembly".into(), "Supabase".into(), "PostgreSQL".into(), "GitLab / GitHub".into(), "Vercel".into(), "REST API".into(), "Testing".into(), "DB設計".into()],
        soft_skills: vec!["問題解決".into(), "適応力".into(), "コミュニケーション".into(), "組織力".into(), "継続的学習".into(), "チームワーク".into(), "協調性".into()],
    }
}

fn languages() -> Vec<Language> {
    vec![
        Language { name: "フランス語".into(), level: "ネイティブ".into(), bars: 5 },
        Language { name: "英語".into(),       level: "C1".into(),         bars: 4 },
        Language { name: "スペイン語".into(), level: "B1".into(),         bars: 2 },
        Language { name: "日本語".into(),     level: "初級".into(),       bars: 1 },
    ]
}

fn certifications() -> Vec<Certification> {
    vec![
        Certification {
            name:   "Webアプリケーション設計・開発者".into(),
            issuer: "RNCP レベル6（学士相当）— フランス・コンペタンス".into(),
        },
        Certification {
            name:   "ドローンパイロット理論適性証明書".into(),
            issuer: "フランス民間航空総局（DGAC）".into(),
        },
    ]
}

fn hobbies() -> Vec<Hobby> {
    vec![
        Hobby { name: "写真".into(),         detail: Some("ストリート・野生動物".into()),                                                      links: vec![] },
        Hobby { name: "アウトドアスポーツ".into(), detail: Some("クライミング・ハイキング".into()),                                            links: vec![] },
        Hobby { name: "グルメ".into(),        detail: Some("日本料理・発酵食品".into()),                                                        links: vec![] },
        Hobby { name: "日本語学習".into(),    detail: Some("2026年1月より積極的に学習中 — ひらがな、カタカナ、日常語彙".into()),               links: vec![] },
        Hobby {
            name:   "ゲーム開発".into(),
            detail: Some("GDScript (Godot) · Rust — 進行中のプロジェクト".into()),
            links:  vec![
                HobbyLink { label: "godot-survivor".into(),      href: "https://github.com/NicolasGarric/godot-survivor".into() },
                HobbyLink { label: "godot-knight-jumper".into(), href: "https://github.com/NicolasGarric/godot-knight-jumper".into() },
            ],
        },
    ]
}
