use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use cli_clipboard::{ClipboardContext, ClipboardProvider};

#[derive(Debug, Serialize, Deserialize)]
struct IpAddress {
    ip:String
}

fn main() {
    println!("
███╗   ███╗██╗   ██╗    ██╗██████╗
████╗ ████║╚██╗ ██╔╝    ██║██╔══██╗
██╔████╔██║ ╚████╔╝     ██║██████╔╝
██║╚██╔╝██║  ╚██╔╝      ██║██╔═══╝
██║ ╚═╝ ██║   ██║       ██║██║
╚═╝     ╚═╝   ╚═╝       ╚═╝╚═╝");
    println!("🧭 Get My Rusty IP v1");
    println!();

    let ip = get_my_ip().unwrap_or_else(|_e| "Unavailable".to_owned());


    println!("🛸 Your Public IP is {ip}");
    println!();

    if ip != "Unavailable" {
        let mut ctx = ClipboardContext::new().unwrap();

        ctx.set_contents(ip.to_owned()).unwrap();

        println!("📋 Copied To Clipboard.");
    }

    thread::sleep(Duration::from_secs(999)) //make the terminal stay open
}


fn get_my_ip () -> Result<String, reqwest::Error> {
    let request = reqwest::blocking::get("https://api.ipify.org?format=json")?;

    let body: IpAddress = request.json()?;
    Ok(body.ip)
}