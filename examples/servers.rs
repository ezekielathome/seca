use seca::api::Seca;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seca = Seca::from_official()?;
    let servers = seca.get_server_list()?;
    for server in servers {
        // Check if server has any players
        if let Some(_players) = server.players {
            println!("{:#?}", server);
        }
    }

    Ok(())
}
