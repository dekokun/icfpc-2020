use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let server_url = &((&args[1]).to_owned() + "/aliens/send");
    let player_key = &args[2];

    let _game_response = send(server_url, make_join_request(player_key)).await;
    let _game_response = send(server_url, make_start_request(player_key)).await;
    loop {
        let _game_response = send(server_url, make_commands_request(player_key)).await;
    }
}

fn make_join_request(player_key: &str) -> String {
    format!("{}{}{}{}{}{}{}", mod_str("("), mod_int(2), mod_str(","), mod_int(player_key.parse().unwrap()), mod_str(","), mod_str("nil"), mod_str(")"))
}

fn make_start_request(player_key: &str) -> String {
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", mod_str("("), mod_int(3), mod_str(","), mod_int(player_key.parse().unwrap()), mod_str(","), mod_str("("), mod_int(1), mod_str(","), mod_int(1), mod_str(","), mod_int(1), mod_str(","), mod_int(1), mod_str(")"), mod_str(")"))
}

fn make_commands_request(player_key: &str) -> String {
    format!("(4, {}, (1, 0))", player_key)
}


fn mod_int(i: i64) -> String {
    if i == 0 {
        return "010".to_owned()
    }
    let mut i = i;
    let prefix = if i < 0 {
        i = -i;
        "10"
    } else {
        "01"
    };
    let num = format!("{num:b}", num = i);
    let len = num.len();
    let num_of_one = ((len - 1) / 4) + 1;
    let pad = "1".repeat(num_of_one);
    let pad2_length = if len % 4 == 0 {
        0
    } else {
        4 - (len % 4)
    };
    let pad2 = "0".repeat( pad2_length );
    return prefix.to_owned() + &pad + "0" + &pad2 + &num
}

fn mod_str(s: &str) -> &str {
    match s {
        "(" => "11",
        ")" => "00",
        "nil" => "00",
        "," => "11",
        _ => unreachable!(),
    }
}

async fn send(
    server_url: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    println!("server_url: {}, body: {}", server_url, body);
    let req = Request::builder()
        .method(Method::POST)
        .header("Content-Type", "text/plain")
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

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #![allow(unused_imports)]
    use super::*;

    #[test]
    fn test0() {
        assert_eq!(mod_int(0), "010");
    }
    #[test]
    fn test1() {
        assert_eq!(mod_int(1), "01100001");
    }
    #[test]
    fn test15() {
        assert_eq!(mod_int(15), "01101111");
    }
    #[test]
    fn testminus1() {
        assert_eq!(mod_int(-1), "10100001");
    }
    #[test]
    fn test2() {
        assert_eq!(mod_int(2), "01100010");
    }
    #[test]
    fn test256() {
        assert_eq!(mod_int(256), "011110000100000000");
    }
}
