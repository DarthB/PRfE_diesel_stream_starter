# Readme

The Stream depends on a Development Setup with a Postgres Database Server. For those who want to follow along the Install Steps are externalized in Docker Containers. You will need to Install the following software:

1. [Docker Desktop](https://docs.docker.com/get-docker/)
2. [Visual Code](https://code.visualstudio.com/download)
3. [Visual Code Docker Extension](https://code.visualstudio.com/docs/containers/overview)

When you clone this Repository ensure to do the two following steps in the order:

1. Reopen the folder in a container: In Visual Code press Press F1 and type `reopen in container`
2. `docker compose up` the delivered [docker-compose.yml](./docker-compose.yml) file. `Compose up` should be accessible over the context menu when you click the file in the file explorer of Visual Code.

## License

All the code is dual-licensed under either:

- MIT License
- Apache License, Version 2.0

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem.