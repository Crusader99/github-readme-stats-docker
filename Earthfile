VERSION 0.7

build-all-platforms:
    BUILD --platform=linux/amd64 +build
    BUILD --platform=linux/arm/v7 +build
    BUILD --platform=linux/arm64/v8 +build

build:
    FROM +github-readme-stats
    COPY --chmod=777 +rust/github-readme-stats-docker /app/run
    EXPOSE 80
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
    ARG TARGETPLATFORM
    ARG TARGETOS
    FROM --platform=linux/amd64 busybox
    RUN echo $TARGETPLATFORM
    IF [ "$TARGETPLATFORM" = "linux/amd64" ]
        FROM --platform=linux/amd64 +rust-build --COMPILE_IMAGE_TAG=x86_64-musl
    ELSE IF [ "$TARGETPLATFORM" = "linux/arm/v7" ]
        FROM --platform=linux/amd64 +rust-build --COMPILE_IMAGE_TAG=armv7-musleabihf
    ELSE IF [ "$TARGETPLATFORM" = "linux/arm64" ]
        FROM --platform=linux/amd64 +rust-build --COMPILE_IMAGE_TAG=aarch64-musl
    END
    RUN ls /project/target
    SAVE ARTIFACT /project/target/*-unknown-${TARGETOS}-*/release/github-readme-stats-docker

rust-build:
    ARG COMPILE_IMAGE_TAG=x86_64-musl
    FROM messense/rust-musl-cross:$COMPILE_IMAGE_TAG
    WORKDIR /project
    COPY --dir Cargo.toml src /project/
    RUN cargo build --release
