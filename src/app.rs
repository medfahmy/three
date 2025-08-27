//! Primitives for creating and controlling [`Window`](struct.Window.html).

use std::{sync::Arc, path::PathBuf};

use winit::{
    event::{Event, WindowEvent},
    dpi::LogicalSize,
    window::{Window, WindowId},
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};

use crate::{
    // camera::Camera,
    // factory::Factory,
    // glutin::{GlProfile, GlRequest, PossiblyCurrent},
    input::Input,
    // render::Renderer,
    // scene::Scene,
};

/// `Window` is the core entity of every `three-rs` application.
///
/// It provides [user input](struct.Window.html#method.update),
/// [`Factory`](struct.Factory.html) and [`Renderer`](struct.Renderer.html).
#[derive(Default)]
pub struct App {
    dpi: f64,
    // /// See [`Input`](struct.Input.html).
    // pub input: Input,
    // /// See [`Renderer`](struct.Renderer.html).
    // pub renderer: Renderer,
    // /// See [`Factory`](struct.Factory.html).
    // pub factory: Factory,
    // /// See [`Scene`](struct.Scene.html).
    // pub scene: Scene,
    // /// Reset input on each frame? See [`Input::reset`](struct.Input.html#method.reset).
    ///
    /// Defaults to `true`.
    pub reset_input: bool,
    is_fullscreen: bool,
    window: Option<Arc<Window>>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("App::resumed");
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, window_id: WindowId, event: WindowEvent) {
        println!("App::window_event");
    }
}


/// Builder for creating new [`App`](struct.App.html) with desired parameters.
#[derive(Debug, Clone)]
pub struct AppBuilder {
    dimensions: LogicalSize<f64>,
    fullscreen: bool,
    multisampling: u16,
    shader_directory: Option<PathBuf>,
    title: String,
    vsync: bool,
}

impl AppBuilder {
    /// Set the size of the viewport (the resolution) in logical pixels.
    /// That is the dpi setting affects the amount of pixels used but the window will
    /// take up the same amount of space regardless of dpi. Defaults to 1024x768.
    pub fn dimensions(&mut self, width: f64, height: f64) -> &mut Self {
        self.dimensions = LogicalSize::new(width, height);
        self
    }

    /// Whether enable fullscreen mode or not. Defauls to `false`.
    pub fn fullscreen(&mut self, option: bool) -> &mut Self {
        self.fullscreen = option;
        self
    }

    /// Sets the multisampling level to request. A value of `0` indicates that multisampling must
    /// not be enabled. Must be the power of 2. Defaults to `0`.
    pub fn multisampling(&mut self, option: u16) -> &mut Self {
        self.multisampling = option;
        self
    }

    /// Specifies the user shader directory.
    pub fn shader_directory<P: Into<PathBuf>>(&mut self, option: P) -> &mut Self {
        self.shader_directory = Some(option.into());
        self
    }

    /// Whether to enable vertical synchronization or not. Defaults to `true`.
    pub fn vsync(&mut self, option: bool) -> &mut Self {
        self.vsync = option;
        self
    }

    /// Create new `Window` with desired parameters.
    pub fn build(&mut self) -> App {
        // @TODO: run event loop inside here, mutate app from outside using methods
        // let monitor_id = if self.fullscreen { Some(event_loop.get_primary_monitor()) } else { None };
        // let monitor_id = todo!();
        // let is_fullscreen = self.fullscreen;
        //
        // let builder = glutin::WindowBuilder::new().with_fullscreen(monitor_id).with_dimensions(self.dimensions).with_title(self.title.clone());
        // let builder = todo!();
        //
        // let context = glutin::ContextBuilder::new().with_gl_profile(GlProfile::Core).with_gl(GlRequest::Latest).with_vsync(self.vsync).with_multisampling(self.multisampling);
        // let context = todo!();
        //
        // let mut source_set = render::source::Set::default();
        // if let Some(path) = self.shader_directory.as_ref() {
        //     let path = path.to_str().unwrap();
        //     macro_rules! try_override {
        //         ($name:ident) => {
        //             match render::Source::user(path, stringify!($name), "vs") {
        //                 Ok(src) => {
        //                     info!("Overriding {}_vs.glsl", stringify!($name));
        //                     source_set.$name.vs = src;
        //                 }
        //                 Err(err) => {
        //                     error!("{:#?}", err);
        //                     info!("Using default {}_vs.glsl", stringify!($name));
        //                 }
        //             }
        //             match render::Source::user(path, stringify!($name), "ps") {
        //                 Ok(src) => {
        //                     info!("Overriding {}_ps.glsl", stringify!($name));
        //                     source_set.$name.ps = src;
        //                 }
        //                 Err(err) => {
        //                     error!("{:#?}", err);
        //                     info!("Using default {}_ps.glsl", stringify!($name));
        //                 }
        //             }
        //         };
        //         ( $($name:ident,)* ) => {
        //             $( try_override!($name); )*
        //         };
        //     }
        //     try_override!(basic, gouraud, pbr, phong, quad, shadow, skybox, sprite,);
        // }
        //
        // let (renderer, windowedContext, mut factory) = Renderer::new(builder, context, &event_loop, &source_set);
        // let dpi = windowedContext.window().get_hidpi_factor();
        // let scene = factory.scene();
        // Window { event_loop, windowedContext, dpi, input: Input::new(), renderer, factory, scene, reset_input: true, is_fullscreen }


            let event_loop = EventLoop::new().unwrap();

            // @TODO: add field for control flow
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            event_loop.set_control_flow(ControlFlow::Poll);

            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            // event_loop.set_control_flow(ControlFlow::Wait);

            let mut app = App::default();
            println!("running app");
            event_loop.run_app(&mut app);

        // todo!()
            app
    }
}

impl App {
    /// Create a new window with default parameters.
    pub fn new<T: Into<String>>(title: T) -> Self {
        Self::builder(title).build()
    }

    /// Create new `Builder` with standard parameters.
    pub fn builder<T: Into<String>>(title: T) -> AppBuilder {
        AppBuilder { dimensions: LogicalSize::new(1024.0, 768.0), fullscreen: false, multisampling: 0, shader_directory: None, title: title.into(), vsync: true }
    }

    /// `update` method returns `false` if the window was closed.
    pub fn update(&mut self) -> bool {
        // let mut running = true;
        // let renderer = &mut self.renderer;
        // let input = &mut self.input;
        // if self.reset_input {
        //     input.reset();
        // }
        //
        // let wc = &self.windowedContext;
        // self.windowedContext.swap_buffers().unwrap();
        // let dpi = self.dpi;
        //
        // self.event_loop.poll_events(|event| {
        //     use winit::WindowEvent;
        //     match event {
        //         Event::WindowEvent { event, .. } => match event {
        //             WindowEvent::Resized(size) => renderer.resize(wc, size),
        //             WindowEvent::HiDpiFactorChanged(dpi) => renderer.dpi_change(wc, dpi),
        //             WindowEvent::Focused(state) => input.window_focus(state),
        //             WindowEvent::CloseRequested | WindowEvent::Destroyed => running = false,
        //             WindowEvent::KeyboardInput { input: winit::keyboard::KeyboardInput { state, virtual_keycode: Some(keycode), .. }, .. } => input.keyboard_input(state, keycode),
        //             WindowEvent::MouseInput { state, button, .. } => input.mouse_input(state, button),
        //             WindowEvent::CursorMoved { position, .. } => {
        //                 let pos = position.to_physical(dpi);
        //                 input.mouse_moved([pos.x as f32, pos.y as f32].into(), renderer.map_to_ndc([pos.x as f32, pos.y as f32]));
        //             }
        //             WindowEvent::MouseWheel { delta, .. } => input.mouse_wheel_input(delta),
        //             _ => {}
        //         },
        //         Event::DeviceEvent { event, .. } => match event {
        //             winit::DeviceEvent::Motion { axis, value } => {
        //                 input.axis_moved_raw(axis as u8, value as f32);
        //             }
        //             _ => {}
        //         },
        //         _ => {}
        //     }
        // });
        //
        // running

        todo!()
    }

    // /// Render the current scene with specific [`Camera`](struct.Camera.html).
    // pub fn render(&mut self, camera: &Camera) {
    //     self.renderer.render(&self.scene, camera);
    // }

    /// Get current window size in pixels.
    pub fn size(&self) -> mint::Vector2<f32> {
        // let size = self.windowedContext.window().get_inner_size().expect("Can't get window size").to_physical(self.dpi);
        // [size.width as f32, size.height as f32].into()

        todo!()
    }

    // /// Returns underlaying `glutin::WindowedContext`.
    // #[cfg(feature = "opengl")]
    // pub fn glutin_window(&self) -> &glutin::WindowedContext<PossiblyCurrent> {
    //     &self.windowedContext
    // }

    /// Returns the current full screen mode.
    pub fn is_fullscreen(&self) -> bool {
        self.is_fullscreen
    }

    /// Sets the full screen mode.
    /// If the window is already in full screen mode, does nothing.
    pub fn set_fullscreen(&mut self, fullscreen: bool) {
        if self.is_fullscreen == fullscreen {
            return;
        }

        // self.is_fullscreen = fullscreen;
        // let monitor = if fullscreen { Some(self.event_loop.get_primary_monitor()) } else { None };
        // self.windowedContext.window().set_fullscreen(monitor);
    }

    /// Toggles the full screen mode.
    /// Returns the new actual mode.
    pub fn toggle_fullscreen(&mut self) -> bool {
        let fullscreen = !self.is_fullscreen;
        self.set_fullscreen(fullscreen);
        fullscreen
    }
}
