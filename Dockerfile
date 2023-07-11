FROM rustlang/rust:nightly-alpine3.14 as builder

# Install build dependencies
RUN apk add --no-cache musl-dev

# Create a new empty shell project
RUN USER=root cargo new --bin endpoint
WORKDIR /endpoint

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
RUN rm ./target/release/deps/endpoint*
RUN cargo build --release

# Start a new stage so that the final image does not contain the cached dependencies
FROM alpine:3.14

# Install libgcc (runtime dependency)
RUN apk add --no-cache libgcc

# Create a new user
RUN addgroup -g 1000 actix
RUN adduser -D -s /bin/sh -u 1000 -G actix actix

# Copy the build artifact from the previous stage and set the executable permission
COPY --from=builder /endpoint/target/release/endpoint /usr/local/bin
RUN chown actix:actix /usr/local/bin/endpoint
RUN chmod 755 /usr/local/bin/endpoint

# Switch to the new user
USER actix

# Expose the container port
EXPOSE 8000

# Run the binary
CMD ["/usr/local/bin/endpoint"]
