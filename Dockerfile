FROM alpine:latest AS base
ENV TERM xterm-256color
RUN apk add --no-cache rust cargo pkgconfig openssl-dev
WORKDIR /builder
COPY . .

#
# builder state
#
FROM base AS builder
RUN cargo install --path . --root .

#
# test stage
#
FROM base AS test
RUN apk add --no-cache openrc

CMD cd /builder && \
    cargo test

#
# create clean image with chat application only
#
FROM alpine:latest AS runtime
# libgcc is required to run rust applications
RUN apk add --no-cache libgcc

COPY --from=builder /builder/bin/* /usr/local/bin

RUN adduser user -D -G users
USER user

EXPOSE 8080

WORKDIR /usr/local/bin
ENTRYPOINT ["/usr/local/bin/stock_math_api"]
