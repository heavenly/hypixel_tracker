use std::io;
use std::time;
fn send_webhook_req(message_data: &str, webhook_url: &str) {
    let _r = ureq::post(&webhook_url).send_form(&[
        ("username", "helli.sh player tracker"),
        ("content", message_data),
    ]);
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");

    input.trim().to_string()
}

fn is_player_online(username: &str, api_key: &str) -> bool {
    let hypixel_api_url = format!(
        "https://api.hypixel.net/player?key={}&name={}",
        api_key, username
    );
    let r: serde_json::Value = ureq::get(&hypixel_api_url)
        .call()
        .unwrap()
        .into_json()
        .unwrap();

    if r["success"].as_bool().unwrap() == false {
        println!("failed to acquire player api data for {}", username);
        return false;
    }

    if r["player"].as_object().is_none() {
        println!("failed to acquire player api object for {}", username);
        return false;
    }

    r["player"]["lastLogout"].as_i64().unwrap() < r["player"]["lastLogin"].as_i64().unwrap()
}

fn main() {
    println!("enter username to track: ");
    let test_player = read_line();
    println!("helli.sh hypixel player tracker by laurent");
    println!("currently tracking: {}", test_player);
    println!("enter your hypixel api key: (do /api new and copy here)");
    let api_key = read_line();
    println!("enter your discord webhook url: (go to integrations and create new webhook)");
    let webhook_url = read_line();

    let mut is_online = is_player_online(&test_player, &api_key);
    loop {
        let temp_is_online = is_player_online(&test_player, &api_key);
        if temp_is_online != is_online {
            is_online = temp_is_online;
            let message_data = {
                if is_online {
                    format!("{} is now online.", &test_player);
                } else {
                    format!("{} is now offline.", &test_player);
                }
                String::new()
            };
            println!("{}", message_data);
            send_webhook_req(&message_data, &webhook_url);
        }
        std::thread::sleep(time::Duration::from_secs(2));
    }
}
