use schnuffel::plugin::Wrapper;

fn main() {
    let plugin = Wrapper::load(
        "https://github.com/extism/plugins/releases/latest/download/count_vowels.wasm",
    );
}
