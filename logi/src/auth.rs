use oauth2::{
  basic::BasicClient, reqwest::http_client, AuthUrl, AuthorizationCode, ClientId,
  ClientSecret, CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use std::env;
use dotenv::dotenv;
use tiny_http::{Response, Server};
use url::Url;

pub fn get_google_token() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
  dotenv().ok(); // .env 로드

  let client_id = ClientId::new(env::var("GOOGLE_CLIENT_ID")?);
  let client_secret = ClientSecret::new(env::var("GOOGLE_SECRET")?);

  let auth_url = AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string())?;
  let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string())?;

  let redirect_url = RedirectUrl::new("http://localhost:8080".to_string())?;

  let client = BasicClient::new(client_id, Some(client_secret), auth_url, Some(token_url))
      .set_redirect_uri(redirect_url.clone());

  let (auth_url, _csrf_token) = client
      .authorize_url(CsrfToken::new_random)
      .add_scope(Scope::new("https://www.googleapis.com/auth/drive.file".to_string()))
      .url();

  println!("🔑 Open this URL in your browser:\n\n{}\n", auth_url);
  open::that(auth_url.as_str())?;

  // 🚀 작은 서버를 띄워서 인증 코드를 받음
  let server = Server::http("0.0.0.0:8080")?;
  println!("📡 Waiting for Google OAuth redirect...");

  let request = server.recv()?;
  let query = request.url();
  let url = Url::parse(&format!("http://localhost:8080{}", query))?;

  let code_pair = url
      .query_pairs()
      .find(|pair| pair.0 == "code")
      .ok_or("No code parameter in redirect URL")?;

  let code = AuthorizationCode::new(code_pair.1.into_owned());

  // 응답을 사용자 브라우저에 보여줌
  let response = Response::from_string("✅ logi CLI authenticated successfully. You may close this tab.");
  request.respond(response)?;

  // 🔐 code를 사용해서 access token 요청
  let token_result = client.exchange_code(code).request(http_client)?;

  println!("✅ Google access token obtained!");
  Ok(token_result.access_token().secret().to_string())
}
