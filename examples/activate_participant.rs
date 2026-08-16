use clap::Parser;
use edc_identity_hub_client::{
  IdentityHubClient, IdentityHubClientError, IdentityHubClientVersion, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
  /// Endpoint of the Identity Hub service
  #[arg(
    short,
    long,
    default_value = "https://identity-hub.participant-li1.demo.luminvent.com"
  )]
  endpoint: String,
  /// Endpoint of the OpenID endpoint
  #[arg(
    short,
    long,
    default_value = "https://sso.participant-li1.demo.luminvent.com/realms/connector-a/protocol/openid-connect/token"
  )]
  openid_configuration_endpoint: String,
  /// OAuth2 Client ID to authenticate
  #[arg(short, long)]
  client_id: String,
  /// OAuth2 Client Secret to authenticate
  #[arg(long)]
  client_secret: String,
  /// Participant Context ID
  #[arg(long)]
  participant_context_id: String,
  /// Create an Active participant
  #[arg(long, default_value = "true")]
  active: bool,
  /// OAuth2 profiles
  #[arg(
    short,
    long,
    default_value = "issuer-admin-api:write issuer-admin-api:read identity-api:write identity-api:read"
  )]
  scopes: String,
}

#[tokio::main]
async fn main() -> Result<()> {
  let args = Args::parse();

  let query_body = QueryBody {
    grant_type: "client_credentials".to_string(),
    client_id: args.client_id,
    client_secret: args.client_secret,
    scope: args.scopes,
  };

  let token = get_authentication_token(&args.openid_configuration_endpoint, &query_body)
    .await
    .expect("Could not authenticate client");

  println!("{token}");

  let client = reqwest::Client::new();

  let identity_hub_client = IdentityHubClient::new(
    client,
    args.endpoint,
    Some(token),
    IdentityHubClientVersion::V1Alpha,
  );

  if let Err(error) = identity_hub_client
    .activate_participant(&args.participant_context_id, args.active)
    .await
  {
    match error {
      IdentityHubClientError::Response(response) => {
        println!("{:?}", response);
        println!("{:?}", response.text().await?);
      }
      IdentityHubClientError::Reqwest(error) => {
        println!("{:?}", error);
      }
    }
  } else {
    println!("Participant activated");
  }

  Ok(())
}

#[derive(Debug, Serialize)]
struct QueryBody {
  grant_type: String,
  client_id: String,
  client_secret: String,
  scope: String,
}

async fn get_authentication_token(
  openid_configuration_endpoint: &str,
  query_body: &QueryBody,
) -> Option<String> {
  let client = reqwest::Client::new();

  let response = client
    .post(openid_configuration_endpoint)
    .form(&query_body)
    .send()
    .await
    .ok()?;
  if response.status().is_success() {
    #[derive(Debug, Deserialize)]
    struct TokenResponse {
      access_token: String,
    }

    response
      .json::<TokenResponse>()
      .await
      .ok()
      .map(|token_response| token_response.access_token)
  } else {
    None
  }
}
