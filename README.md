A small webservice generating passwords.

Based on Rust and the Rocket web-framework and the password crate passwords. Webpage is made with Askama and htmx.

Build and run the docker-image

```
docker build -t rocket_htmx:dev .
docker run -p 8000:8000 rocket_htmx:dev
```

Open a browser to http://localhost:8000.

Rocket framework: https://rocket.rs/

Password-crate: https://docs.rs/passwords/latest/passwords/index.html

Askama template engine: https://djc.github.io/askama/

Htmx extends html: https://htmx.org/

