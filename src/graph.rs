use std::net::IpAddr;
use url::Url;

use crate::types::{DNSRecord, Domain, EmailAddress, PhoneNumber};

#[derive(Clone, Debug)]
pub enum Node<'a> {
    SocialMedia {
        social_media_url: Url,
        account_url: Url,
    },
    Website {
        url: Url,
    },
    IP(IpAddr),
    PhoneNumber(PhoneNumber<'a>),
    EmailAddress(EmailAddress<'a>),
    Person(&'a str),
    Organization(&'a str),
    Domain(Domain<'a>),
    DNSEntry {
        nameserver: Domain<'a>,
        record: DNSRecord<'a>,
    },
}
