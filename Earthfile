VERSION 0.7

build:
    BUILD +rust
    FROM +github-readme-stats
    COPY +rust/github-readme-stats-docker /app/run
    SAVE IMAGE --push docker.io/crusaders/github-readme-stats-docker-ex
    RUN chmod 777 run
    CMD [ "./run" ]
    SAVE IMAGE --push docker.io/crusaders/github-readme-stats-docker

# See https://github.com/anuraghazra/github-readme-stats#on-other-platforms
github-readme-stats:
    FROM node:18-alpine
    WORKDIR /app
    COPY github-readme-stats /app
    RUN npm install express
    EXPOSE 9000
    CMD [ "node", "express.js" ]
    SAVE IMAGE docker.io/crusaders/github-readme-stats-docker-raw

rust: 
    FROM ekidd/rust-musl-builder:stable
    WORKDIR /project
    COPY --dir Cargo.toml src /project/
    RUN cargo build --release
    SAVE ARTIFACT /project/target/x86_64-unknown-linux-musl/release/github-readme-stats-docker AS LOCAL bin