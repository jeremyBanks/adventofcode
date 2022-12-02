use std::fmt::Debug;
use std::fmt::Display;
use std::process::ExitCode;
use std::process::Termination;

use clap::Parser;

fn main() -> Result<(), panic> {
    ::dotenv::dotenv().unwrap();
    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var(
            "RUST_LOG",
            format!("warn,advent=trace,{}=trace", env!("CARGO_CRATE_NAME")),
        );
    }

    ::tracing::subscriber::set_global_default(
        ::tracing_subscriber::layer::SubscriberExt::with(
            ::tracing_subscriber::fmt()
                .with_env_filter(::tracing_subscriber::EnvFilter::from_default_env())
                .pretty()
                .with_span_events(::tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .finish(),
            ::tracing_error::ErrorLayer::default(),
        ),
    )?;

    ::color_eyre::install()?;

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("fatal error");

    let args = advent::Args::parse();
    runtime.block_on(advent::main(args));

    Ok(())
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(non_camel_case_types)]
pub enum panic {}

impl<Err> From<Err> for panic
where
    Err: Display + Debug,
{
    #[track_caller]
    fn from(error: Err) -> Self {
        panic!("{error}")
    }
}

impl Termination for panic {
    fn report(self) -> ExitCode {
        unreachable!()
    }
}
