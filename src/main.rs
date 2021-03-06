use clap::Parser;

fn main() {
    dotenv::dotenv().unwrap();
    color_eyre::install().unwrap();
    tracing_subscriber::util::SubscriberInitExt::init(tracing_subscriber::Layer::with_subscriber(
        tracing_error::ErrorLayer::default(),
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .with_target(false)
            .with_span_events(
                tracing_subscriber::fmt::format::FmtSpan::NEW
                    | tracing_subscriber::fmt::format::FmtSpan::CLOSE,
            )
            .finish(),
    ));

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("fatal error");

    let args = advent::Args::parse();
    runtime.block_on(advent::main(args))
}
