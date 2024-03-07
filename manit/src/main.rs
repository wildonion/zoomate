mod app;

use app::App;



// https://github.com/jaemk/cached/tree/master/examples/wasm

fn main() {
    yew::Renderer::<App>::new().render();
}
