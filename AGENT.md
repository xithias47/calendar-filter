# Project Context: calendar-filter

## Project Overview

`calendar-filter` is a lightweight web service written in Rust designed to proxy and filter iCalendar (`.ics`) feeds. Its primary function is to fetch a calendar from an upstream URL and filter out events that are marked as "Transparent" (i.e., "Free" time), effectively returning a calendar that only contains "Busy" events.

This is useful for privacy or for consolidating calendars where you only want to show actual commitments.

### Core Technologies

*   **Language:** Rust (Edition 2024)
*   **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
*   **Async Runtime:** [Tokio](https://tokio.rs/)
*   **HTTP Client:** [Reqwest](https://github.com/seanmonstar/reqwest)
*   **ICS Parsing:** [icalendar](https://github.com/hoodie/icalendar-rs)
*   **Containerization:** Docker & Docker Compose

## Application Logic

The application exposes a single endpoint:

`GET /calendar.ics?url=<UPSTREAM_URL>`

1.  **Fetch:** It makes an HTTP GET request to the `url` provided in the query string.
2.  **Parse:** The response body is parsed as an iCalendar stream.
3.  **Filter:** It iterates through the components. Any `VEVENT` containing `TRANSP:TRANSPARENT` is discarded.
4.  **Response:** The remaining components are reconstructed into a new calendar and returned with `Content-Type: text/calendar`.

## Building and Running

### Local Development

Ensure you have Rust and Cargo installed.

```bash
# Run the server locally
cargo run
```

The server will start on `0.0.0.0:3000` by default.

### Docker

The project includes a multi-stage `Dockerfile` and a `docker-compose.yml` for easy deployment.

**Using Docker Compose:**

```bash
docker-compose up --build
```

**Manual Docker Build:**

```bash
docker build -t calendar-filter .
docker run -p 3000:3000 calendar-filter
```

## Key Files

*   `src/main.rs`: Contains the entire application logic, including the HTTP handler and filtering logic.
*   `Cargo.toml`: Defines dependencies and package metadata.
*   `Dockerfile`: Multi-stage build (uses `rust:1-bookworm` for building and `debian:bookworm-slim` for the runtime).
*   `docker-compose.yml`: Orchestrates the container, mapping port 3000.

## Configuration

*   **Port:** The application listens on port `3000` by default. This can be overridden by setting the `PORT` environment variable.

## Development Notes

*   **Error Handling:** The project uses `anyhow` for internal error management, which is mapped to a generic `500 Internal Server Error` response for the client.
*   **Filtering Implementation:** The filtering logic currently relies on checking the string representation of the event for "TRANSP:TRANSPARENT". This is a robust fallback for handling loose ICS standards but might be refined in the future to use stricter property parsing if needed.
