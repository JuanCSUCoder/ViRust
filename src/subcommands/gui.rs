use log::info;

/// Starts the Graphical User Interface of the Benchmark Tool
pub fn start_gui() {
    eframe::run_native(
        "ViRust Brutal Benchmarking Application",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Box::<BenchmarkApplication>::default())
    ).expect("Unable to start GUI app");
}

/// Benchmark Application State
struct BenchmarkApplication {
    memory_amount: u64,
}

impl Default for BenchmarkApplication {
    fn default() -> Self {
        Self { memory_amount: 100 }
    }
}

impl eframe::App for BenchmarkApplication {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("ViRust Brutal Benchmarking Application");
            ui.add(eframe::egui::Slider::new(&mut self.memory_amount, 100..=50000000));
            if ui.button("Fill Memory").clicked() {
                info!("Executing memory fill of {} KB ...", &self.memory_amount);
            }
        });
    }
}
