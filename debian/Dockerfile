FROM rust:1.69 as builder
WORKDIR /rust-docker-web
COPY . ./
RUN cargo build --release

# RUN rm ./target/release/deps/rust_docker_web/*
# CMD ["sh", "-c", "sleep infinity"]

FROM debian:latest
ARG APP=/usr/src/app

# RUN apt-get update \
#     && apt-get install -y libc-bin=2.29 libc6=2.29 \
#     && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /rust-docker-web/target/release/stock_math_api ${APP}/rust-docker-web

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./rust-docker-web"]