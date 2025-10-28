mod app;
mod router;
mod ui;

mod components;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
