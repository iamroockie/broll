use std::time::Instant;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
};
use broll::{Input, Output, core::run_simulations};
use clap::Parser;
use serde_json::json;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let app = Router::new()
        .route("/simulate", post(simulate))
        .with_state(cli.num_simulations)
        .layer(CorsLayer::permissive());

    let addr = format!("0.0.0.0:{}", cli.port);
    println!("Listening on http://{addr}");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn simulate(
    State(num_simulations): State<usize>,
    Query(query): Query<SimulationQuery>,
    body: String,
) -> Result<Json<Output>, Response> {
    let input: Input = serde_json::from_str(&body).map_err(|e| {
        let msg = e.to_string();
        let clean = msg.split(" at line ").next().unwrap_or(&msg);
        return error_json(StatusCode::BAD_REQUEST, clean.to_string());
    })?;

    input
        .validate()
        .map_err(|e| error_json(StatusCode::BAD_REQUEST, e))?;

    let start = Instant::now();

    let input_for_run = input.clone();

    let results = tokio::task::spawn_blocking(move || {
        run_simulations(&input_for_run, query.seed, num_simulations)
    })
    .await
    .map_err(|_| {
        let msg = "Internal error".to_string();
        error_json(StatusCode::INTERNAL_SERVER_ERROR, msg)
    })?;

    let body = Output::new(start.elapsed().as_secs_f64(), &input, &results);

    Ok(Json(body))
}

fn error_json(status: StatusCode, msg: String) -> Response {
    (status, Json(json!({"error": &msg}))).into_response()
}

#[derive(serde::Deserialize)]
struct SimulationQuery {
    seed: Option<u64>,
}

#[derive(Parser)]
#[command(about = "Broll | Poker Bankroll Simulator | HTTP server")]
struct Cli {
    /// Server port
    #[arg(short, long, default_value_t = 3000)]
    port: u16,

    /// Number of simulations
    #[arg(short, long, default_value_t = 30_000)]
    num_simulations: usize,
}
