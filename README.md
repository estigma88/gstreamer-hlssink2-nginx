# gstreamer-hlssink2-nginx

This is an example of using hlssink2 and its signals to push the playlist and fragments to an origin, using HTTP. In
this case, the origin is nginx.

# Requirements

1. Ubuntu 22
2. Gstreamer
3. Rust

# Running the test with Cargo

Run `cargo test`

# Running the test with Cargo and Devcontainers

1. Navigate to `.devocontainer` folder
2. Create a Docker image using `docker build . -t gstreamer-hlssink2-nginx`
3. Run the Devcontainer using `devcontainer.json` file with `IntellJ` or `VSCode`
4. Run `cargo test` inside your IDE

# Running the test with IDE and Devcontainers

1. Navigate to `.devocontainer` folder
2. Create a Docker image using `docker build . -t gstreamer-hlssink2-nginx`
3. Run the Devcontainer using `devcontainer.json` file with `IntellJ` or `VSCode`
4. Open the `src/hlssick2_origin.rs` file
5. Click on the test and run it


