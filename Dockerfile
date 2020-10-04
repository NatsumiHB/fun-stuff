FROM rust:1.46-slim

WORKDIR /srv/random-website-redirect
COPY . .

RUN cargo install --path .

EXPOSE 5001
CMD ["random-website-redirect"]