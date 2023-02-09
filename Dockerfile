FROM rust:alpine as builder
RUN apk add --no-cache musl-dev upx clang mold openssl

WORKDIR /build
COPY . .
RUN RUSTFLAGS="-C linker=clang -C link-arg=-fuse-ld=/usr/bin/mold" cargo build --release --bin ssps-workshop-bot && \
    upx --best --lzma ./target/release/ssps-workshop-bot

FROM alpine

WORKDIR /app

RUN addgroup -S workshop-bot && adduser -S workshop-bot -G workshop-bot
COPY --from=builder /build/target/release/ssps-workshop-bot /app/ssps-workshop-bot
RUN chown workshop-bot:workshop-bot /app -R
USER workshop-bot:workshop-bot

LABEL org.opencontainers.image.source = "https://github.com/ssps-kb/workshop-bot"

ENTRYPOINT ["/app/ssps-workshop-bot"]
