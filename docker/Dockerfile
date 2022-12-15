ARG RUST_TAG="alpine3.16"

ARG ALPINE_TAG="3.16.1"

ARG WORKDIR="/usr/app"


FROM rust:${RUST_TAG} as dev

ARG WORKDIR

WORKDIR ${WORKDIR}

RUN apk add --no-cache \
    alpine-sdk

# Install Packages.
RUN cargo install cargo-watch

EXPOSE 8080

# Copy local files.
COPY . .


FROM dev as build-stg

ARG WORKDIR

WORKDIR ${WORKDIR}

RUN cargo build --release

#RUN apk add --no-cache \
#    alpine-sdk
#
#COPY . .


FROM alpine:${ALPINE_TAG} as prod

ARG WORKDIR

WORKDIR ${WORKDIR}

#RUN apk add --no-cache \
#    ca-certificates \
#    gcc

COPY --from=build-stg /usr/app/target/release/app .

EXPOSE 8088

CMD ["/usr/app/app"]
