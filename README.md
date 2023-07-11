# Containerizing a Rust Actix server with Alpine 14MB

## Optimize release in Cargo.toml

```toml
[profile.release]
lto = true
strip = true
opt-level = 3
panic = 'abort'
codegen-units = 1
```

## Create Dockerfile
```Dockerfile
FROM rustlang/rust:nightly-alpine3.14 as builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create a new empty shell project
RUN USER=root cargo new --bin actix-docker
WORKDIR /actix-docker

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# Copy your source tree
COPY ./src ./src

# Build for release. 
# This will uninstall the dummy dependencies and install the real ones
RUN rm ./target/release/deps/actix_docker*
RUN cargo build --release

# Start a new stage so that the final image does not contain the cached dependencies
FROM alpine:3.14

# Install libgcc (runtime dependency)
RUN apk add --no-cache libgcc

# Create a new user
RUN addgroup -g 1000 actix
RUN adduser -D -s /bin/sh -u 1000 -G actix actix

# Copy the build artifact from the previous stage and set the executable permission
COPY --from=builder /actix-docker/target/release/actix-docker /usr/local/bin
RUN chown actix:actix /usr/local/bin/actix-docker
RUN chmod 755 /usr/local/bin/actix-docker

# Switch to the new user
USER actix

# Expose the container port
EXPOSE 8000

# Run the binary
CMD ["/usr/local/bin/actix-docker"]

```

```sh
docker build -t actix-web .
```
