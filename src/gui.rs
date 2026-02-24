use crate::dgg;
use anyhow::Result;
use eframe::egui;
use std::io::Write;
use std::{f32, fs::File};
use tokio::{sync::mpsc, task::JoinHandle};

struct App {
    use_command: bool,
    command: String,
    time: u64,

    handle: Option<JoinHandle<Result<Vec<String>>>>,
    receiver: Option<mpsc::UnboundedReceiver<String>>,
    usernames: Vec<String>,
}
impl App {
    fn save_to_csv(&self) {
        if self.usernames.is_empty() {
            return;
        }
        let mut file = File::create("usernames.csv").expect("Unable to create file");

        for name in &self.usernames {
            writeln!(file, "{}", name).unwrap();
        }
        println!("Saved {} usernames to usernames.csv", self.usernames.len());
    }
}
impl Default for App {
    fn default() -> Self {
        Self {
            use_command: false,
            command: String::from("!join"),
            time: 300,
            handle: None,
            receiver: None,
            usernames: Vec::new(),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        if let Some(ref mut rx) = self.receiver {
            while let Ok(nick) = rx.try_recv() {
                self.usernames.push(nick);
            }
        }

        if let Some(handle) = &self.handle {
            if handle.is_finished() {
                self.handle = None;
                self.receiver = None;
                self.save_to_csv();
                println!("Connection closed.");
            }
        }
        egui::SidePanel::right("users").show(ctx, |ui| {
            ui.heading(format!("Users List: {}", self.usernames.len()));
            ui.separator();

            let mut scroll = egui::ScrollArea::vertical()
                .auto_shrink(false)
                .max_height(f32::INFINITY);
            if self.handle.is_some() {
                scroll = scroll.stick_to_bottom(true);
            }
            scroll.show(ui, |ui| {
                for name in &self.usernames {
                    ui.label(name);
                }
            })
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            let is_running = self.handle.is_some();
            ui.heading("DGG Marbles Manager");

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.use_command, "Use Command?");
                if self.use_command {
                    ui.label("Command: ");
                    ui.text_edit_singleline(&mut self.command);
                }
            });
            ui.horizontal(|ui| {
                ui.label("Time (seconds): ");
                ui.add(egui::DragValue::new(&mut self.time).speed(0.7));
            });
            ui.horizontal(|ui| {
                if ui
                    .add_enabled(!is_running, egui::Button::new("Start"))
                    .clicked()
                {
                    let (tx, rx) = mpsc::unbounded_channel();
                    self.receiver = Some(rx);
                    self.usernames.clear();

                    let time = self.time;
                    let use_command = self.use_command;
                    let command = self.command.clone();

                    self.handle = Some(tokio::spawn(async move {
                        dgg::connect(time, tx, use_command, command).await
                    }));
                }

                if ui
                    .add_enabled(is_running, egui::Button::new("Cancel/Stop"))
                    .clicked()
                {
                    if let Some(handle) = self.handle.take() {
                        handle.abort();
                        self.receiver = None;
                        self.save_to_csv();
                    }
                }
            });
            if is_running {
                ui.label(format!(
                    "Running... Collected {} users",
                    self.usernames.len()
                ));
                ui.spinner();
            }
        });
        if self.handle.is_some() {
            ctx.request_repaint();
        }
    }
}
pub fn start() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([720.0, 430.0]),
        ..Default::default()
    };
    eframe::run_native(
        "DGG Marbles Manager",
        options,
        Box::new(|_| Ok(Box::<App>::default())),
    )
    .unwrap();
}
