use crate::components::common::link::LinkSimple;
use icondata as i;
use leptos::*;
use leptos_icons::*;

pub enum Icon {
    Rust,
    Leptos,
    Axum,
    Astro,
    Svelte,
    PostgreSQL,
    InfluxDB,
    TailwindCSS,
    Flutter,
    ExpressIf,
    Vercel,
    Docker,
    Kubernetes,
    Linux,
    Python,
    Grafana,
}

impl Icon {
    pub fn to_view(&self) -> impl IntoView {
        match self {
            Icon::Rust => view! {
                <a href="https://astro.build/" title="Rust" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiRust />
                </a>
            },
            Icon::Leptos => view! {
                <a title="ExpressIf" href="https://astro.build/" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiEspressif />
                </a>
            },
            Icon::Axum => view! {
                <a href="https://github.com/tokio-rs/axum" title="Axum" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiRust />
                </a>
            },
            Icon::Astro => view! {
                <a href="https://astro.build/" title="Astro" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiAstro />
                </a>
            },
            Icon::Svelte => view! {
                <a href="https://svelte.dev/" title="Svelte" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiSvelte />
                </a>
            },
            Icon::PostgreSQL => view! {
                <a href="https://astro.build/" title="PostgreSQL" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::BiPostgresql />
                </a>
            },
            Icon::InfluxDB => view! {
                <a href="https://www.influxdata.com/" title="InfluxDB" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiInfluxdb />
                </a>
            },
            Icon::TailwindCSS => view! {
                <a href="https://astro.build/" title="TailwindCSS" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiTailwindcss />
                </a>
            },
            Icon::Flutter => view! {
                <a href="https://astro.build/" title="Flutter" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiFlutter />
                </a>
            },
            Icon::ExpressIf => view! {
                <a href="https://astro.build/" title="ExpressIf" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiEspressif />
                </a>
            },
            Icon::Vercel => view! {
                <a href="https://astro.build/" title="Vercel" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiVercel />
                </a>
            },
            Icon::Docker => view! {
                <a href="https://astro.build/" title="Docker" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiDocker />
                </a>
            },
            Icon::Kubernetes => view! {
                <a href="https://astro.build/" title="Kubernetes" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiKubernetes />
                </a>
            },
            Icon::Linux => view! {
                <a href="https://astro.build/" title="GNU/Linux" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiLinux />
                </a>
            },
            Icon::Python => view! {
                <a href="https://astro.build/" title="Python" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiPython />
                </a>
            },
            Icon::Grafana => view! {
                <a href="https://astro.build/" title="Grafana" rel="noopener" class="text-gray-700 hover:text-[#68b5fc]" target="_blank">
                    <Icon width="1.3em" height="1.3em" icon=i::SiGrafana />
                </a>
            },
        }
    }
}

pub enum Link {
    Rust,
    Axum,
    Actix,
    Tokio,
    SQLx,
    GraphQL,
    Argon2,
    Vercel,
    SpaceX,
    Nothing,
    Apple,
    Leptos,
    Guille,
    Nio,
}

impl Link {
    pub fn to_view(&self) -> impl IntoView {
        match self {
            Link::Guille => view! {
                <LinkSimple title="Guillermo Rauch".to_string()
                    link="https://rauchg.com/about".to_string()
                    blank=true
                />
            },
            Link::Rust => view! {
                <LinkSimple title="Rust".to_string()
                    link="https://www.rust-lang.org/".to_string()
                    blank=true
                />
            },
            Link::Leptos => view! {
                <LinkSimple title="Leptos".to_string()
                    link="https://www.leptos.dev/".to_string()
                    blank=true
                />
            },
            Link::Axum => view! {
                <LinkSimple title="Axum".to_string()
                    link="https://github.com/tokio-rs/axum".to_string()
                    blank=true
                />
            },
            Link::Actix => view! {
                <LinkSimple title="Actix".to_string()
                    link="https://actix.rs/".to_string()
                    blank=true
                />
            },
            Link::Tokio => view! {
                <LinkSimple title="Tokio".to_string()
                    link="https://tokio.rs/".to_string()
                    blank=true
                />
            },
            Link::SQLx => view! {
                <LinkSimple title="SQLx".to_string()
                    link="https://crates.io/crates/sqlx".to_string()
                    blank=true
                />
            },
            Link::GraphQL => view! {
                <LinkSimple title="async-graphql".to_string()
                    link="https://crates.io/crates/async-graphql".to_string()
                    blank=true
                />
            },
            Link::Argon2 => view! {
                <LinkSimple title="Argon2".to_string()
                    link="https://en.wikipedia.org/wiki/Argon2".to_string()
                    blank=true
                />
            },
            Link::Vercel => view! {
                <LinkSimple title="Vercel".to_string()
                    link="https://vercel.com/home".to_string()
                    blank=true
                />
            },
            Link::SpaceX => view! {
                <LinkSimple title="SpaceX".to_string()
                    link="https://www.spacex.com/".to_string()
                    blank=true
                />
            },
            Link::Nothing => view! {
                <LinkSimple title="Nothing".to_string()
                    link="https://us.nothing.tech/".to_string()
                    blank=true
                />
            },
            Link::Apple => view! {
                <LinkSimple title="Apple".to_string()
                    link="https://developer.apple.com/design/human-interface-guidelines/".to_string()
                    blank=true
                />
            },
            Link::Nio => view! {
                <LinkSimple title="Nio".to_string()
                    link="https://www.nio.com/ep9".to_string()
                    blank=true
                />
            },
        }
    }
}
