use phishtank::PhishtankClient;

fn main() {
    let mut args = std::env::args().skip(1);
    let api_key = match args
        .next()
        .ok_or("Please provide the `api key` as 1st argument!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let url_to_check = match args
        .next()
        .ok_or("Please provide the `URL` to check as 2nd argument!")
    {
        Ok(api_key) => api_key,
        Err(e) => {
            println!("{:?}", e);
            std::process::exit(1)
        }
    };

    let res = PhishtankClient::new(&api_key).lookup(&url_to_check);
    match res {
        Ok(data) => println!("{:#?}", data),
        Err(e) => println!("Error: {:?}", e.to_string()),
    }
}
