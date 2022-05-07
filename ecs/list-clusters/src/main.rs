use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ecs::{Error, Region};
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    region: String,
}

async fn show_clusters(client: &aws_sdk_ecs::Client) -> Result<(), aws_sdk_ecs::Error> {
    // let resp = client.describe_clusters().send().await?;
    let resp = client.list_clusters().send().await?;

    // let clusters = resp.clusters().unwrap_or_default();
    let clusters = resp.cluster_arns().unwrap_or_default();
    println!("Found {} clusters:", clusters.len());

    for cluster in clusters {
        println!("  ARN:  {}", cluster);
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    let region_provider = RegionProviderChain::first_try(Region::new(args.region))
        .or_default_provider()
        .or_else(Region::new("us-west-2"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = aws_sdk_ecs::Client::new(&shared_config);

    show_clusters(&client).await
}
