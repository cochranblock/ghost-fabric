use android_activity::AndroidApp;
use eframe::NativeOptions;

/// f12=android_main — Android native activity entry point
#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default().with_max_level(log::LevelFilter::Info),
    );

    // Set HOME for config path resolution
    if let Some(path) = app.internal_data_path() {
        unsafe { std::env::set_var("HOME", path.to_string_lossy().as_ref()) };
    }

    // Auto-init node on first launch
    let data_dir = app.internal_data_path().map(|p| p.to_path_buf());
    let init_msg = ghost_fabric_core::f11(data_dir);
    log::info!("{}", init_msg);

    let options = NativeOptions {
        android_app: Some(app),
        ..Default::default()
    };

    eframe::run_native(
        "Ghost Fabric",
        options,
        Box::new(|_cc| Ok(Box::new(GhostApp::default()))),
    )
    .expect("eframe");
}

struct GhostApp {
    status: String,
}

impl Default for GhostApp {
    fn default() -> Self {
        Self {
            status: ghost_fabric_core::f10(),
        }
    }
}

impl eframe::App for GhostApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_pixels_per_point(2.5);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Ghost Fabric");
            ui.separator();
            ui.monospace(&self.status);
            ui.separator();
            if ui.button("Refresh").clicked() {
                self.status = ghost_fabric_core::f10();
            }
        });
    }
}
