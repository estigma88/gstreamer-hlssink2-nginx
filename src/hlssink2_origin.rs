use testcontainers::core::IntoContainerPort;

#[cfg(test)]
pub(crate) mod test {
    use std::{env, fs};
    use std::path::{Path, PathBuf};
    use std::time::Duration;
    use gio::{Cancellable, File, FileCreateFlags, OutputStream, SocketClient};
    use gio::glib::Value;
    use gio::prelude::{ApplicationExt, FileExt, IOStreamExt, OutputStreamExt, SocketClientExt, SocketConnectableExt, ToValue};
    use gst::Pipeline;
    use gst::prelude::{Cast, ElementExt, GstBinExt, ObjectExt};
    use gst::State::{Null, Playing};
    use log::Level;
    use reqwest::Url;
    use testcontainers::core::{Host, Mount, WaitFor};
    use testcontainers::{GenericImage, ImageExt};
    use testcontainers::core::logs::consumer::logging_consumer::LoggingConsumer;
    use testcontainers::runners::AsyncRunner;
    use super::*;

    #[tokio::test]
    async fn pipeline_hlssink_nginx() {
        env::set_var("RUST_LOG", "DEBUG");
        env_logger::init();

        let relative_path = Path::new("nginx.conf");
        let absolute_path = fs::canonicalize(&relative_path).unwrap();

        let nginx_port = 80;
        let nginx_host = env::var("TEST_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());

        let nginx = GenericImage::new(
            "nginx",
            "1.27.2",
        )
            .with_wait_for(WaitFor::seconds(5))
            .with_mapped_port(nginx_port, nginx_port.tcp())
            .with_host("host.docker.internal", Host::HostGateway)
            .with_mount(Mount::bind_mount(absolute_path.to_str().unwrap(), "/etc/nginx/nginx.conf:ro"))
            .with_log_consumer(
                LoggingConsumer::new()
                    .with_prefix("nginx -> ")
                    .with_stderr_level(Level::Debug)
                    .with_stdout_level(Level::Debug),
            )
            .start()
            .await
            .unwrap_or_else(|e| panic!("Unable to set ngnix origin container, Error: {}", e));

        // Initialize gst
        gst::init().unwrap();

        // Create the pipeline using a command string
        let pipeline_string = format!(
            "videotestsrc is-live=true ! \
              video/x-raw,width=1280,height=720,framerate=30/1 ! \
              x264enc bitrate=1000 ! \
              h264parse ! \
              hlssink2 name=sink location=http://{}:{}/live/upload/segment_%05d.ts playlist-location=http://{}:{}/live/upload/playlist.m3u8 target-duration=1 max-files=5
                ",
            nginx_host,
            nginx_port,
            nginx_host,
            nginx_port,
        );

        let pipeline_element = gst::parse::launch(&pipeline_string).unwrap();

        // Cast the parsed element to a Pipeline
        let pipeline = pipeline_element
            .downcast::<Pipeline>()
            .expect("Failed to cast to Pipeline");

        let hlssink2 = pipeline
            .clone()
            .dynamic_cast::<gst::Bin>()
            .expect("Pipeline is not a GstBin")
            .by_name("sink")
            .expect("Could not find hlssink2 element named 'sink'");

        hlssink2.connect("get-playlist-stream", false, move |values| {
            if let Some(playlist) = values.get(1) {
                println!("Playlist created: {:?}", playlist);

                let parsed_url = Url::parse(&*playlist.get::<String>().unwrap()).unwrap();

                let client = SocketClient::new();

                // Connect to the server
                let connection = client
                    .connect_to_host(
                        format!("{}:{}",
                                parsed_url.host_str().unwrap(),
                                parsed_url.port().unwrap_or(nginx_port)
                        ).as_str(),
                        parsed_url.port().unwrap_or(nginx_port),
                        None::<&Cancellable>
                    )
                    .expect("Failed to connect to socket");

                let output_stream = connection.output_stream();

                output_stream.write(format!("PUT {} HTTP/1.1\n", parsed_url.path()).as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Host: localhost\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Transfer-Encoding: chunked\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Connection: close\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("\r\n".as_bytes(), None::<&Cancellable>).unwrap();

                let value = output_stream.to_value();

                return Some(value);
            }
            None
        });

        hlssink2.connect("get-fragment-stream", false, move |values| {
            println!("Callback values: {:?}", values);
            if let Some(fragment) = values.get(1) {
                println!("Fragment name: {:?}", fragment);

                let parsed_url = Url::parse(&*fragment.get::<String>().unwrap()).unwrap();

                let client = SocketClient::new();

                // Connect to the server
                let connection = client
                    .connect_to_host(
                        format!("{}:{}",
                                parsed_url.host_str().unwrap(),
                                parsed_url.port().unwrap_or(nginx_port)
                        ).as_str(),
                        parsed_url.port().unwrap_or(nginx_port),
                        None::<&Cancellable>
                    )
                    .expect("Failed to connect to socket");

                let output_stream = connection.output_stream();

                output_stream.write(format!("PUT {} HTTP/1.1\n", parsed_url.path()).as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Host: localhost\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Transfer-Encoding: chunked\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Connection: close\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("\r\n".as_bytes(), None::<&Cancellable>).unwrap();

                let value = output_stream.to_value();

                return Some(value);
            }
            None
        });


        hlssink2.connect("delete-fragment", false, move |values| {
            if let Some(fragment) = values.get(1) {
                println!("Fragment removed: {:?}", fragment);

                let parsed_url = Url::parse(&*fragment.get::<String>().unwrap()).unwrap();

                let client = SocketClient::new();

                // Connect to the server
                let connection = client
                    .connect_to_host(
                        format!("{}:{}",
                                parsed_url.host_str().unwrap(),
                                parsed_url.port().unwrap_or(nginx_port)
                        ).as_str(),
                        parsed_url.port().unwrap_or(nginx_port),
                        None::<&Cancellable>
                    )
                    .expect("Failed to connect to socket");

                let output_stream = connection.output_stream();

                output_stream.write(format!("DELETE {} HTTP/1.1\n", parsed_url.path()).as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Host: localhost\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Transfer-Encoding: chunked\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("Connection: close\n".as_bytes(), None::<&Cancellable>).unwrap();
                output_stream.write("\r\n".as_bytes(), None::<&Cancellable>).unwrap();

                let value = output_stream.to_value();

                return Some(value);
            }
            None
        });

        // Start the pipeline
        pipeline.set_state(gst::State::Playing).unwrap();

        tokio::time::sleep(Duration::from_secs(20)).await;

        // Stop the pipeline
        pipeline.set_state(gst::State::Null).expect("Couldn't stop pipeline");
    }
}
