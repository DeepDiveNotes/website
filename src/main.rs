#[macro_use]
extern crate tera;

#[macro_use]
extern crate serde_derive;

mod deep_dive;

use actix_web::{
    error, fs, http, middleware, server, App, HttpRequest, HttpResponse, Query, State,
};
use failure::Error;
use std::collections::HashMap;
use tera::Template;

/// AppState
///
/// Store persistent data
struct AppState {
    template: tera::Tera,
    seasons: Vec<deep_dive::Season>,
}

impl AppState {
    pub fn new(seasons: Vec<deep_dive::Season>) -> Self {
        Self {
            template: compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")),
            seasons,
        }
    }
}

fn main() -> Result<(), Error> {
    ::std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let seasons =
        deep_dive::read_seasons_from_path(concat!(env!("CARGO_MANIFEST_DIR"), "/data/seasons"));

    server::new(move || {
        let state = AppState::new(seasons.clone());
        App::with_state(state)
            .middleware(middleware::Logger::default())
            .resource("/", |r| r.method(http::Method::GET).with(index))
            .resource("/season/{id}", |r| r.method(http::Method::GET).f(season))
            .resource("/season/{season_id}/episode/{episode_id}", |r| {
                r.method(http::Method::GET).f(episode)
            })
            .resource("/search", |r|r.f(search))
            .handler(
                "/static",
                fs::StaticFiles::new(concat!(env!("CARGO_MANIFEST_DIR"), "/static")).unwrap(),
            )
            .finish()
    }).bind("127.0.0.1:80")
        .unwrap()
        .run();

    Ok(())
}

/// Index endpoint
fn index(
    (state, _query): (State<AppState>, Query<HashMap<String, String>>),
) -> Result<HttpResponse, error::Error> {
    let seasons = &state.seasons;
    let templates = &state.template;

    let mut ctx = tera::Context::new();
    ctx.insert("seasons", seasons);

    let body = templates
        .render("index.html", &ctx)
        .map_err(|err| error::ErrorInternalServerError(format!("{:?}", err)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}

/// Endpoint for viewing a season page
fn season(req: &HttpRequest<AppState>) -> Result<HttpResponse, error::Error> {
    let state = req.state();

    let id: usize = req.match_info().get("id").unwrap().parse().map_err(|err| {
        error::ErrorInternalServerError(format!("Could not parse season id {:?}", err))
    })?;
    let id = id - 1; // Front end id's start at 1, but since we use a vector to access, we need to start at 0

    if let Some(season) = state.seasons.get(id) {
        let mut ctx = tera::Context::new();
        ctx.insert("season", season);

        let body = state
            .template
            .render("season.html", &ctx)
            .map_err(|err| error::ErrorInternalServerError(format!("{:?}", err)))?;

        Ok(HttpResponse::Ok().content_type("text/html").body(body))
    } else {
        Err(error::ErrorNotFound(format!(
            "Could not find season with id: {}",
            id
        )))
    }
}

/// Endpoint for viewing an episode page
fn episode(req: &HttpRequest<AppState>) -> Result<HttpResponse, error::Error> {
    let state = req.state();

    let season_id: usize = req
        .match_info()
        .get("season_id")
        .unwrap()
        .parse()
        .map_err(|err| error::ErrorBadRequest(format!("Could not parse season id {:?}", err)))?;
    let episode_id: usize = req
        .match_info()
        .get("episode_id")
        .unwrap()
        .parse()
        .map_err(|err| error::ErrorBadRequest(format!("Could not parse episode id {:?}", err)))?;

    if let Some(season) = state.seasons.get(season_id - 1) {
        if let Some(episode) = season.episodes.get(episode_id - 1) {

            let initial_time = if let Some(time) = req.query().get("t") {
                if let Ok(time) = time.parse::<u32>() {
                    time
                } else {
                    0
                }
            } else {
                0
            };


            let mut ctx = tera::Context::new();
            ctx.insert("season", season);
            ctx.insert("episode", episode);
            ctx.insert("time", &initial_time);

            let body = state
                .template
                .render("episode.html", &ctx)
                .map_err(|err| error::ErrorInternalServerError(format!("{:?}", err)))?;

            Ok(HttpResponse::Ok().content_type("text/html").body(body))
        } else {
            Err(error::ErrorNotFound(format!(
                "Could not find episode with id: {}",
                episode_id
            )))
        }
    } else {
        Err(error::ErrorNotFound(format!(
            "Could not find season with id: {}",
            season_id
        )))
    }
}

fn search(req: &HttpRequest<AppState>) -> Result<HttpResponse, error::Error> {
    use crate::deep_dive::search::Search;

    let state = req.state();

    let results = if let Some(query) = req.query().get("q") {
        let regex = regex::RegexBuilder::new(query).case_insensitive(true).build().map_err(|err| error::ErrorInternalServerError(format!("regex error {:?}", err)))?;

        state.seasons.search(&regex).unwrap()

    } else {
        Vec::new()
    };

    let mut ctx = tera::Context::new();
    ctx.insert("results", &results);

    let body = state.template.render("search_result.html", &ctx).map_err(|err| error::ErrorInternalServerError(format!("{:?}", err)))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(body))
}
