use clap::{arg, command, ArgAction};
use std::path::Path;
use reqwest::*;
pub fn read_file(file_path:&Path){


}
pub fn args() {
    let matches = command!() // requires `cargo` feature
        .next_line_help(true)
        .arg(arg!(-f --file <VALUE> "Path of proxie file").required(true).action(ArgAction::Set))
        .arg(arg!(-p  --proxy <VALUE> "Proxy type").required(true).action(ArgAction::Set))
        .get_matches();

    
        let file_path=matches.get_one::<String>("file").expect("required");
        let proxy_type =matches.get_one::<String>("proxy").expect("required");
        
        match proxy_type.as_str() {
            "socks4"=>Some("socks4://"),
            "socks5"=>Some("socks5://"),
            "http"=>Some("http://"),
            "https"=>Some("https://"),
            _=>panic!()
            
        };
        println!("proxy type is {}",proxy_type);

   
}