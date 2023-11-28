use eframe::egui;
use egui_winit::winit;

#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use eframe::{NativeOptions, Renderer};
    use winit::platform::android::EventLoopBuilderExtAndroid;

    std::env::set_var("RUST_BACKTRACE", "full");
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let options = NativeOptions {
        event_loop_builder: Some(Box::new(|builder| {
            builder.with_android_app(app);
        })),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "egui-android-demo",
        options,
        Box::new(|_cc| Box::<DemoApp>::default()),
    ).unwrap();
}

#[derive(Default)]
struct DemoApp {
    demo_windows: egui_demo_lib::DemoWindows,
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_windows.ui(ctx);
    }
}
