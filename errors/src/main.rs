use std::fs::File;
use std::net::Ipv6Addr;

#[derive(Debug)]
enum UpstreamError {
    IO(std::io::Error),
    Parsing(std::net::AddrParseError),
}

fn main() {
    match do_something() {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn do_something() -> Result<(), UpstreamError> {
    let _f = File::open("invisible.txt").map_err(UpstreamError::IO)?;
    let _localhost = "::1".parse::<Ipv6Addr>().map_err(UpstreamError::Parsing)?; // error[E0277]: the trait bound `std::net::Ipv6Addr: std::convert::From<std::net::AddrParseError>` is not satisfied
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_something_with_valid_file() {
        assert!(do_something().is_ok());
    }

    #[test]
    fn test_do_something_with_invalid_file() {
        assert!(do_something().is_err());
    }
}
