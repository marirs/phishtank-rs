# Phishtank API
![Crates.io](https://img.shields.io/crates/v/phishtank)
![docs.rs](https://img.shields.io/docsrs/phishtank/latest)
[![Build Status](https://travis-ci.com/marirs/phishtank-rs.svg?branch=master)](https://travis-ci.com/marirs/phishtank-rs)

Phishtank API gives access to phishtank to download the phishtank database or lookup for a url in phishtank database.

## Usage
```toml
[dependencies]
phishtank = "0.0.2"
```
and then
```rust
use phishtank::PhishtankClient;

// Download the Phishtank Database
fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let limit = 5;
    let res = PhishtankClient::new(&api_key)
        .user_agent("phishtank/[username]")
        .download_data();
    
    match res {
        Ok(data) => {
            for d in data.iter().take(limit) {
                println!("{:#?}", d)
            }
            println!("Showing {} out of {}", limit, data.len())
        }
        Err(e) => println!("Error: {:?}", e.to_string()),
    }
}
```

## Examples

- To download the database: `cargo run --example get_database <your api key>`

## Developer reference

- https://www.phishtank.com/developer_info.php

---
License: MIT