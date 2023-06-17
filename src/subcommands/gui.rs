use std::ops::RangeInclusive;

use log::info;
use eframe::egui::{self, Vec2};
use thousands::Separable;

/// Starts the Graphical User Interface of the Benchmark Tool
pub fn start_gui() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(300.0, 100.0)),
        ..Default::default()
    };

    eframe::run_native(
        "ViRust Brutal Benchmarking Application",
        options,
        Box::new(|_cc| Box::<BenchmarkApplication>::default())
    ).expect("Unable to start GUI app");
}

fn format_bytes(n: f64, _: RangeInclusive<usize>) -> String {
    let n = n as u64;

    if n >= 10000000 {
        format!("{} GB", (n/1000000).separate_with_dots())
    } else if n >= 1000000 {
        format!("{} MB", (n/1000).separate_with_dots())
    } else {
        format!("{} KB", n.separate_with_dots())
    }
}

fn custom_drag_value(value: &mut u64) -> egui::DragValue<'_> {
    let actual_value = value.clone();

    egui::DragValue::new(value).fixed_decimals(0).clamp_range(1000..=50000000)
    .speed(if actual_value >= 10000000 {50000} else if actual_value >= 1000000 {5000} else {50})
    .custom_formatter(format_bytes).custom_parser(|x| x.parse::<f64>().ok())
}

/// Benchmark Application State
struct BenchmarkApplication {
    min_amount: u64,
    max_amount: u64,
    memory_amount: u64,
}

impl Default for BenchmarkApplication {
    fn default() -> Self {
        Self {
            min_amount: 1000,
            max_amount: 2000000,
            memory_amount: 100
        }
    }
}

impl eframe::App for BenchmarkApplication {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.heading("ViRust Brutal Benchmarking");
            ui.horizontal(|ui| {
                ui.label("Min:");
                ui.add(custom_drag_value(&mut self.min_amount));
                ui.label("Max:");
                ui.add(custom_drag_value(&mut self.max_amount));
            });
            ui.add(
                eframe::egui::Slider::new(&mut self.memory_amount, self.min_amount..=self.max_amount)
                    .custom_formatter(format_bytes)
            );
            if ui.button("Fill Memory").clicked() {
                info!("Executing memory fill of {} KB ...", &self.memory_amount);
            }
        });
    }
}
