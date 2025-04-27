FROM rust:latest AS builder

WORKDIR /usr/src/rocket_htmx
COPY Cargo.toml .
COPY Rocket.toml .
COPY src ./src
COPY site ./site
COPY templates ./templates

RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM debian:12-slim

COPY --from=builder /usr/src/rocket_htmx/target/release/rocket_htmx /app/

# Copy other necessary folders
COPY --from=builder /usr/src/rocket_htmx/Rocket.toml /app
COPY --from=builder /usr/src/rocket_htmx/site /app/site
COPY --from=builder /usr/src/rocket_htmx/templates /app/templates

# Ensure the executable is runnable
RUN chmod +x /app/rocket_htmx

EXPOSE 8000

# Run the application
WORKDIR /app
CMD ["/app/rocket_htmx"]
