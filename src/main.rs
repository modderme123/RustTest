use wgpu;
use winit::{
    event,
    event_loop,
    window as win,
    dpi,
};

struct Window {

    //

}

fn main(){
    //
}

// use https://docs.rs/winit/0.20.0/winit/
fn build_window() -> Window {
    let event_loop = event_loop::EventLoop::new();
    let builder = win::WindowBuilder::new()
        .with_min_inner_size(dpi::PhysicalSize::new(800, 600))
        .with_max_inner_size(dpi::PhysicalSize::new(800, 600))
        .with_visible(true);
    let window = builder.build(&event_loop);

    return Window{};
}
