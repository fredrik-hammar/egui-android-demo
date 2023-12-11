use eframe::NativeOptions;
use egui_android_demo::DemoApp;

fn main() -> Result<(), eframe::Error> {
    let options = NativeOptions::default();
    DemoApp::run(options)
}
