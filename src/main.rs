#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod dgg;
mod gui;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    gui::start();
}
