use hello_app::*;
use std::error::Error;

mod hello_app {

    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::{Window, WindowBuilder},
        dpi::{Size, PhysicalSize},
    };
    use std::error::Error;

    const WIDTH : u32 = 800;
    const HEIGHT : u32 = 600;

    pub struct HelloApp {
        event_loop : EventLoop<()>,
        window : Window,
    }
    
    impl HelloApp {

        pub fn new() -> HelloApp<> {
            let (event_loop, window) = HelloApp::initWindow();
            HelloApp {
                event_loop, 
                window,
            }
        }

        fn initWindow() -> (EventLoop<()>, Window) {
            let event_loop = EventLoop::new();
            let window = WindowBuilder::new()
                .with_inner_size(Size::from(PhysicalSize::new(WIDTH, HEIGHT)))
                .with_title("Trayracer")
                .with_resizable(false)
                .build(&event_loop)
                .expect("Issue initializing window"); //causes immediate panic! on failure, this seems reasonable
            (event_loop, window)
        }

        pub fn run(&self) {
            self.initVulcan();
            self.mainLoop();
            self.cleanup();
        }
        fn initVulcan(&self) {
            //nothing goes in here yet
        }
        fn mainLoop(&self) {
            //https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code (still stuck at this point, almost finished now)
            //https://github.com/MaikKlein/ash/blob/master/ash-window/examples/winit.rs (old code example, doesn't seem to work)
            //https://github.com/rust-windowing/winit (code below does work but complains about lack of static lifetime)

            // self.event_loop.run(move |event, _, control_flow| {
            //     *control_flow = ControlFlow::Wait;
        
            //     match event {
            //         Event::WindowEvent {
            //             event: WindowEvent::CloseRequested,
            //             window_id,
            //         } if window_id == self.window.id() => *control_flow = ControlFlow::Exit,
            //         _ => (),
            //     }
            // });
        }
        fn cleanup(&self) {
            //some of code above I think perhaps needs to go in here, not certain though
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let app = HelloApp::new();
    app.run();
    Ok(())
}