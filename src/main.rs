extern crate libc;

use std::error::Error;
use std::ffi::{CString};

//use libc::c_char;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder},
    dpi::{Size, LogicalSize},
};
use ash::{vk, Entry, version::EntryV1_0};

// use hello_triangle_application::*;
// mod hello_triangle_application {
//     use super::*;
//     pub struct HelloTriangleApplication {
//         instance: Instance,
//     }
//     impl HelloTriangleApplication {
//         pub fn new(instance: Instance) -> Self {
//             HelloTriangleApplication{
//                 instance
//             }
//         }
//         pub fn run(&mut self) {
//             self.initVulkan();
//             self.mainLoop();
//             self.cleanup();
//         }
//         fn initVulkan(&mut self) {
    
//         }
//         fn mainLoop(&mut self) {
            
//         }
//         fn cleanup(&mut self) {
    
//         }
//     }
// }

fn main() -> Result<(), Box<dyn Error>> {
    //window creation
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
                .with_inner_size(Size::new(LogicalSize{width: 800, height: 600}))
                .with_title("Vulcan")
                .with_resizable(false)
                .build(&event_loop)?;
    
    //instance creation
    let app_info = vk::ApplicationInfo::builder()
        .application_name(&CString::new("Hello Triangle")?)
        .application_version(vk::make_version(1, 0, 0))
        .engine_name(&CString::new("No Engine")?)
        .engine_version(vk::make_version(1, 0, 0))
        .api_version(vk::make_version(1, 0, 0))
        .build();
    let surface_extensions = ash_window::enumerate_required_extensions(&window)?;
    // for surface_extension in &surface_extensions {
    //     println!("{:?}", surface_extension);
    // }
    let instance_extensions = surface_extensions.iter().map(|ext| ext.as_ptr()).collect::<Vec<_>>();
    let create_info = vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .enabled_extension_names(&instance_extensions)
        .build();
    let entry = unsafe { Entry::new()? }; //entry point into Vulkan???
    let instance = unsafe { entry.create_instance(&create_info, None)? };

    //app creation
    //let app = HelloTriangleApplication::new(instance);
    
    //check that vulkcan has all necessary extensions available for window surface
    let available_extensions: Vec<String> = entry.enumerate_instance_extension_properties()?
        .into_iter()
        .map(|ext| {
            let name = ext.extension_name.to_vec().into_iter().map(|x| x as u8).collect::<Vec<_>>();
            String::from_utf8(name).unwrap().to_owned() //trimming doesn't seem to fix massive empty space at the end :/
        })
        .collect();
    
    for available_extension in available_extensions {
        print!("{}", available_extension);
    }

    //window loop
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            _ => (),
        }
    });
}