use extism_convert::{FromBytes, Msgpack, ToBytes};
use petgraph::graph::DiGraph;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use url_serde::SerdeUrl;

use crate::types::{DNSRecord, Domain, EmailAddress, PhoneNumber};

pub type Graph = DiGraph<Node, String>;

#[derive(Clone, Debug, PartialEq, Eq, FromBytes, ToBytes, Serialize, Deserialize)]
#[encoding(Msgpack)]
/// Node is a node in the Graph.
pub enum Node {
    SocialMedia {
        social_media_url: SerdeUrl,
        account_url: SerdeUrl,
    },
    Website {
        url: SerdeUrl,
    },
    IP(IpAddr),
    PhoneNumber(PhoneNumber),
    EmailAddress(EmailAddress),
    Person(String),
    Organization(String),
    Domain(Domain),
    DNSEntry {
        nameserver: Domain,
        record: DNSRecord,
    },
}
