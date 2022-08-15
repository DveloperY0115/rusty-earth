extern crate approx;
extern crate glium;
extern crate nalgebra as na;
use glium::Surface;

fn main() {
    use glium::glutin;

    let (event_loop, display) = create_eventloop_and_display();

    event_loop.run(move |ev, _, control_flow| {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.finish().unwrap();

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            _ => (),
        }
    });
}

/// Creates a new 'EventLoop' and 'Display'.
///
/// 'EventLoop' and 'Display' must be initialized before
/// invoking any draw calls in OpenGL programming.
/// This function creates default 'EventLoop' and 'Display'.
///
/// # Examples
///
/// Basic uages:
///
/// ```
/// let (event_loop, display) = init_display();
/// ```
fn create_eventloop_and_display() -> (glium::glutin::event_loop::EventLoop<()>, glium::Display) {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = glium::glutin::window::WindowBuilder::new();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    return (event_loop, display);
}
