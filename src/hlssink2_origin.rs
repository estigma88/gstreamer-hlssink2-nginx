use testcontainers::core::IntoContainerPort;

#[cfg(test)]
pub(crate) mod test {
    use std::env;
    use log::Level;
    use gst::Pipeline;
    use gst::prelude::{Cast, ElementExt};
    use testcontainers::core::{Mount, WaitFor};
    use testcontainers::{GenericImage, ImageExt};
    use testcontainers::core::Host::HostGateway;
    use testcontainers::core::logs::consumer::logging_consumer::LoggingConsumer;
    use testcontainers::runners::AsyncRunner;
    use super::*;

    #[tokio::test]
    async fn pipeline_hlssink_nginx() {
        env::set_var("RUST_LOG", "DEBUG");
        env_logger::init();

        let container = GenericImage::new(
            "nginx",
            "1.27.2",
        )
            .with_wait_for(WaitFor::seconds(5))
            .with_mapped_port(80, 80.tcp())
            .with_host("host.docker.internal", HostGateway)
            .with_mount(Mount::volume_mount("nginx.con", "/etc/nginx/nginx.conf:ro"))
            .with_log_consumer(
                LoggingConsumer::new()
                    .with_prefix("nginx -> ")
                    .with_stderr_level(Level::Debug)
                    .with_stdout_level(Level::Debug),
            )
            .start()
            .await
            .unwrap_or_else(|e| panic!("Unable to set ngnix origin container, Error: {}", e));
        //
        // // Initialize gst
        // gst::init().unwrap();
        //
        // // Create the pipeline using a command string
        // let pipeline = gst::parse::launch("videotestsrc ! fakesink").unwrap();
        //
        // // Cast the parsed element to a Pipeline
        // let pipeline = pipeline
        //     .downcast::<Pipeline>()
        //     .expect("Failed to cast to Pipeline");
        //
        // // Start the pipeline
        // pipeline.set_state(gst::State::Playing).unwrap();
        //
        // // Stop the pipeline
        // pipeline.set_state(gst::State::Null).unwrap();
    }
}
