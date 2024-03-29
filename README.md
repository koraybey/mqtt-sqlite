## Introduction

A tiny library that connects to MQTT broker and stores message payloads to a SQLite database. 

Decided to remove this package from [mqtt-dashboard](https://github.com/koraybey/mqtt-dashboard) because I want to run it independently from GraphQL server and ensure uninterrupted uptime.

## Building and running

```Dockerfile``` contains multi-stage build which first cross-compiles the executables on [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) image, then copies them to the final image. 

When container starts, the executable creates the database file from embedded migrations and reads the device configuration. Both of these files are located in ```shared``` folder, whose location are defined as docker run arguments.

```shell
docker build -t mqtt-sqlite .
docker run -di -v ./shared:/data/shared -e DATABASE_URL=shared/database.sqlite -e CONFIG_URL=shared/configuration.json mqtt-sqlite:latest
```

