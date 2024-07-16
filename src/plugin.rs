struct PluginConfig<'a> {
    url: &'a str,
    extra_fields: &'a [&'a str],
}

struct Plugin<S: PluginState> {
    state: S,
}

impl Plugin<NewState> {
    fn load(url: &str) -> Plugin<InitializedState> {
        todo!()
    }
}
impl<'a> Plugin<InitializedState<'a>> {
    fn get_config_fields() -> &'a [&'a str] {
        todo!()
    }

    fn set_config_field() {
        todo!()
    }

    fn start() -> Plugin<RunningState<'a>> {
        todo!()
    }
}
impl<'a> Plugin<RunningState<'a>> {
    fn status() {
        todo!()
    }

    fn exec_on() {
        todo!()
    }
}

trait PluginState {}
struct NewState {}
struct InitializedState<'a> {
    config: PluginConfig<'a>,
}
struct RunningState<'a> {
    config: PluginConfig<'a>,
}
impl PluginState for NewState {}
impl<'a> PluginState for InitializedState<'a> {}
impl<'a> PluginState for RunningState<'a> {}
