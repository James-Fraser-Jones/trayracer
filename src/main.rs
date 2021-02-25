use hello_app::*;

mod hello_app {
    pub struct HelloApp();
    
    impl HelloApp {
        pub fn run(&self) {
            HelloApp::initWindow();
            HelloApp::initVulcan();
            HelloApp::mainLoop();
            HelloApp::cleanup();
        }

        fn initWindow() {
            /*
            We're currently working on opening a window here, 
            having logic in "initVulcan" to keep looping while the window remains open 
            and then doing cleanup when it gets closed in "cleanup"
            https://github.com/MaikKlein/ash/blob/master/ash-window/examples/winit.rs
            https://vulkan-tutorial.com/Drawing_a_triangle/Setup/Base_code
            */
        }
    
        fn initVulcan() {
    
        }
    
        fn mainLoop() {
    
        }
    
        fn cleanup() {
    
        }
    }
}

fn main() {
    let app = HelloApp();
    app.run(); //should have "try-catch"-like error handling here but probably not necessary in rust???
    println!("Hello, world!");
}