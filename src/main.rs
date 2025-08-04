use ftp_client::prelude::*;
use std::io;

fn main() -> Result<(), ftp_client::error::Error>  {
    let mirror = "openbsd.cs.toronto.edu";

    let mut client = Client::connect(mirror, "anonymous", "anonymous")?;
   
    let _ = client.cwd("pub/OpenBSD/7.7/packages/amd64")?;
    println!("{:?}", client.pwd());

    loop {
        println!();
        println!("Search for package: ");
        let mut keyword = String::new();
        io::stdin().read_line(&mut keyword).unwrap();
        let keyword = keyword.trim();

        if keyword.eq_ignore_ascii_case("quit") {
            break;
        }

        if keyword.is_empty() {
            continue;
        }

        let names = client.list_names(".")?;
        println!("Listing packages with the '{}' keyword:", keyword);
        println!();
        for name in names {
            if name.to_lowercase().contains(&keyword.to_lowercase()) {
                let n = name.replace(".tgz", "");
                println!("{}", n);
            }
        }
    }

    Ok(())
}
