use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let server_url = &args[1];
    let player_key = &args[2];

    println!(
        "ServerUrl: {}; PlayerKey: {}",
        server_url.to_owned() + "/aliens/send",
        player_key
    );
    let _game_response = send(server_url, make_join_request(player_key)).await;
    let _game_response = send(server_url, make_start_request(player_key)).await;
    loop {
        let _game_response = send(server_url, make_commands_request(player_key)).await;
    }
}

fn make_join_request(player_key: &str) -> String {
    format!("(2, {}, nil)", player_key)
}

fn make_start_request(player_key: &str) -> String {
    format!("(3, {}, (0, 0, 0, 0))", player_key)
}

fn make_commands_request(player_key: &str) -> String {
    format!("(4, {}, (1, 0))", player_key)
}
async fn send(
    server_url: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    let req = Request::builder()
        .method(Method::POST)
        .uri(server_url)
        .body(Body::from(body))?;

    match client.request(req).await {
        Ok(mut res) => match res.status() {
            StatusCode::OK => {
                print!("Server response: ");
                while let Some(chunk) = res.body_mut().data().await {
                    match chunk {
                        Ok(content) => println!("{:?}", content),
                        Err(why) => println!("error reading body: {:?}", why),
                    }
                }
            }
            _ => {
                println!("Unexpected server response:");
                println!("HTTP code: {}", res.status());
                print!("Response body: ");
                while let Some(chunk) = res.body_mut().data().await {
                    match chunk {
                        Ok(content) => println!("{:?}", content),
                        Err(why) => println!("error reading body: {:?}", why),
                    }
                }
                process::exit(2);
            }
        },
        Err(err) => {
            println!("Unexpected server response:\n{}", err);
            process::exit(1);
        }
    }

    Ok(())
}
