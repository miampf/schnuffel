use core::panic;
use std::collections::HashMap;

use extism::{typed_plugin, Manifest, Plugin, Wasm};

use schnuffel_types::graph::{Graph, Node};
use schnuffel_types::plugin::{Input, Output};

// The type information that a plugin must adhere to.
// TODO: let plugins return results once own error is written.
typed_plugin!(SchnuffelPlugin {
    default_config(Input<String>) -> Output<HashMap<String, String>>;
    exec_on_node(Input<Node>) -> Output<Graph>;
    exec_on_graph(Input<Graph>) -> Output<Graph>;
});

#[derive(Copy, Clone, Debug)]
pub struct Wrapper<S: State> {
    state: S,
}

impl Wrapper<NewState> {
    /// Load a plugin from an URL.
    ///
    /// # Panics
    ///
    /// Panics if the plugin has a wrong type or getting the plugin
    /// configuration fails.
    #[must_use]
    pub fn load(url: &str) -> Wrapper<InitializedState> {
        let url = Wasm::url(url);
        let manifest = Manifest::new([url]);
        // temporarily construct the plugin to check types and load config
        let mut plugin: SchnuffelPlugin = Plugin::new(&manifest, [], true)
            .unwrap()
            .try_into()
            .unwrap();
        let config = plugin
            .default_config(Input {
                config: HashMap::new(),
                data: String::new(),
            })
            .unwrap()
            .data;
        Wrapper {
            state: InitializedState {
                config,
                plugin_manifest: manifest,
            },
        }
    }
}
impl<'a> Wrapper<InitializedState> {
    /// Set a config field of the plugin.
    ///
    /// # Panics
    ///
    /// Currently panics if the config field doesn't exist.
    pub fn set_config_field(&mut self, field: &str, to: &str) {
        // TODO: remove panic once own error type is written
        match self.state.config.insert(field.to_string(), to.to_string()) {
            Some(_) => {}
            None => panic!("No such config field {}", field),
        }
    }

    /// Finish all configurations and enter the running state.
    ///
    /// # Panics
    ///
    /// Panics if the plugin has invalid types.
    #[must_use]
    pub fn start(&self) -> Wrapper<RunningState> {
        let plugin: SchnuffelPlugin = Plugin::new(&self.state.plugin_manifest, [], true)
            .unwrap()
            .try_into()
            .unwrap();
        Wrapper {
            state: RunningState {
                config: self.state.config.clone(),
                plugin,
            },
        }
    }
}
impl<'a> Wrapper<RunningState> {
    /// Execute the plugin on a given node.
    ///
    /// # Panics
    ///
    /// Panics if the execution fails
    pub fn exec_on_node(&mut self, node: Node) -> Graph {
        let input = Input {
            config: self.state.config.clone(),
            data: node,
        };
        // TODO: error handling
        let res = self.state.plugin.exec_on_node(input).unwrap();
        res.data
    }
    /// Execute the plugin on a given graph.
    ///
    /// # Panics
    ///
    /// Panics if the execution fails.
    pub fn exec_on_graph(&mut self, graph: Graph) -> Graph {
        let input = Input {
            config: self.state.config.clone(),
            data: graph,
        };
        // TODO: error handling
        let res = self.state.plugin.exec_on_graph(input).unwrap();
        res.data
    }
}

pub trait State {}

#[derive(Copy, Clone, Debug)]
pub struct NewState {}

#[derive(Clone, Debug)]
pub struct InitializedState {
    config: HashMap<String, String>,
    plugin_manifest: Manifest,
}

pub struct RunningState {
    config: HashMap<String, String>,
    plugin: SchnuffelPlugin,
}

impl State for NewState {}
impl State for InitializedState {}
impl State for RunningState {}
