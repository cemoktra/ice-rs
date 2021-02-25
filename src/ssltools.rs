use std::io::prelude::*;
use openssl::ssl::SslVersion;
use openssl::x509::*;
use openssl::pkcs12::*;
use openssl::pkey::*;
use std::fs::File;
use std::path::Path;

use crate::errors::*;

pub (crate) fn read_pem(pem_file: &str, dir: &Path) ->Result<(X509, Option<PKey<Private>>), Box<dyn std::error::Error + Sync + Send>> {
    let mut buffer = vec![];
    let mut file = File::open(&dir.join(pem_file))?;
    file.read_to_end(&mut buffer)?;

    match (X509::from_pem(&buffer), PKey::private_key_from_pem(&buffer)) {
        (Ok(cert), Ok(pkey)) => {
            Ok((cert, Some(pkey)))
        }
        (Ok(cert), _) => {
            Ok((cert, None))
        }
        _ => Err(Box::new(ProtocolError::new(&format!("SSL: Error reading PEM file: {}", pem_file))))
    }
}

pub (crate) fn read_pkcs12(pkcs12_path: &Path, password: &str, dir: &Path) -> Result<ParsedPkcs12, Box<dyn std::error::Error + Sync + Send>> {
    let mut buffer = vec![];
    let mut file = File::open(&dir.join(pkcs12_path))?;
    file.read_to_end(&mut buffer)?;

    match Pkcs12::from_der(&buffer) {
        Ok(pkcs12) => {
            match pkcs12.parse(password) {
                Ok(parsed) => Ok(parsed),
                _ => Err(Box::new(ProtocolError::new("SSL: Could not parse pkcs12")))
            }
        },
        _ => Err(Box::new(ProtocolError::new("SSL: Could not read pkcs12")))
    }
}

pub (crate) fn parse_protocol(protocol: &str) -> Option<SslVersion> {
    match protocol.to_lowercase().as_ref() {
        "ssl3" | "sslv3" => {
            Some(SslVersion::SSL3)
        },
        "tls1_3" | "tlsv1_3" => {
            Some(SslVersion::TLS1_3)
        }
        "tls1_2" | "tlsv1_2" => {
            Some(SslVersion::TLS1_2)
        }
        "tls1_1" | "tlsv1_1" => {
            Some(SslVersion::TLS1_1)
        }
        "tls1" | "tlsv1" | "tls1_0" | "tlsv1_0" => {
            Some(SslVersion::TLS1)
        }
        _ => {
            None
        }
    }
}

fn protocol_to_num(version: SslVersion) -> i32 {
    match version {
        SslVersion::TLS1 => 1,
        SslVersion::TLS1_1 => 2,
        SslVersion::TLS1_2 => 3,
        SslVersion::TLS1_3 => 4,
        _ => 0
    }
}

fn num_to_protocol(num: i32) -> SslVersion {
    match num {
        4 => SslVersion::TLS1_3,
        3 => SslVersion::TLS1_2,
        2 => SslVersion::TLS1_1,
        1 => SslVersion::TLS1,
        _ => SslVersion::SSL3,
    }
}

pub (crate) fn max_protocol(new_value: SslVersion, current_max: Option<SslVersion>) -> SslVersion {
    let new_numeric = protocol_to_num(new_value);
    if let Some(current_max) = current_max {
        num_to_protocol(std::cmp::max(new_numeric, protocol_to_num(current_max)))
    } else {
        new_value
    }
}

pub (crate) fn min_protocol(new_value: SslVersion, current_min: Option<SslVersion>) -> SslVersion {
    let new_numeric = protocol_to_num(new_value);
    if let Some(current_min) = current_min {
        num_to_protocol(std::cmp::min(new_numeric, protocol_to_num(current_min)))
    } else {
        new_value
    }
}