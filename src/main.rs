use petgraph::dot::{Config, Dot};

use schnuffel::graph::{Graph, Node};
use schnuffel::plugin::Wrapper;
use schnuffel::types::{EmailAddress, PhoneNumber};

fn main() {
    let plugin = Wrapper::load(
        "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm",
    );
}
