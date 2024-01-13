FROM rust:alpine as builder

WORKDIR /usr/src/oxyweb
COPY . .
RUN RUSTFLAGS='-C target-feature=+crt-static' cargo install --example hello_world --path .

FROM scratch

COPY --from=builder /usr/local/cargo/bin/oxyweb /oxyweb
COPY --from=builder /usr/src/oxyweb/res /res

CMD [ "/oxyweb" ]