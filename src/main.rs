use serde_json::{Value};
use bytes::Bytes;
use warp::{Filter};

#[tokio::main]
async fn main() {
    // POST /v1/receive/:log_name

    let log_receiver = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("receive"))
        .and(warp::path::param())
        .and(warp::body::bytes())
        .map(|log_name: String, body: Bytes| {
            let body_msg = std::str::from_utf8(&body).unwrap();
            let json_test : Value = serde_json::from_str(&body_msg).unwrap();
            if json_test.is_object() {
                println!("{} {} {}", log_name, body_msg, json_test);
            } else {
                println!("{} {} Oops! json body is invalid! : {}", log_name, body_msg, json_test);
            }

            warp::reply::with_status("", warp::http::StatusCode::OK)
        });

    warp::serve(log_receiver).run(([0,0,0,0], 8080)).await
}
