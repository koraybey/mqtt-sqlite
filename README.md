## Introduction

A tiny library that connects to MQTT broker and stores message payloads to a SQLite database. 

Decided to remove this package from [mqtt-dashboard](https://github.com/koraybey/mqtt-dashboard) because I want to run it independently from GraphQL server and ensure uninterrupted uptime.

## Building and running

```Dockerfile``` contains multi-stage build which first cross-compiles the executables on [cargo-zigbuild](https://github.com/rust-cross/cargo-zigbuild) image, then copies them to the final image. 

When container starts, the executable creates the database file from embedded migrations and reads the device configuration. Both of these files are located in ```shared``` folder, whose location are defined in ```.env``` file.

```shell
docker build -t mqtt-dashboard .
docker run -it -v ./shared:/data/shared --entrypoint=/bin/sh  mqtt-dashboard:latest
```

