use std::path::Path;

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
    output: OutputMethod,
    /// The path to write the output to. If not provided,
    /// will default to the current working directory.
    #[clap(long, short = 'p')]
    path: Option<String>,
}

#[derive(Default, ValueEnum, Clone)]
enum OutputMethod {
    /// Output in human readable format
    #[default]
    Human,
    /// Output in CSV format
    Csv,
    /// Output in JSON format
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

    let mut output_file_path = args
        .path
        .map(Into::into)
        .unwrap_or(std::env::current_dir()?.join("output"));

    match args.command {
        Command::PayloadsDelivered { slot } => {
            let payloads = client
                .get_payloads_delivered_bidtraces_on_all_relays(&PayloadDeliveredQueryOptions {
                    slot: Some(slot),
                    ..Default::default()
                })
                .await?;

            match args.output {
                OutputMethod::Human => println!("{:#?}", &payloads),
                OutputMethod::Csv => unimplemented!(),
                OutputMethod::Json => {
                    output_file_path = output_file_path
                        .join("payloads-delivered")
                        .join(format!("{}.json", slot));
                    write_json(output_file_path.clone(), payloads)?;
                }
            }
        }

        Command::BlockBids { slot, block_hash } => {
            if slot.is_none() && block_hash.is_none() {
                anyhow::bail!("Must provide either a slot or block hash");
            }

            let block_bids = client
                .get_builder_blocks_received_on_all_relays(&BuilderBidsReceivedOptions {
                    slot,
                    block_hash: block_hash.clone(),
                    ..Default::default()
                })
                .await?;

            let query_name = if let Some(slot) = slot {
                format!("slot-{}", slot)
            } else {
                format!(
                    "block-hash-{}",
                    block_hash
                        .as_ref()
                        .unwrap()
                        .chars()
                        .take(8)
                        .collect::<String>()
                )
            };
            output_file_path = output_file_path.join(format!("block-bids-{}", query_name));

            match args.output {
                OutputMethod::Human => println!("{:#?}", &block_bids),
                OutputMethod::Csv => unimplemented!(),
                OutputMethod::Json => {
                    for (relay, bids) in block_bids {
                        if bids.is_empty() {
                            continue;
                        }

                        let filename = format!("{}.json", relay);
                        println!("Writing {} bids to {}", bids.len(), filename);
                        write_json(output_file_path.join(filename), bids)?;
                    }
                }
            }
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

    Ok(())
}

#[allow(unused)]
fn write_csv<T: serde::Serialize>(path: impl AsRef<Path>, data: Vec<T>) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut res = csv::Writer::from_path(path)?;
    for row in data {
        res.serialize(row)?;
    }
    res.flush()?;
    Ok(())
}

#[allow(unused)]
fn write_json<T: serde::Serialize>(path: impl AsRef<Path>, data: T) -> anyhow::Result<()> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut res = std::fs::File::create(path)?;
    serde_json::to_writer_pretty(&mut res, &data)?;
    Ok(())
}
