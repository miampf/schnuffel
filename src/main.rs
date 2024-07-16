use petgraph::dot::{Config, Dot};
use petgraph::graph::DiGraph;

use schnuffel::graph::Node;
use schnuffel::types::{EmailAddress, PhoneNumber};

fn main() {
    let mut g = DiGraph::new();

    let person = g.add_node(Node::Person("John Doe"));
    let phone = g.add_node(Node::PhoneNumber(PhoneNumber::from("+00000000000")));
    let email = g.add_node(Node::EmailAddress(EmailAddress::from(
        "john.doe@example.com",
    )));

    g.add_edge(person, phone, "");
    g.add_edge(person, email, "");

    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));
}
