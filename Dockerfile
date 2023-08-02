# Select a Rust version
FROM rust:latest
LABEL authors="bonfaceZane"

# Create a new empty shell project
RUN USER=root cargo new --bin myapp
WORKDIR /myapp

# Copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# This build step will cache your dependencies
RUN cargo build --release
RUN rm ./backend/src/*.rs

# Now that we've built our dependencies, copy the actual source code
COPY ./backend/src ./src

# Build for release.
# Our previous `cargo build` step built dependencies only,
# and this step uses the cached dependencies.
RUN rm ./target/release/deps/myapp*
RUN cargo build --release

# Port to expose (change to your liking)
EXPOSE 8000

# Command to start the service
CMD ["./target/release/myapp"]