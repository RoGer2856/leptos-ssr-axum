mod cli;
mod utils;

use axum::{extract::State, routing::get, Router};
use axum_helpers::app::AxumApp;
use boardgame_design::app::{shell, App};
use clap::Parser;
use cli::{Cli, Commands};
use leptos::prelude::*;
use leptos_axum::LeptosRoutes;

use crate::log_location;

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Serve => run_server().await,
    }
}

fn api_routes() -> Router {
    Router::new()
        .route(
            "/",
            get(|State(state): State<&'static str>| async move { state }),
        )
        .with_state("Hello World!")
}

fn routes(conf: &ConfFile) -> Router {
    let leptos_options = conf.leptos_options.clone();
    let routes = leptos_axum::generate_route_list(App);

    Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options)
        .nest("/api", api_routes())
}

async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("starting application in server mode");

    let conf = get_configuration(None)?;
    let addr = conf.leptos_options.site_addr;

    let mut app = AxumApp::new(routes(&conf));
    if let Err(e) = app.spawn_server(addr).await {
        log::info!(
            "{}, could not listen on address = {addr}, error = {e:?}",
            log_location!()
        );
    }

    app.join().await;

    Ok(())
}
