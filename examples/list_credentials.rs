use clap::{Parser, ValueEnum};
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
  /// Endpoint of the OpenID Token endpoint
  #[arg(
    short,
    long,
    default_value = "https://sso.dataspace.luminvent.com/realms/connector-a/protocol/openid-connect/token"
  )]
  openid_token_endpoint: String,
  /// OAuth2 Client ID to authenticate
  #[arg(short, long)]
  client_id: String,
  /// OAuth2 Client Secret to authenticate
  #[arg(long)]
  client_secret: String,
  /// Participant Context ID
  #[arg(long)]
  participant_context_id: String,
  /// OAuth2 profiles
  #[arg(short, long, default_value = "identity-api:read")]
  scopes: String,
  /// Version of the Identity Hub API
  #[arg(long, value_enum, default_value_t = ApiVersion::V1Alpha)]
  api_version: ApiVersion,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum ApiVersion {
  #[value(name = "v1alpha")]
  V1Alpha,
  #[value(name = "v1beta")]
  V1Beta,
  #[value(name = "v1")]
  V1,
}

impl From<ApiVersion> for IdentityHubClientVersion {
  fn from(version: ApiVersion) -> Self {
    match version {
      ApiVersion::V1Alpha => IdentityHubClientVersion::V1Alpha,
      ApiVersion::V1Beta => IdentityHubClientVersion::V1Beta,
      ApiVersion::V1 => IdentityHubClientVersion::V1,
    }
  }
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

  let token = get_authentication_token(&args.openid_token_endpoint, &query_body)
    .await
    .expect("Could not authenticate client");

  let client = reqwest::Client::new();

  let identity_hub_client =
    IdentityHubClient::new(client, args.endpoint, Some(token), args.api_version.into());

  match identity_hub_client
    .get_credentials(&args.participant_context_id)
    .await
  {
    Ok(credentials) => {
      println!("{} verifiable credential(s) found", credentials.len());

      for credential in credentials {
        let claims = &credential.verifiable_credential.credential;

        let formatted_title = format!("{:-^1$}", format!(" {} ", credential.id), 50);
        println!("{formatted_title}");
        println!("Issuer:      {}", credential.issuer_id);
        println!("Holder:      {}", credential.holder_id);
        println!("Format:      {}", credential.verifiable_credential.format);
        println!("Types:       {}", claims.r#type.join(", "));
        if let Some(name) = &claims.name {
          println!("Name:        {name}");
        }
        if let Some(description) = &claims.description {
          println!("Description: {description}");
        }
        println!("Issued:      {}", claims.issuance_date);
        println!("Expires:     {}", claims.expiration_date);
        println!("Subject(s):");
        for subject in &claims.credential_subject {
          println!(
            "{}",
            serde_json::to_string_pretty(subject).unwrap_or_else(|_| subject.to_string())
          );
        }
      }
    }
    Err(IdentityHubClientError::Response(response)) => {
      println!("{:?}", response);
      println!("{:?}", response.text().await?);
    }
    Err(IdentityHubClientError::Reqwest(error)) => {
      println!("{:?}", error);
    }
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
  openid_token_endpoint: &str,
  query_body: &QueryBody,
) -> Option<String> {
  let client = reqwest::Client::new();

  let response = client
    .post(openid_token_endpoint)
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
