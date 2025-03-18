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
╚═╝     ╚═╝   ╚═╝       ╚═╝╚═╝
");
    println!("🧭 Get My Rusty IP v1");
    println!();

    let ip: String = get_my_ip();

    println!("🛸 Your Public IP is {}", ip);
    println!();

    if ip != "Unavailable" {
        let mut ctx = ClipboardContext::new().unwrap();

        ctx.set_contents(ip.to_owned()).unwrap();

        println!("📋 Copied To Clipboard.");
    }
}


fn get_my_ip () ->String {
    let request = reqwest::blocking::get("https://api.ipify.org?format=json");
    let unwrapped_request;
    match request {
        Err(_e) => return String::from("Unavailable"),
        Ok(v) => unwrapped_request = v,
    }

   if unwrapped_request.status().is_success() {
    let body: IpAddress = unwrapped_request.json().unwrap();
       body.ip
   } else {
       String::from("Unavailable")
   }
}