# gstreamer-hlssink2-nginx

This is an example of using hlssink2 and its signals to push the playlist and fragments to an origin, using HTTP. In
this case, the origin is nginx.

The following are the errors/warnings we see:

```
Callback values: [(GstHlsSink2) (GstHlsSink2) sink, (gchararray) "http://host.docker.internal:80/live/upload/segment_00000.ts"]
Fragment name: (gchararray) "http://host.docker.internal:80/live/upload/segment_00000.ts"

(gstreamer_hlssink2_nginx-2edcb8bdc60e9f14:5635): GStreamer-WARNING **: 19:57:40.083: ../subprojects/gstreamer/gst/gstpad.c:4463:gst_pad_chain_data_unchecked:<mpegtsmux0:sink_65> Got data flow before segment event

(gstreamer_hlssink2_nginx-2edcb8bdc60e9f14:5635): GStreamer-CRITICAL **: 19:57:40.083: gst_segment_to_running_time: assertion 'segment->format == format' failed
```

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


