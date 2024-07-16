use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhoneNumber {
    number: String,
}
impl PhoneNumber {
    #[must_use]
    pub fn from(number: &str) -> PhoneNumber {
        // TODO: phone number validation
        PhoneNumber {
            number: number.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EmailAddress {
    email: String,
}
impl EmailAddress {
    #[must_use]
    pub fn from(email: &str) -> EmailAddress {
        // TODO: email validation
        EmailAddress {
            email: email.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Domain {
    domain: String,
}
impl Domain {
    #[must_use]
    pub fn from(domain: &str) -> Domain {
        // TODO: domain validation
        Domain {
            domain: domain.to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum DNSRecord {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME {
        from: String,
        to: Domain,
    },
    MX(Domain),
    SRV {
        service: String,
        protocol: String,
        from: String,
        to: String,
        to_port: u16,
    },
    TXT(String),
}
