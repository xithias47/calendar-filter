use axum::{
    extract::Query,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use icalendar::{Calendar, Component}; // Component trait might be needed for .push()
use serde::Deserialize;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/calendar.ics", get(handler));

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct Params {
    url: String,
}

async fn handler(Query(params): Query<Params>) -> Result<Response, AppError> {
    let upstream_ics = reqwest::get(&params.url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to fetch upstream ICS: {}", e))?
        .text()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to read upstream body: {}", e))?;

    let calendar = Calendar::from_str(&upstream_ics)
        .map_err(|e| anyhow::anyhow!("Failed to parse ICS: {}", e))?;

    // Filter out TRANSPARENT events
    let mut new_calendar = Calendar::new();
    for component in calendar.components {
        // Check if it's an event and verify TRANSP property
        // We use string representation check as a reliable fallback since 
        // icalendar crate API property access can be tricky without docs.
        // Format of TRANSP is: "TRANSP:TRANSPARENT" on a line.
        let is_free = if let icalendar::CalendarComponent::Event(event) = &component {
            // Check for strict "TRANSP:TRANSPARENT" line or occurrence
            // Using .to_string() on the event component to check its content.
            // This avoids depending on internal API of `icalendar` crate.
            let s = event.to_string();
            s.contains("TRANSP:TRANSPARENT")
        } else {
            false
        };

        if !is_free {
             new_calendar.push(component);
        }
    }

    // Preserve PRODID checks or just let new_calendar have default.
    // The upstream usually has a PRODID, we probably want to keep it or just use ours.
    // For now, default implementation of Calendar::new() is fine.
    
    Ok((
        [(axum::http::header::CONTENT_TYPE, "text/calendar")],
        new_calendar.to_string(),
    )
        .into_response())
}

// Basic error handling
struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
