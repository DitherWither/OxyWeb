# oxidized-webserver

Web server in rust with zero dependencies/crates

It uses TCP/IP directly

## Building

To build a podman image, run
```bash
scripts/build.sh
```
and it should generate an image called `oxidized-webserver:latest`. Docker can be used instead, you need to note that instead of `Dockerfile`, this project uses `Containerfile` for its manifest

This means that you need to adjust the docker command to use that instead (i don't have docker installed on my current pc, so i can't test this

## Build/Run locally

Just use `cargo run` to run and `cargo build` to build

## Configuration

It runs at port 8080 by default, can be configured by the `PORT` environment variable
