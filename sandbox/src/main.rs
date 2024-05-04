
use eframe::{egui::{self, Button, CentralPanel, Label}, glow::SAMPLER, App, NativeOptions};


#[derive(Default)]
struct Sanbox;


impl App for Sanbox {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui|{
            ui.add(Label::new("Hello World"));
        });
    }
}

fn main() {
    let _ = eframe::run_native("Sandbox", NativeOptions::default(), Box::new(|_cc| {
        Box::new(Sanbox::default())
    }));
}
