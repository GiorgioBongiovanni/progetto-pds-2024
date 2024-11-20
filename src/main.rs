mod ui;
mod caster;
mod receiver;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "Screencasting App",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(ui::MyApp::new(cc)))),
    )
}




