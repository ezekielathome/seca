use seca::api::Seca;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seca = Seca::from_official()?;
    let stable = seca.get_server_list()?;
    for server in stable {
        if let Some(_players) = server.players {
            println!("{:#?}", server);
        }
    }

    let beta = seca.get_beta_server_list()?;
    for server in beta {
        if let Some(_players) = server.players {
            println!("{:#?}", server);
        }
    }

    Ok(())
}
