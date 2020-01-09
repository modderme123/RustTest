use wgpu;
use winit::{
    event,
    event_loop,
    window as win,
    dpi,
};
use winit::platform::macos::WindowBuilderExtMacOS;

struct Config {

    min_width: u16,
    min_height: u16,
    max_width: u16,
    max_height: u16,
    resizeable: bool,
    visible: bool,
    titlebar: bol,
    title: String,

}

fn main(){
    //
}

// use https://docs.rs/winit/0.20.0/winit/
fn build_window(config: &Config) {
    let event_loop = event_loop::EventLoop::new();
    let builder = win::WindowBuilder::new()
        .with_min_inner_size(dpi::PhysicalSize::new(config.min_width, config.min_height))
        .with_max_inner_size(dpi::PhysicalSize::new(config.max_width, config.max_height))
        .with_visible(config.visible)
        .with_resizable(config.resizeable)
        .with_titlebar_hidden(config.titlebar)
        .with_title(config.title.into()); // TODO add more things here
    let window = builder.build(&event_loop).unwrap();
}
