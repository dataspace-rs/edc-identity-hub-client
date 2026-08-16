use clap::Parser;
use edc_identity_hub_client::models::{IdentityService, IdentityServiceType, ParticipantContext};
use edc_identity_hub_client::{
  IdentityHubClient, IdentityHubClientError, IdentityHubClientVersion, Result,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
  /// Endpoint of the OpenID endpoint
  #[arg(short, long, default_value = "http://vault:8200")]
  vault_url: String,
  /// OAuth2 Client ID to authenticate
  #[arg(short, long)]
  client_id: String,
  /// OAuth2 Client Secret to authenticate
  #[arg(long)]
  client_secret: String,
  /// OAuth2 Client ID to issue credentials
  #[arg(short, long)]
  issue_client_id: String,
  /// OAuth2 Client Secret to issue
  #[arg(long)]
  issue_client_secret: String,
  /// Participant DID
  #[arg(long)]
  participant_id: String,
  /// Participant Context ID
  #[arg(long)]
  participant_context_id: String,
  /// Create an Active participant
  #[arg(long)]
  active: bool,
  /// Role(s) of the participant
  #[arg(long)]
  roles: Vec<String>,
  /// Service endpoint(s) of the participant
  service_endpoint: Vec<String>,
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

  let mut identity_services = vec![];

  for service_endpoint in args.service_endpoint {
    if service_endpoint.is_empty() {
      continue;
    }

    if service_endpoint.contains(",https://") {
      if let Some(identity_service_type) = service_endpoint.split(',').next() {
        identity_services.push(IdentityService {
          id: Uuid::new_v4().to_string(),
          r#type: IdentityServiceType::Custom(identity_service_type.to_string()),
          service_endpoint: service_endpoint.replace(&format!("{identity_service_type},"), ""),
        });
      } else {
        unreachable!();
      }
    } else {
      if let Some(identity_service) = IdentityService::new(service_endpoint).await {
        identity_services.push(identity_service);
      }
    }
  }

  let participant_context = ParticipantContext::new(
    args.participant_id,
    args.participant_context_id,
    args.active,
    identity_services,
    args.roles,
    Some(serde_json::json!({
      "edc.vault.hashicorp.config": {
        "credentials": {
          "clientId": args.issue_client_id,
          "clientSecret": args.issue_client_secret,
          "tokenUrl": args.openid_configuration_endpoint
        },
        "config": {
          "secretPath": "v1/participants",
          "folderPath": "issuer",
          "vaultUrl": args.vault_url
        }
      }
    })),
  );

  if let Err(error) = identity_hub_client
    .create_participant(&participant_context)
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
    println!("Participant created");
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
