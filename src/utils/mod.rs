pub fn convert_port_to_u16(raw_port: &str) -> u16 {
//! Converts the &str received from the .env file to u16 for use in the HttpServer
    let port: u16;
    match raw_port.parse::<u16>() {
        Ok(num) => port = num,
        Err(_) => port = 8080 as u16
    }

    return port;
}