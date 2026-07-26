use common::{init_config, init_tracing, load_config};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    let cfg = load_config()?;
    init_config(cfg);

    muipserver::run(muipserver::MuipOptions::from_config()).await
}
