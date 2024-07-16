use std::net::{Ipv4Addr, Ipv6Addr};

#[derive(Copy, Clone, Debug)]
pub struct PhoneNumber<'a> {
    number: &'a str,
}
impl<'a> PhoneNumber<'a> {
    #[must_use]
    pub fn from(number: &str) -> PhoneNumber {
        // TODO: phone number validation
        PhoneNumber { number }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EmailAddress<'a> {
    email: &'a str,
}
impl<'a> EmailAddress<'a> {
    #[must_use]
    pub fn from(email: &str) -> EmailAddress {
        // TODO: email validation
        EmailAddress { email }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Domain<'a> {
    domain: &'a str,
}
impl<'a> Domain<'a> {
    #[must_use]
    pub fn from(domain: &str) -> Domain {
        // TODO: domain validation
        Domain { domain }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum DNSRecord<'a> {
    A(Ipv4Addr),
    AAAA(Ipv6Addr),
    CNAME {
        from: &'a str,
        to: Domain<'a>,
    },
    MX(Domain<'a>),
    SRV {
        service: &'a str,
        protocol: &'a str,
        from: &'a str,
        to: &'a str,
        to_port: u16,
    },
    TXT(&'a str),
}
