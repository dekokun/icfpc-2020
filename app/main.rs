use http_body::Body as _;
use hyper::{Body, Client, Method, Request, StatusCode};
// use rand::Rng;
use std::env;
use std::process;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let server_url = &((&args[1]).to_owned() + "/aliens/send");
    let player_key = &args[2];

    println!("server_url: {}", server_url);

    // let mut rng = rand::thread_rng();
    let x0: i64 = 442;
    let x1: i64 = 1;
    let x2: i64 = 0;
    let x3: i64 = 1;
    let _game_response = send(server_url, make_join_request(player_key)).await;
    let (_game_stage, _role, ship_id, _position_x, _position_y, mut velocity_x, mut velocity_y) =
        send(server_url, make_start_request(player_key, x0, x1, x2, x3))
            .await
            .unwrap();
    loop {
        let (_game_stage, _role, _ship_id, _position_x, _position_y, velocity_x2, velocity_y2) =
            send(
                server_url,
                make_commands_request(player_key, ship_id, velocity_x, velocity_y),
            )
            .await
            .unwrap();
        velocity_x = velocity_x2;
        velocity_y = velocity_y2;
    }
}

fn make_join_request(player_key: &str) -> String {
    format!(
        "{}{}{}{}{}{}{}",
        mod_str("("),
        mod_int(2),
        mod_str(","),
        mod_int(player_key.parse().unwrap()),
        mod_str(","),
        mod_str("nil"),
        mod_str(")")
    )
}

fn make_start_request(player_key: &str, x0: i64, x1: i64, x2: i64, x3: i64) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        mod_str("("),
        mod_int(3),
        mod_str(","),
        mod_int(player_key.parse().unwrap()),
        mod_str(","),
        mod_str("("),
        mod_int(x0),
        mod_str(","),
        mod_int(x1),
        mod_str(","),
        mod_int(x2),
        mod_str(","),
        mod_int(x3),
        mod_str(")"),
        mod_str(")")
    )
}

fn make_commands_request(
    player_key: &str,
    ship_id: i64,
    velocity_x: i64,
    velocity_y: i64,
) -> String {
    format!(
        "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
        mod_str("("),
        mod_int(4),
        mod_str(","),
        mod_int(player_key.parse().unwrap()),
        mod_str(","),
        mod_str("("),
        mod_str("("),
        mod_int(0),
        mod_str(","),
        mod_int(ship_id),
        mod_str(","),
        mod_str("cons"),
        mod_int(velocity_x),
        mod_int(velocity_y),
        mod_str(")"),
        mod_str(")"),
        mod_str(")")
    )
    // format!("{}{}{}{}{}{}{}{}{}{}{}{}{}", mod_str("("), mod_int(4), mod_str(","), mod_int(player_key.parse().unwrap()), mod_str(","), mod_str("("),mod_str("("), mod_int(1), mod_str(","), mod_int(ship_id),mod_str(")"),mod_str(")"),mod_str(")"))
    // format!(
    //     "{}{}{}{}{}{}{}{}",
    //     mod_str("("),
    //     mod_int(4),
    //     mod_str(","),
    //     mod_int(player_key.parse().unwrap()),
    //     mod_str(","),
    //     mod_str("("),
    //     mod_str(")"),
    //     mod_str(")")
    // )
}

fn mod_int(i: i64) -> String {
    if i == 0 {
        return "010".to_owned();
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
    let pad2_length = if len % 4 == 0 { 0 } else { 4 - (len % 4) };
    let pad2 = "0".repeat(pad2_length);
    prefix.to_owned() + &pad + "0" + &pad2 + &num
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
        "cons" => "11",
        "," => "11",
        _ => unreachable!(),
    }
}

#[allow(dead_code)]
fn decode_response(s: &str) -> String {
    dbg!(s);
    if s == "" {
        return "".to_owned();
    }
    let now = &s[0..2];
    match now {
        "11" => "(".to_owned() + &decode_response(&s[2..s.len() - 2]) + ")",
        "10" | "01" => {
            let (num, remain) = decode_int(&s);
            num.to_string() + &decode_response(remain)
        }
        "00" => "nil".to_owned() + &decode_response(&s[2..]),
        _ => unimplemented!(),
    }
}

fn decode_response2(s: &str) -> (i64, i64, i64, i64, i64, i64, i64) {
    let s = expect_token(s, "(");
    let (_one, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (game_stage, s) = decode_int(s);
    let s = expect_token(s, ",");
    // staticGameInfo start
    let s = expect_token(s, "(");
    let (static_x0, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (role, s) = decode_int(s);
    let s = expect_token(s, ",");
    // x2 start
    let s = expect_token(s, "(");
    let (static_x2_0, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (static_x2_1, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (static_x2_2, s) = decode_int(s);
    let s = expect_token(s, ")");
    let s = expect_token(s, ",");
    // x3 start
    let s = expect_token(s, "(");
    let (static_x3_0, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (static_x3_1, s) = decode_int(s);
    let s = expect_token(s, ")");
    let s = expect_token(s, ",");
    // x4 is nil or (x, x, x, x)
    // consume nill or
    let (mut s, is_not_nil) = consume_token(s, "(");
    if is_not_nil {
        let (_static_x4_0, s2) = decode_int(s);
        s = s2;
        s = expect_token(s, ",");
        let (_static_x4_1, s2) = decode_int(s);
        s = s2;
        s = expect_token(s, ",");
        let (_static_x4_2, s2) = decode_int(s);
        s = s2;
        s = expect_token(s, ",");
        let (_static_x4_3, s2) = decode_int(s);
        s = s2;
        s = expect_token(s, ")");
    } else {
        s = expect_token(s, "nil");
    }
    let s = expect_token(s, ")");
    let s = expect_token(s, ",");
    // gameState start
    // consume nill or
    let (s, is_nil) = consume_token(s, "nil");
    if is_nil {
        return (game_stage, role, 0, 0, 0, 0, 0);
    }
    let s = expect_token(s, "(");
    let (game_tick, s) = decode_int(s);
    let s = expect_token(s, ",");
    // x1 start
    let s = expect_token(s, "(");
    let (state_x1_0, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (state_x1_1, s) = decode_int(s);
    let s = expect_token(s, ")");
    let s = expect_token(s, ",");
    // shipsAndCommands start
    let s = expect_token(s, "(");
    let s = expect_token(s, "(");
    let s = expect_token(s, "(");
    let (role, s) = decode_int(s);
    let s = expect_token(s, ",");
    let (ship_id, s) = decode_int(s);
    let s = expect_token(s, "cons");
    // よくわからんがもう一度受けてみる。
    let s = expect_token(s, "cons");
    let (position_x, s) = decode_int(s);
    let (position_y, s) = decode_int(s);
    let s = expect_token(s, ",");
    let s = expect_token(s, "cons");
    let (velocity_x, s) = decode_int(s);
    let (velocity_y, _s) = decode_int(s);
    dbg!(
        game_stage,
        static_x0,
        role,
        static_x2_0,
        static_x2_1,
        static_x2_2,
        static_x3_0,
        static_x3_1,
        game_tick,
        ship_id,
        position_x,
        position_y
    );
    dbg!(state_x1_0, state_x1_1,);
    (
        game_stage, role, ship_id, position_x, position_y, velocity_x, velocity_y,
    )
}

fn consume_token<'a>(s: &'a str, token: &str) -> (&'a str, bool) {
    let expect_raw = mod_str(token);
    let ret = &s[..2] == expect_raw;
    if ret {
        (&s[2..], ret)
    } else {
        (s, ret)
    }
}
fn expect_token<'a>(s: &'a str, token: &str) -> &'a str {
    let expect_raw = mod_str(token);
    assert_eq!(&s[..2], expect_raw);
    &s[2..]
}

#[allow(dead_code)]
fn expect_int(s: &str) {
    assert!(&s[..2] == "01" || &s[..2] == "10");
}
fn decode_int(mut s: &str) -> (i64, &str) {
    let minus = match &s[0..2] {
        "10" => true,
        "01" => false,
        _ => unimplemented!(),
    };
    s = &s[2..];
    let mut count = 0;
    loop {
        match &s[count..count + 1] {
            "0" => break,
            "1" => {
                count += 1;
            }
            _ => unimplemented!(),
        }
    }
    s = &s[count + 1..];
    let mut ans: i64 = 0;
    let size = count * 4;
    let num = &s[0..size];
    for (i, c) in num.chars().rev().enumerate() {
        ans += c.to_digit(10).unwrap() as i64 * 2_i64.pow(i as u32);
    }
    ans = if minus { -ans } else { ans };
    (ans, &s[size..])
}

async fn send(
    server_url: &str,
    body: String,
) -> Result<(i64, i64, i64, i64, i64, i64, i64), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();
    println!("body: {}", body);
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
                        Ok(content) => {
                            println!("{:?}", content);
                            let st = &format!("{:?}", content);
                            // debug出力(b"で始まり"で終わる)を迂回…
                            let st = &st[2..st.len() - 1];
                            return Ok(decode_response2(st));
                        }
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

    Ok((0, 0, 0, 0, 0, 0, 0))
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
        assert_eq!(
            format!(
                "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                mod_str("("),
                mod_int(3),
                mod_str(","),
                mod_int("10000".parse().unwrap()),
                mod_str(","),
                mod_str("("),
                mod_int(1),
                mod_str(","),
                mod_int(1),
                mod_str(","),
                mod_int(1),
                mod_str(","),
                mod_int(1),
                mod_str(")"),
                mod_str(")")
            ),
            "110110001111011111000100111000100001111011000011101100001110110000111011000010000"
        )
    }
    #[test]
    fn join() {
        assert_eq!(
            format!(
                "{}{}{}{}{}{}{}",
                mod_str("("),
                mod_int(2),
                mod_str(","),
                mod_int("10000".parse().unwrap()),
                mod_str(","),
                mod_str("nil"),
                mod_str(")")
            ),
            "11011000101101111100010011100010000110000"
        )
    }
    #[test]
    fn decode_test() {
        assert_eq!(
            // https://icfpc2020-api.testkontur.ru/logs?logKey=uHK5ukFTb3Z0X%2F7EbI08ZtEdR%2FT6pFYyxMkOlWTufuc%3D&apiKey=aeabd1ef5a0c4a0f98041be9c1ad59fd
            // https://icfpcontest2020.github.io/#/visualize?game=7652258d-b953-4911-ab44-393279550470
            decode_response("11011000011101100001111101111000010000000011011000011111011110000111000000110110000111011100100000000111101110000100001101110100000000011000011110110001011110111000010000110111010000000001111111101100001110101111101010011011000101110111110100001011000011111011110000110111001110110000111010110110000100110110100011011100100000011011000010011111101011110110000101100001000000111111010110110000111110110100001110001100001111010010111101110111111101101101010110110101011011000010011010110111001000000110110000100111111010111101010100001000000000000"),
            "(1(1((256(1((448(1(64nil((16(128nil(nilnil((2((16(128nil((((1(0((-9-46((-11((441(1(0(1nil(8(64(1nil(((0((11nilnilnil(((0(1((848((00((254))))))))))))))))))))))))))))))))))))))))))))))))))".to_owned()
            // "(1,1:gateStage,(256:x0,0:role,(512,1,64):x2,(16,128):x3,(10,10,10,10):x4):staticGameInfo,(0: gameTick,(16,128): x1,(((1:role,0:shipId,cons -7 -48:position,cons 0 0: velocity,(10,10,10,10):x4,0:x5,64:x6,1:x7)(nilnil(((0(1((748((0))))))))))))))))))))))))))))))))))))))))))))))))".to_owned()
            // "(1,1:gameStage,(256:x0,1:role,(448,1,64):x2,(16,128):x3,nil:x4):staticGameInfo,(2:gameTick,(16,128):x1,(((1(0((-9-46((-11((441(1(0(1nil(8(64(1nil(((0((11nilnilnil(((0(1((848((00((254))))))))))))))))))))))))))))))))))))))))))))))))))".to_owned()
        )
    }
    #[test]
    fn decode_response2_test() {
        assert_eq!(
            decode_response2("11011000011101100001111101111000011000000011011000011111011110000111000000110110000111011100100000000111101110000100001101110100000000011000011110110010011110111000010000110111010000000001111111101100001110101111011100001110001110001000101111101000101010011011110111100001101110001101100001110101101100001001101110000100001101110010000001101100001001111110101111011000010110000100000011111101011011000011111101100010000010110001001101111010011001001111011000011101100010110110001111011001000011010110111001000000110110000100110000000000"), (1,1,0,0,0,0,0)
        );
    }
    #[test]
    fn decode_response_test2() {
        assert_eq!(
            decode_response("110110000111011000011111011110000110000000110110000111110111100001110000001101100001110111001000000001111011100001000011011101000000000110000111101011110111000010000110111010000000001111111101100001110101111011100010000001110001100001111010010111101111000011011101011011000011101011011000010011010110111001000000110110000100110000111111010110110000111111011000100000101100011000011110100101111011000011101100010110110001111011001000011010110111001000000110110000100110000000000"), ""
        );
        // (1,1,(384,1,(448,1,64),(16,128),nil),(0,(16,128),(((1(0((3248((00((442(1(0(1nil(0(64(1nil(nilnil(((0(1((-32-48((00)))))))))))))))))))))))))))))))))))))))))))
    }
}
