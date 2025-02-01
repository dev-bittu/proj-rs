use std::{net::IpAddr, str::FromStr};

struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u32
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not Enough Arguments")
        } else if args.len() > 4 {
            return Err("Tooo many args");
        }

        let f = args[1].clone();

        if let Ok(ip_addr) = IpAddr::from_str(f){
            return Ok(Arguments { flag: String::from(""), ip_addr: ip_addr, threads: 4 });
        }
    }
}


fn main() {
    println!("Hello, world!");
}
