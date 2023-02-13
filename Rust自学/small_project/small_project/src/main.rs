// A small project using Rust to write a snake game
mod app;


fn main() {
    
    let native_options = eframe::NativeOptions::default();
    eframe::run_native("My egui App", native_options, Box::new(|cc| Box::new(app::MyEguiApp::new(cc))));

}