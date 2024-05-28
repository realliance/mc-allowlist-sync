FROM docker.io/rust:1.78-slim as BUILDER

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libpq-dev openssl libssl-dev libclang-dev llvm

ADD . .

RUN --mount=type=cache,target=/app/target cargo install --locked --root install --path .

RUN ldd /app/install/bin/mc-oauth-allowlist
RUN ldd /app/install/bin/mc-oauth-allowlist | grep "/" | cut -d '>' -f 2 | cut -d '(' -f 1 | while read -r line ; do cp $line /app/install/bin/; echo "$line"; done;

FROM gcr.io/distroless/cc

COPY --from=BUILDER /app/install/bin /app/

ENV LD_LIBRARY_PATH="/app/"

CMD ["/app/mc-oauth-allowlist"]
