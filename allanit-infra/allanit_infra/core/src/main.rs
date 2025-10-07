use tokio::runtime::Runtime;

pub mod core;
pub mod modules;
pub mod services;
pub mod utils;

fn main() {
    let runtime = Runtime::new().expect("tokio runtime");
    if let Err(err) = runtime.block_on(async { tokio_async_runtime().await }) {
        eprintln!("{:#}", err);
        std::process::exit(1);
    }
}

async fn tokio_async_runtime() -> anyhow::Result<(), anyhow::Error> {
    use crate::core::bootstrap::{bootstrap, start_system};

    let boot = bootstrap().await?;
    start_system(boot).await?;
    Ok(())
}
