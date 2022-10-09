use std::{collections::HashMap};

use hmac::{Hmac, Mac};
use indexmap::IndexMap;
use reqwest::{self, StatusCode};
use sha2::{Digest, Sha256, Sha512};
mod helper_structs;
use helper_structs::{
    BaseResponse, CriticalErrorResponse, OrderDetails, ServerTimeType, TradingPairAll,
    TradingPairFees, TradingPairLeverage, TradingPairMargin, TradingPairRuler, XBTUSDRequest,
};

use cucumber::{given, then, when, World as _};

const API_URL: &str = "https://api.kraken.com";
const API_KEY_ENV: &str = "API_KEY";
const API_SEC_ENV: &str = "API_SEC";

#[derive(cucumber::World, Debug, Default)]
struct World {
    last_response_text: String,
    creds: SquidCreds,
    nonce_gen: NonceGen,
}

#[derive(Debug, Default)]
struct SquidCreds {
    key: String,
    secret: String,
}

impl SquidCreds {
    fn load_from_env() -> SquidCreds {
        SquidCreds {
            key: std::env::var(API_KEY_ENV).expect("API_KEY not found in environment"),
            secret: std::env::var(API_SEC_ENV).expect("API_SEC not fount in environment"),
        }
    }
}

#[derive(Debug, Default)]
struct NonceGen {
    last_nonce: i64,
}

impl NonceGen {
    fn get_nonce(&mut self) -> String {
        let mut nonce = chrono::Utc::now().timestamp() * 1000;
        if nonce == self.last_nonce {
            nonce += 1;
        }
        self.last_nonce = nonce;
        nonce.to_string()
    }
}

#[given("Squid website is responding")]
async fn squid_is_responding(_w: &mut World) {
    let resp = reqwest::get(API_URL).await;
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

#[when("Get request for server time is send")]
async fn send_get_time_request(w: &mut World) {
    let resp = reqwest::get("https://api.kraken.com/0/public/Time")
        .await
        .expect("Unable to send request for server time");
    w.last_response_text = resp.text().await.unwrap();
}

#[then("Server time is returned in the response body")]
async fn check_get_time_response(w: &mut World) {
    let body = serde_json::from_str::<BaseResponse<ServerTimeType>>(&w.last_response_text)
        .expect("Unable deserialize server response. Mesage may be malformed");
    assert!(
        body.error.is_empty(),
        "Found unexpexted error message in server time response"
    );
    let current_time = chrono::Utc::now();
    let unix_timestamp = current_time.timestamp();
    let server_timestamp = body.result.unixtime;
    let timestamp_diff = (unix_timestamp - server_timestamp).abs();
    const MAX_ALLOWED_TIMESTAMP_DIFF: i64 = 5;
    assert!(
        timestamp_diff < MAX_ALLOWED_TIMESTAMP_DIFF,
        "Too much difference between server time and system time, diff: {timestamp_diff}. Server or testing environment may have incorrect time source"
    );
    let server_time = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(server_timestamp, 0),
        chrono::Utc,
    );
    let server_time_rfc1123 = server_time.format("%a, %d %b %y %T %z").to_string();
    assert_eq!(
        server_time_rfc1123, body.result.rfc1123,
        "Servers unix time seems to be ok, but formatted time seems diferent"
    );
}


#[when("Get request for XBT/USD trading pair is send")]
async fn send_xbt2usd_traiding_pair_request(w: &mut World) {
    let resp = reqwest::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD")
        .await
        .expect("Some internal test problem. Cannot send request for XBT/USD traiding pair");
    w.last_response_text = resp.text().await.unwrap();
}

#[then("All trading pair informations are in response body")]
async fn check_traiding_pair_response(w: &mut World) {
    let body =
        serde_json::from_str::<BaseResponse<XBTUSDRequest<TradingPairAll>>>(&w.last_response_text)
            .expect("Cannot deserialize server response. Response body may be malformed.");
    assert!(body.error.is_empty());
    let reference = TradingPairRuler {
        aclass_base: "currency".to_string(),
        aclass_quote: "currency".to_string(),
        altname: "XBTUSD".to_string(),
        base: "XXBT".to_string(),
        fee_volume_currency: "ZUSD".to_string(),
        lot: "unit".to_string(),
        quote: "ZUSD".to_string(),
        wsname: "XBT/USD".to_string(),
    };
    let pair_data = body.result.XXBTZUSD;
    assert_eq!(pair_data, reference);
}

#[when("Get request for XBT/USD trading pair, limited to leverage info is send")]
async fn send_xbt2usd_traiding_pair_request_leverage(w: &mut World) {
    let resp =
        reqwest::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD&info=leverage")
            .await
            .expect("Some internal test problem. Cannot send request for XBT/USD traiding pair");
    w.last_response_text = resp.text().await.unwrap();
}

#[then("Leverage info for trading pair is in response body")]
async fn check_traiding_pair_response_leverage(w: &mut World) {
    let body = serde_json::from_str::<BaseResponse<XBTUSDRequest<TradingPairLeverage>>>(
        &w.last_response_text,
    )
    .expect("Cannot deserialize server response. Response body may be malformed.");
    assert!(body.error.is_empty());
}


#[when("Get request for XBT/USD trading pair, limited to fees info is send")]
async fn send_xbt2usd_traiding_pair_request_fees(w: &mut World) {
    let resp = reqwest::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD&info=fees")
        .await
        .expect("Some internal test problem. Cannot send request for XBT/USD traiding pair");
    w.last_response_text = resp.text().await.unwrap();
}

#[then("Fees info for trading pair is in response body")]
async fn check_traiding_pair_response_fees(w: &mut World) {
    let body =
        serde_json::from_str::<BaseResponse<XBTUSDRequest<TradingPairFees>>>(&w.last_response_text)
            .expect("Cannot deserialize server response. Response body may be malformed.");
    assert!(body.error.is_empty());
}


#[when("Get request for XBT/USD trading pair, limited to margin info is send")]
async fn send_xbt2usd_traiding_pair_request_margin(w: &mut World) {
    let resp = reqwest::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD&info=margin")
        .await
        .expect("Some internal test problem. Cannot send request for XBT/USD traiding pair");
    w.last_response_text = resp.text().await.unwrap();
}

#[then("Margin info for trading pair is in response body")]
async fn check_traiding_pair_response_margin(w: &mut World) {
    let body = serde_json::from_str::<BaseResponse<XBTUSDRequest<TradingPairMargin>>>(
        &w.last_response_text,
    )
    .expect("Cannot deserialize server response. Response body may be malformed.");
    assert!(body.error.is_empty());
}


#[when("Get request for XBT/USD trading pair, with info query value info is send")]
async fn send_xbt2usd_traiding_pair_request_info(w: &mut World) {
    let resp = reqwest::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD&info=info")
        .await
        .expect("Some internal test problem. Cannot send request for XBT/USD traiding pair");
    w.last_response_text = resp.text().await.unwrap();
}



#[when("Get request for XBT/USD trading pair, with info invalid value is send")]
async fn send_xbt2usd_traiding_pair_request_invalid(w: &mut World) {
    let resp =
        reqwest::get("https://api.kraken.com/0/public/AssetPairs?pair=XXBTZUSD&info=invalid_value")
            .await
            .expect("Some internal test problem. Cannot send request for XBT/USD traiding pair");
    w.last_response_text = resp.text().await.unwrap();
}

#[then("Response body contains Invalid argumants error")]
async fn check_traiding_pair_response_invalid(w: &mut World) {
    let body = serde_json::from_str::<CriticalErrorResponse>(&w.last_response_text)
        .expect("Cannot deserialize server response. Response body may be malformed.");
    assert_eq!(body.error, vec!["EGeneral:Invalid arguments"]);
}


fn create_signature(secret: &str, urlpath: &str, postdata: &str, nonce: &str) -> String {
    let msg_sha_input = nonce.to_string() + postdata;

    let mut sha256 = Sha256::default();
    sha256.update(&msg_sha_input.as_bytes());

    let data_hash = sha256.finalize();

    let mut hmac_input = urlpath.as_bytes().to_vec();
    for elem in data_hash {
        hmac_input.push(elem);
    }

    let hmac_key = base64::decode(secret.as_bytes()).unwrap();
    let mut mac = Hmac::<Sha512>::new_from_slice(&hmac_key[..]).unwrap();
    mac.update(&hmac_input);
    base64::encode(&mac.finalize().into_bytes())
}

fn urlencode(data: &IndexMap<&str, String>) -> String {
    let mut encoded = String::new();
    for (key, val) in data {
        encoded.push_str(&format!("{key}={val}&"));
    }
    encoded.pop();
    encoded
}

async fn private_request(
    uri_path: &str,
    data: IndexMap<&str, String>,
    api_key: &str,
    api_secret: &str,
) -> String {
    let client = reqwest::Client::new();
    let nonce = data.get("nonce").unwrap();
    let data_urlencoded = urlencode(&data);
    let signature = create_signature(api_secret, uri_path, &data_urlencoded, &nonce);

    let request = client
        .post(format!("{API_URL}{uri_path}"))
        .header("API-Key", api_key)
        .header("API-Sign", signature)
        .body(data_urlencoded);
    let response = request.send().await;
    let body = response
        .expect("Could not receive server response for OpenOrders")
        .text()
        .await;
    body.expect("could not translate server response for OpenOrders request to text format")
}

async fn get_open_orders(creds: &SquidCreds, nonce_gen: &mut NonceGen) -> String {
    let nonce = nonce_gen.get_nonce();
    let data = IndexMap::from([("nonce", nonce), ("trades", "true".to_string())]);
    private_request("/0/private/OpenOrders", data, &creds.key, &creds.secret).await
}

#[when("User add some order")]
async fn add_order_step(w: &mut World) {
    let _out = add_order(&w.creds, &mut w.nonce_gen).await;
}

async fn add_order(creds: &SquidCreds, nonce_gen: &mut NonceGen) -> String {
    let nonce = nonce_gen.get_nonce();
    let data = IndexMap::from([
        ("nonce", nonce),
        ("ordertype", "limit".to_string()),
        ("type", "sell".to_string()),
        ("volume", "0.011".to_string()),
        ("pair", "ethxbt".to_string()),
        ("price", "0.5".to_string()),
    ]);
    let out = private_request("/0/private/AddOrder", data, &creds.key, &creds.secret).await;
    out
}

#[given("User has no open orders")]
async fn cancel_all_orders_step(w: &mut World) {
    cancel_all_orders(&w.creds, &mut w.nonce_gen).await;
}

async fn cancel_all_orders(creds: &SquidCreds, nonce_gen: &mut NonceGen) -> String {
    let nonce = nonce_gen.get_nonce();
    let data = IndexMap::from([("nonce", nonce)]);
    private_request("/0/private/CancelAll", data, &creds.key, &creds.secret).await
}

#[given("User has account, API KEY, and API secret")]
async fn log_into_account(w: &mut World) {
    w.creds = SquidCreds::load_from_env();
}

#[when("Request for list of orders is send")]
async fn send_request_for_user_orders(w: &mut World) {
    let out = get_open_orders(&w.creds, &mut w.nonce_gen).await;
    w.last_response_text = out;
}

#[then("There is description of one order in response body")]
async fn check_user_orders_list_response(w: &mut World) {
    let body =
        serde_json::from_str::<BaseResponse<HashMap<String, HashMap<String, OrderDetails>>>>(
            &w.last_response_text,
        )
        .expect("Cannot deserialize server response. Response body may be malformed.");

    let (_, details) = &body.result["open"]
        .iter()
        .next()
        .expect("Test error there should be at least one open order");
    assert_eq!(details.status, "open".to_string());
    assert_eq!(details.vol, "0.01100000".to_string());
    assert_eq!(details.descr["pair"], "ETHXBT".to_string());
    assert_eq!(details.descr["price"], "0.50000".to_string());
}

#[then("There are not any orders in response body")]
async fn check_user_orders_list_response_empty(w: &mut World) {
    let body =
        serde_json::from_str::<BaseResponse<HashMap<String, HashMap<String, OrderDetails>>>>(
            &w.last_response_text,
        )
        .expect("Cannot deserialize server response. Response body may be malformed.");

    assert_eq!(*&body.result["open"].len(), 0 as usize);
}

#[tokio::main]
async fn main() {
    World::run("tests/features").await;
}
