use clap::{Parser, Subcommand, ValueEnum};
use mevboost_relay_api::{
    types::{BuilderBidsReceivedOptions, PayloadDeliveredQueryOptions},
    Client,
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
// #[command(propagate_version = true)]
struct Args {
    /// The subcommand to execute.
    #[clap(subcommand)]
    command: Command,
    /// The output method to use. Default: human readable text.
    #[clap(long, short = 'o', default_value = "human")]
    output_method: OutputMethod,
    /// The path to write the output to. If not provided,
    /// output will be written to stdout.
    #[clap(long, short = 'p')]
    output_path: Option<String>,
}

#[derive(Default, ValueEnum, Clone)]
enum OutputMethod {
    /// Output in human readable format
    #[default]
    Human,
    /// Output in CSV format
    Csv,
    /// Output in JSON format, to a file path or
    /// to stdout if no file path is provided.
    Json,
}

#[derive(Subcommand)]
enum Command {
    /// Get the payloads delivered to proposers for a given slot.
    #[clap(name = "payloads-delivered")]
    PayloadsDelivered { slot: u64 },

    /// Get the block bids received by the relay for a given slot.
    #[clap(name = "block-bids")]
    BlockBids {
        #[clap(long)]
        slot: Option<u64>,
        #[clap(long)]
        block_hash: Option<String>,
    },

    /// Get the timestamp of the winning bid for a given slot.
    #[clap(name = "winning-bid-timestamp")]
    WinningBidTimestamp { slot: u64 },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let _ = tracing_subscriber::fmt::try_init();

    let client = Client::default();

    println!();
    if !matches!(args.output_method, OutputMethod::Human) {
        todo!("Only human output is currently supported")
    }

    match args.command {
        Command::PayloadsDelivered { slot } => {
            let payloads = client
                .get_payloads_delivered_bidtraces_on_all_relays(&PayloadDeliveredQueryOptions {
                    slot: Some(slot),
                    ..Default::default()
                })
                .await?;

            println!("{:#?}", payloads);
        }

        Command::BlockBids { slot, block_hash } => {
            let block_bids = client
                .get_builder_blocks_received_on_all_relays(&BuilderBidsReceivedOptions {
                    slot,
                    block_hash,
                    ..Default::default()
                })
                .await?;

            println!("{:#?}", block_bids);
        }

        Command::WinningBidTimestamp { slot } => {
            let payloads = client
                .get_payloads_delivered_bidtraces_on_all_relays(&PayloadDeliveredQueryOptions {
                    slot: Some(slot),
                    ..Default::default()
                })
                .await?;

            for (relay, relay_payloads) in payloads {
                if relay_payloads.is_empty() {
                    continue;
                }

                let block_hash = relay_payloads[0].block_hash.clone();
                let block_bids = client
                    .get_builder_blocks_received(
                        relay,
                        &BuilderBidsReceivedOptions {
                            slot: Some(slot),
                            block_hash: Some(block_hash),
                            ..Default::default()
                        },
                    )
                    .await?;

                let timestamp = block_bids[0].timestamp_ms;
                println!(
                    "The winning bid for slot {} was submitted to {} at: {}",
                    slot, relay, timestamp
                )
            }
        }
    }

    println!();
    Ok(())
}
