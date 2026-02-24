#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]
mod dgg;
mod gui;
mod server;

#[tokio::main]
async fn main() {
    gui::start();
}
