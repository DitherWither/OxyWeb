FROM rust:alpine as builder

WORKDIR /usr/src/oxidized-webserver
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --example hello_world --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/oxidized-webserver /oxidized-webserver
COPY --from=builder /usr/src/oxidized-webserver/res /res

CMD [ "/oxidized-webserver" ]