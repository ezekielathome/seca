use seca::api::Seca;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let seca = Seca::from_official()?;

    // auth_ticket is an encrypted app ticket formatted as an hexadecimal string.
    // you can acquire one by
    // A: calling RequestEncryptedAppTicket with data '=<username>' (then calling GetEncryptedAppTicket in callback)
    // B: monitoring game traffic through something like fiddler
    let stats = seca.get_stats("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx", true)?;
    println!("{:#?}", stats);
    Ok(())
}
