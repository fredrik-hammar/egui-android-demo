use eframe::{egui, NativeOptions};

#[cfg(target_os = "android")]
use egui_winit::winit;
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: winit::platform::android::activity::AndroidApp) {
    use winit::platform::android::EventLoopBuilderExtAndroid;
    use eframe::Renderer;

    std::env::set_var("RUST_BACKTRACE", "full");
    android_logger::init_once(android_logger::Config::default().with_max_level(log::LevelFilter::Info));

    let options = NativeOptions {
        event_loop_builder: Some(Box::new(|builder| {
            builder.with_android_app(app);
        })),
        renderer: Renderer::Wgpu,
        ..Default::default()
    };
    DemoApp::run(options).unwrap();
}

#[derive(Default)]
pub struct DemoApp {
    demo_windows: egui_demo_lib::DemoWindows,
}

impl DemoApp {
    pub fn run(options: NativeOptions) -> Result<(), eframe::Error> {
        eframe::run_native(
            "egui-android-demo",
            options,
            Box::new(|_cc| Ok(Box::<DemoApp>::default())),
        )
    }
}

impl eframe::App for DemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_windows.ui(ctx);
    }
}
