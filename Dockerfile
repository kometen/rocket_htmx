FROM rust

WORKDIR /usr/src/rocket
COPY . .

RUN cargo install --path .
EXPOSE 8000

CMD ["rocket_htmx"]
