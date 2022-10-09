FROM rust:1.64-buster

WORKDIR /usr/src
COPY . .
ENV API_KEY=<paste_api_key_here>
ENV API_SEC=<paste_api_secret_here>

CMD ["cargo", "test"]