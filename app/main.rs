use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
use std::env;
use std::process;
use rand::Rng;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let server_url = &((&args[1]).to_owned() + "/aliens/send");
    let player_key = &args[2];

    let mut rng = rand::thread_rng();
    let x0: i64 = 442;
    let x1: i64 = 1;
    let x2: i64 = 0;
    let x3: i64 = 1;
    let _game_response = send(server_url, make_join_request(player_key)).await;
    let _game_response = send(server_url, make_start_request(player_key, x0, x1, x2, x3)).await;
    loop {
        let _game_response = send(server_url, make_commands_request(player_key, 1)).await;
        let _game_response = send(server_url, make_commands_request(player_key, 0)).await;
    }
}

fn make_join_request(player_key: &str) -> String {
    format!("{}{}{}{}{}{}{}", mod_str("("), mod_int(2), mod_str(","), mod_int(player_key.parse().unwrap()), mod_str(","), mod_str("nil"), mod_str(")"))
}

fn make_start_request(player_key: &str, x0: i64, x1: i64, x2: i64, x3: i64) -> String {
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", mod_str("("), mod_int(3), mod_str(","), mod_int(player_key.parse().unwrap()), mod_str(","), mod_str("("), mod_int(x0), mod_str(","), mod_int(x1), mod_str(","), mod_int(x2), mod_str(","), mod_int(x3), mod_str(")"), mod_str(")"))
}

fn make_commands_request(player_key: &str, ship_id: i64) -> String {
    format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", mod_str("("), mod_int(4), mod_str(","), mod_int(player_key.parse().unwrap()), mod_str(","), mod_str("("), mod_int(2), mod_str(","), mod_int(ship_id), mod_str(","), mod_str("("), mod_int(1), mod_str(","), mod_int(1), mod_str(")"),mod_str(","), mod_int(1), mod_str(")"),mod_str(")"))
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

/**
 * term    = num
           | "(" term ")"
           | term "," term
 */
fn mod_str(s: &str) -> &str {
    // let nil = "".to_owned();
    // let ret = "".to_owned();
    // let num = "".to_owned();
    // for c in s.chars() {
    //     match c {
    //         '(' => ret += "11",
    //         ')' => ret += "00",
    //         ',' => ret += "11",
    //         'n' => if nil != "" {
    //             panic!();
    //         } else {
    //             nil = "n".to_owned();
    //         },
    //         'i' => if nil != "n" {
    //             panic!();
    //         } else {
    //             nil = "ni".to_owned();
    //         },
    //         'l' => if nil != "ni" {
    //             panic!();
    //         } else {
    //             ret += "00";
    //             nil = "".to_owned();
    //         },
    //         '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
    //             num += c;
    //         }


    //     }
    // }
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
    #[test]
    fn start() {
        assert_eq!(format!("{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}", mod_str("("), mod_int(3), mod_str(","), mod_int("10000".parse().unwrap()), mod_str(","), mod_str("("), mod_int(1), mod_str(","), mod_int(1), mod_str(","), mod_int(1), mod_str(","), mod_int(1), mod_str(")"), mod_str(")")), "110110001111011111000100111000100001111011000011101100001110110000111011000010000")
    }
    #[test]
    fn join() {
        assert_eq!(format!("{}{}{}{}{}{}{}", mod_str("("), mod_int(2), mod_str(","), mod_int("10000".parse().unwrap()), mod_str(","), mod_str("nil"), mod_str(")")), "11011000101101111100010011100010000110000")
    }
}
