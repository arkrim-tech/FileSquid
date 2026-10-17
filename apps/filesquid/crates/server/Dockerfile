# filesquid-server image: the FileSquid API plus the default FileSquid UI.
# Build from the repository root:  docker build -f apps/filesquid/crates/server/Dockerfile .
# Do not use this image for the Tauri desktop app.

FROM node:22-bookworm-slim AS ui
WORKDIR /src/apps/filesquid/ui
COPY apps/filesquid/ui/package.json apps/filesquid/ui/package-lock.json ./
RUN npm ci
COPY apps/filesquid/ui ./
COPY apps/filesquid/crates/api/openapi.json /src/apps/filesquid/crates/api/openapi.json
RUN npx vite build

FROM rust:1-bookworm AS build
WORKDIR /src/apps/filesquid
COPY apps/filesquid/Cargo.toml apps/filesquid/Cargo.lock apps/filesquid/rust-toolchain.toml ./
COPY apps/filesquid/crates ./crates
RUN cargo build --release --locked -p filesquid-server

FROM debian:bookworm-slim
RUN useradd --system --create-home filesquid
COPY --from=build /src/apps/filesquid/target/release/filesquid-server /usr/local/bin/filesquid-server
COPY --from=ui /src/apps/filesquid/ui/dist /srv/filesquid-ui
USER filesquid
ENV FILESQUID_BIND=0.0.0.0:8083
ENV FILESQUID_UI_DIR=/srv/filesquid-ui
EXPOSE 8083
CMD ["filesquid-server"]
