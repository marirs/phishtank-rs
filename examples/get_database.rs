use phishtank::PhishtankClient;

fn main() {
    let api_key = match std::env::args().nth(1).ok_or("Please provide the api key!") {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let limit = 5;
    let res = PhishtankClient::new(&api_key).download_db();
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
