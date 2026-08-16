use clap::Parser;
use edc_identity_hub_client::models::DidWeb;
use edc_identity_hub_client::{IdentityHubClient, Result};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Participant DID
  participant_did: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();
  let client = reqwest::Client::new();

  let participant_did = DidWeb::new(&args.participant_did).expect("Not a DID Web form");

  let identity = IdentityHubClient::get_identity(client, participant_did).await?;

  println!("{identity:#?}");

  for identity_service in identity.services {
    if let Some(issuer_service_client) = identity_service.get_issuer_service_client() {
      let metadata = issuer_service_client.get_metadata().await?;

      let formatted_title = format!("{:-^1$}", " Issuer Service ", 50);
      println!("{formatted_title}");
      println!("{metadata:#?}");
    }

    if let Some(dataspace_service_client) = identity_service.get_dataspace_service_client() {
      let protocol_versions = dataspace_service_client.get_protocol_versions().await?;

      let formatted_title = format!("{:-^1$}", " Dataspace Service ", 50);
      println!("{formatted_title}");
      println!("{protocol_versions:#?}");
    }
  }

  Ok(())
}
