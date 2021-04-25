use crate::window;
use imgui_winit_support::{WinitPlatform, HiDpiMode};
use super::fonts::add_fonts;
use winit::event_loop::EventLoop;
use imgui_glium_renderer::Renderer;
use imgui::{Context, Ui, FontSource, FontConfig};
use std::time::Instant;
use glutin::event::{Event, WindowEvent};
use glium::Surface;
use glutin::event_loop::ControlFlow;
use crate::window::WindowController;

pub struct Imgui {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub controller: WindowController,
}

impl Imgui {
    pub fn new(window: window::Window, mut imgui: imgui::Context) -> Self {
        let window::Window { event_loop, controller, display } = window;

        imgui.set_ini_filename(None);

        if let Some(backend) = super::clipboard::init() {
            imgui.set_clipboard_backend(Box::new(backend));
        } else {
            eprintln!("Failed to initialize clipboard");
        }

        let mut platform = WinitPlatform::init(&mut imgui);
        {
            let gl_window = display.gl_window();
            let window = gl_window.window();
            platform.attach_window(imgui.io_mut(), window, HiDpiMode::Rounded);
        }

        let hidpi_factor = platform.hidpi_factor();

        imgui.io_mut().font_global_scale = (1.0 / hidpi_factor) as f32;
        let fonts = add_fonts(&mut imgui);
        let renderer = Renderer::init(&mut imgui, &display).expect("Unable to create imgui renderer");

        Self {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
            controller,
        }
    }

    pub fn run(self, mut run_ui: impl FnMut(&mut Ui, &mut WindowController) + 'static) -> ! {
        let Imgui {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            mut controller
        } = self;

        let mut last_frame = Instant::now();

        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                let gl_window = display.gl_window();
                platform
                    .prepare_frame(imgui.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                controller.update();

                let mut ui = imgui.frame();

                run_ui(&mut ui, &mut controller);

                let gl_window = display.gl_window();
                let mut target = display.draw();
                target.clear_color_srgb(1.0, 1.0, 1.0, 0.0);
                // target.clear_all((0.0, 0.0, 0.0, 0.0), 0.0, 0);
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = ui.render();
                renderer
                    .render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = display.gl_window();
                platform.handle_event(imgui.io_mut(), gl_window.window(), &event);
            }
        })
    }
}