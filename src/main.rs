extern crate approx;
extern crate glium;
extern crate nalgebra as na;
use glium::{implement_vertex, Surface};

fn main() {
    use glium::glutin;

    let (event_loop, display) = create_display();

    // define the basic vertex shader
    let vertex_shader_src = r#"
        #version 140

        in vec2 position;

        void main() {
            gl_Position = vec4(position, 0.0, 1.0);
        }
    "#;

    // define the basic fragment shader
    let fragment_shader_src = r#"
        #version 140

        out vec4 color;

        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
        }
    "#;

    // define a triangle
    #[derive(Copy, Clone)]
    struct Vertex {
        position: [f32; 2],
    }

    implement_vertex!(Vertex, position);

    let v1 = Vertex {
        position: [-0.5, -0.5],
    };
    let v2 = Vertex {
        position: [0.0, 0.5],
    };
    let v3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![v1, v2, v3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    event_loop.run(move |ev, _, control_flow| {
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
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &glium::uniforms::EmptyUniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
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
/// let (event_loop, display) = create_display();
/// ```
fn create_display() -> (glium::glutin::event_loop::EventLoop<()>, glium::Display) {
    let event_loop = glium::glutin::event_loop::EventLoop::new();
    let window_builder = glium::glutin::window::WindowBuilder::new();
    let context_builder = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();
    return (event_loop, display);
}
