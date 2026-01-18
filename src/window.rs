use crate::channel;
use std::any::Any;
use std::sync::mpsc::Receiver;
use eframe::{egui, App, CreationContext};

pub fn window_main(rx:Receiver<String>) -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400.0, 800.0]),
        ..Default::default()
    };
    eframe::run_native(
        "protec",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::new(MyApp::new(cc,rx)))
        }),
    )
}

struct MyApp<'a> {
    text: String,
    triggered: bool,
    receiver: Receiver<String>
}

impl MyApp {
    pub(crate) fn new(_cc: &CreationContext<'_>, sender: Receiver<String>) -> Self {
        Self::default(*sender.clone())
    }
    fn default(receiver: Receiver<String>) -> Self {
        Self {
            text: String::new(),
            triggered: false,
            receiver
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        egui::CentralPanel::default().show(
            ctx,
            |ui| {
                ui.heading(self.text.clone());
                match self.receiver.try_recv(){
                    Ok(text) => {
                        if text == "TRIGGER" { 
                            self.triggered = true;
                        }
                    },
                    _=>{}
                }
                if !self.triggered {
                    ui.image(egui::include_image!(
                        "../assets/fezprotec.png"
                    ));
                }
                else{
                    ui.image(egui::include_image!(
                        "../assets/fezaaa.jpg"
                    ));
                }
            });
    }
}