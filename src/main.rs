use ash::{vk, Entry, version::EntryV1_0};

//supposed "hello world" example below but doesn't actually compile
//nor is most of it used in triangle example in ash_master project

// let entry = Entry::new()?;
// let app_info = vk::ApplicationInfo {
//     api_version: vk::make_version(1, 0, 0),
//     ..Default::default()
// };
// let create_info = vk::InstanceCreateInfo {
//     p_application_info: &app_info,
//     ..Default::default()
// };
// let instance = unsafe { entry.create_instance(&create_info, None)? };

fn main() {
    println!("Hello, world!");
}

//next step: implement rust translation of this (https://vulkan-tutorial.com/en/Drawing_a_triangle/Setup/Base_code)