use wgpu;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    window::Window,
    dpi,
};

struct Config {

    min_width: u16,
    min_height: u16,
    max_width: u16,
    max_height: u16,
    resizeable: bool,
    visible: bool,
    title: String,
    always_on_top: bool,

    // TODO add more elements to the config

}

fn main(){
    let event_loop = EventLoop::new();
    let window: Window = build_window(&event_loop, &Config{
        min_width: 600,
        min_height: 600,
        max_width: 2000,
        max_height: 2000,
        resizeable: true,
        visible: true,
        title: "test title".to_string(),
        always_on_top: false,
    });

    event_loop.run(move | event, _, control_flow | {
        // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
        // dispatched any events. This is ideal for games and similar applications.
        *control_flow = ControlFlow::Poll;

        // ControlFlow::Wait pauses the event loop if no events are available to process.
        // This is ideal for non-game applications that only update in response to user
        // input, and uses significantly less power/CPU time than ControlFlow::Poll.
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit
            },
            Event::MainEventsCleared => {
                // Application update code.
                // Queue a RedrawRequested event.
                window.request_redraw();
            },
            Event::RedrawRequested(_) => {
                // Redraw the application.
                // It's preferrable to render in this event rather than in MainEventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
            },
            _ => ()
        }
    });
}

// use https://docs.rs/winit/0.20.0/winit/
fn build_window(event_loop: &EventLoop<()>, config: &Config) -> Window {
    let builder = WindowBuilder::new()
        .with_min_inner_size(dpi::PhysicalSize::new(config.min_width, config.min_height))
        .with_max_inner_size(dpi::PhysicalSize::new(config.max_width, config.max_height))
        .with_visible(config.visible)
        .with_resizable(config.resizeable)
        .with_always_on_top(config.always_on_top)
        .with_title(&config.title); // TODO add more things here
    let window = builder.build(event_loop).unwrap();

    return window;
}
