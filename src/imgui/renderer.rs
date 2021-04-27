use crate::window;
use imgui_winit_support::{WinitPlatform, HiDpiMode};
use super::fonts::add_fonts;
use winit::event_loop::EventLoop;
use imgui_glium_renderer::Renderer;
use imgui::{Context, Ui, FontSource, FontConfig, FontId};
use std::time::{Instant, Duration};
use glutin::event::{Event, WindowEvent};
use glium::Surface;
use glutin::event_loop::ControlFlow;
use crate::window::WindowController;
use std::marker::PhantomData;
use std::collections::HashMap;
use std::cell::RefCell;
use crate::types::Font;
use std::ptr::null;
use std::mem;

pub struct Imgui {
    pub event_loop: EventLoop<()>,
    pub display: glium::Display,
    pub imgui: Context,
    pub platform: WinitPlatform,
    pub renderer: Renderer,
    pub controller: WindowController,
    pub fonts: HashMap<Font, FontId>
}

impl Imgui {
    pub fn new(window: window::OverlayWindow, mut imgui: imgui::Context) -> Self {
        let window::OverlayWindow { event_loop, controller, display } = window;

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

        // imgui.io_mut().font_global_scale = (hidpi_factor) as f32;
        let fonts = add_fonts(&mut imgui, 1.0);
        let renderer = Renderer::init(&mut imgui, &display).expect("Unable to create imgui renderer");

        Self {
            event_loop,
            display,
            imgui,
            platform,
            renderer,
            controller,
            fonts
        }
    }

    /// Runs the ui with a state that gets inited with Default
    pub fn run(self, mut run_ui: impl FnMut(&mut Ui, &mut RenderState, &mut RenderContext) + 'static) -> ! {
        let Imgui {
            event_loop,
            display,
            mut imgui,
            mut platform,
            mut renderer,
            mut controller,
            fonts,
            ..
        } = self;

        let mut render_context = RenderContext { ui_open: true, bypass_screenshots: true, fonts };
        let mut render_context_init = false;
        let mut fade_start = None;
        let fade_time = Duration::from_millis(1000);

        let mut state = RenderState::new();

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

                // imgui.style_mut().alpha = fade_animation(&fade_start, fade_time, render_context.ui_open);

                let mut ui = imgui.frame();

                let old_render_context = render_context.clone();
                run_ui(&mut ui, &mut state, &mut render_context);
                if old_render_context.ui_open != render_context.ui_open || !render_context_init {
                    fade_start = Some(Instant::now());
                    controller.clickthrough(!render_context.ui_open);
                }
                if old_render_context.bypass_screenshots != render_context.bypass_screenshots || !render_context_init {
                    controller.hide_screenshots(render_context.bypass_screenshots);
                }
                render_context_init = true;

                let gl_window = display.gl_window();
                let mut target = display.draw();
                // target.clear_color_srgb(1.0, 1.0, 1.0, 0.0);
                target.clear_all((0.0, 0.0, 0.0, 0.0), 0.0, 0);
                // target.clear_color(0.0, 0.0, 0.0, 0.0);
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

/// Context that is passed to the callback in the render loop
#[derive(Clone, Eq, PartialEq)]
pub struct RenderContext {
    pub bypass_screenshots: bool,
    pub ui_open: bool,
    pub fonts: HashMap<Font, FontId>
}

#[derive(Default, Debug)]
pub struct RenderState(HashMap<String, Box<dyn std::any::Any>>);

impl RenderState {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn set<T: 'static>(&mut self, key: &str, value: T) {
        self.0.insert(key.to_string(), Box::new(value));
    }

    pub fn get<'a, T: 'static>(&mut self, key: &str, default: T) -> &'a mut T {
        self.get_or_else(key, move || default)
    }

    pub fn get_or_else<'a, T: 'static>(&mut self, key: &str, default: impl FnOnce() -> T) -> &'a mut T {
        let n = self.0.entry(key.to_string())
            .or_insert_with(|| Box::new(default()))
            .downcast_mut::<T>()
            .unwrap();
        unsafe { (n as *mut T).as_mut().unwrap() }
    }
}

// Animation for fading in a window. Returns a f32 from 0.0 to 1.0 of the alpha the window should be
fn fade_animation(fade_start: &Option<Instant>, fade_time: Duration, ui_open: bool) -> f32 {
    // Fade in window
    let alpha = match fade_start {
        Some(start) => {
            let elapsed = start.elapsed();
            if elapsed > fade_time {
                if ui_open { 1.0 } else { 0.0 }
            } else {
                let mut alpha = elapsed.as_secs_f32() / fade_time.as_secs_f32();
                if ui_open {
                    alpha = 1.0 - alpha;
                }
                alpha
            }
        }
        None => if ui_open { 1.0 } else { 0.0 }
    };

    f32::clamp(alpha, 0.0, 1.0)
}