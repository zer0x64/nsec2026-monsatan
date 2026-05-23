use axum::{Router, routing::get};
use clap::Parser;
use rand::seq::SliceRandom;

/// Hardcoded list of quotes served by the API.
const QUOTES: &[&str] = &[
    "At Monsatan, we don't modify nature. We improve upon its first draft. – CEO Brenda Vyle, 2025 Shareholders Gala",
    "Every seed we sell is a promise: a promise that you will buy next year's seeds too. – Monsatan Seed Division Motto",
    "Our carbon footprint is net zero. We checked ourselves and we're happy to report everything is fine. – Monsatan Annual Sustainability Report",
    "Some call them 'weeds'. We call them 'unlicensed crop competitors'. – Monsatan Legal & Herbicide Division",
    "The forest was inefficient. We replaced it with something 40% more photosynthetically optimal. – Monsatan Reforestation Initiative™",
    "We believe farmers deserve freedom — the freedom to choose from our full range of Monsatan-approved Monsatan products. – Monsatan Farmer Partnership Program",
    "Our bees are fine. The wild ones were a liability anyway. – Monsatan Pollinator Solutions™ press release",
    "Feeding the world, one patent at a time. – Monsatan corporate tagline, revised 2024",
    "We didn't invent hunger. We just found a way to monetize the solution. – internal Monsatan strategy memo, leaked 2023, denied 2023",
    "Think of our GMO crops as open-source — except closed, proprietary, and legally enforced. – Monsatan DevRel Team",
];

#[derive(Parser)]
struct Cli {
    #[clap(long, default_value = "[::1]:3001")]
    address: std::net::SocketAddr,
}

/// Handler for GET /quote — returns a random quote as plain text.
async fn random_quote() -> &'static str {
    let mut rng = rand::thread_rng();
    QUOTES
        .choose(&mut rng)
        .copied()
        .unwrap_or("No quotes available.")
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let app = Router::new().route("/quote", get(random_quote));

    let listener = tokio::net::TcpListener::bind(cli.address)
        .await
        .expect("Failed to bind to port 3001");

    println!(
        "Quote of the Day server listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.expect("Server error");
}
