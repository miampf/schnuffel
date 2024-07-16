pub mod plugin;

#[cfg(test)]
mod test {
    use petgraph::dot::{Config, Dot};
    use schnuffel_types::graph::{Graph, Node};

    use crate::plugin::Wrapper;
    #[test]
    pub fn test_simple_plugin() {
        let mut plugin = Wrapper::load("https://github.com/miampf/schnuffel-testplugin/releases/download/v0.0.1/schnuffel_testplugin.wasm").start();
        let result = plugin.exec_on_node(Node::Person("John Doe".to_string()));
        let cmp = Graph::new();
        assert_eq!(
            format!("{:?}", Dot::with_config(&result, &[Config::EdgeNoLabel])),
            format!("{:?}", Dot::with_config(&cmp, &[Config::EdgeNoLabel]))
        );
    }
}
