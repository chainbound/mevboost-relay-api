use clap::Parser;

#[derive(Parser)]
#[clap(author = "Chainbound")]
struct Args {
    #[clap(long, short = 'r', default_value = "flashbots")]
    relay_name: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _ = tracing_subscriber::fmt::try_init();

    let client = mevboost_relay_api::Client::default();
    if !client.relays.contains_key(args.relay_name.as_str()) {
        anyhow::bail!("Relay `{}` not found in list of relays.", args.relay_name)
    }

    // TODO

    Ok(())
}
