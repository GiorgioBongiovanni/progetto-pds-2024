use eframe::egui;
use std::thread;

// Importa le funzioni di caster e receiver
use crate::caster::capture_and_send_frames;
use crate::receiver::receive_frames;

pub struct MyApp {
    operating_mode: Option<String>,
    caster_address: String,
    receivers: Vec<String>, // Lista degli indirizzi dei receiver
    receiver_address: String, // Indirizzo locale del receiver
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            operating_mode: None,
            caster_address: "127.0.0.1:8080".to_string(),
            receivers: vec!["127.0.0.1:9090".to_string()], // Receiver di test
            receiver_address: "127.0.0.1:9090".to_string(),
        }
    }
}

impl MyApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(mode) = self.operating_mode.as_deref() {
                match mode {
                    "caster" => {
                        // Modalità Caster
                        ui.label("Caster Mode");
                        ui.horizontal(|ui| {
                            ui.label("Your Address:");
                            ui.text_edit_singleline(&mut self.caster_address);
                        });

                        ui.label("Receivers:");
                        for receiver in &self.receivers {
                            ui.label(receiver);
                        }

                        if ui.button("Start Casting").clicked() {
                            let caster_address = self.caster_address.clone();
                            let receivers = self.receivers.clone();

                            // Avvia un thread separato per la trasmissione
                            thread::spawn(move || {
                                if let Err(e) =
                                    capture_and_send_frames(&caster_address)
                                {
                                    eprintln!("Error in caster thread: {}", e);
                                }
                            });
                        }
                    }
                    "receiver" => {
                        // Modalità Receiver
                        ui.label("Receiver Mode");
                        ui.horizontal(|ui| {
                            ui.label("Your Address:");
                            ui.text_edit_singleline(&mut self.receiver_address);
                        });

                        if ui.button("Connect to Caster").clicked() {
                            let receiver_address = self.receiver_address.clone();

                            // Avvia un thread per il receiver
                            thread::spawn(move || {
                                if let Err(e) = receive_frames(&"127.0.0.1:8080") {
                                    eprintln!("Error in receiver thread: {}", e);
                                }
                            });

                            println!("Receiver started at {}", self.receiver_address);
                        }
                    }
                    _ => {}
                }
            } else {
                // Selezione iniziale: Caster o Receiver
                ui.label("Choose the operating mode:");
                if ui.button("Caster").clicked() {
                    self.operating_mode = Some("caster".to_string());
                }
                if ui.button("Receiver").clicked() {
                    self.operating_mode = Some("receiver".to_string());
                }
            }
        });
    }
}


