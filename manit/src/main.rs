mod app;

use app::App;



// https://github.com/jaemk/cached/tree/master/examples/wasm
// https://crates.io/crates/wasm-bindgen-rayon/
// https://github.com/ctron/yew-oauth2

fn main() {
    yew::Renderer::<App>::new().render();
}
