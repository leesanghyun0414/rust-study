ARG RUST_TAG="1-alpine3.15"

ARG ALPINE_TAG="3.16.1"

ARG WORKDIR="/usr/src/app"



FROM rust:${RUST_TAG} as builder

ARG WORKDIR

WORKDIR ${WORKDIR}

# Copy local files.
COPY . .

# Install Packages.
RUN cargo install --path .



FROM rust:${RUST_TAG} as executer

ARG WORKDIR

WORKDIR ${WORKDIR}

COPY . .


FROM alpine:${ALPINE_TAG} as main

ARG WORKDIR

WORKDIR ${WORKDIR}

RUN apk add --no-cache \
    ca-certificates \
    gcc

COPY --from=builder /usr/local/cargo/bin/app /usr/local/bin/app

CMD ["app"]
