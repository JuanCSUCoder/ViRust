use std::ops::RangeInclusive;

use log::{info, warn};
use eframe::egui::{self, Vec2};
use subprocess::{Exec, Popen};
use thousands::Separable;

/// Starts the Graphical User Interface of the Benchmark Tool
pub fn start_gui() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(270.0, 150.0)),
        renderer: eframe::Renderer::Wgpu,
        resizable: false,
        ..Default::default()
    };

    eframe::run_native(
        "ViRust Memory Tester",
        options,
        Box::new(|_cc| Box::<BenchmarkApplication>::default())
    ).expect("Unable to start GUI app");
}

fn format_bytes(n: f64, _: RangeInclusive<usize>) -> String {
    let n = n as u64;

    if n >= 3_000_000 {
        format!("{} GB", (n/1_000_000).separate_with_dots())
    } else if n >= 3_000 {
        format!("{} MB", (n/1_000).separate_with_dots())
    } else {
        format!("{} KB", n.separate_with_dots())
    }
}

fn custom_drag_value(value: &mut u64) -> egui::DragValue<'_> {
    let actual_value = value.clone();

    egui::DragValue::new(value).fixed_decimals(0).clamp_range(1_000..=50_000_000)
    .speed(if actual_value >= 3_000_000 {50_000} else if actual_value >= 3_000 {5_000} else {50})
    .custom_formatter(format_bytes).custom_parser(|x| x.parse::<f64>().ok())
}

/// Benchmark Application State
struct BenchmarkApplication {
    min_amount: u64,
    max_amount: u64,
    memory_amount: u64,
    process: Option<Popen>,
    loaded: bool,
}

impl Default for BenchmarkApplication {
    fn default() -> Self {
        Self {
            min_amount: 1000,
            max_amount: 2000000,
            memory_amount: 100,
            process: None,
            loaded: false,
        }
    }
}

impl eframe::App for BenchmarkApplication {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |ui| {
            ui.style_mut().wrap = Some(false);
            ui.heading("ViRust Memory Tester");
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

            self.loaded = if let Some(x) = &mut self.process {
                if x.poll().is_some() {false} else {true}
            } else {
                false
            };

            if self.loaded {
                ui.label("Status: Loaded");

                if ui.button("Release Memory").clicked() {
                    info!("Releasing memory ...");

                    self.process.as_mut().unwrap().terminate().unwrap();
                }
            } else {
                ui.label("Status: Idle");

                if ui.button("Fill Memory").clicked() {
                    info!("Executing memory fill of {} KB ...", self.memory_amount);

                    self.process = Exec::cmd(std::env::current_exe().unwrap().canonicalize().unwrap().as_os_str()).args(&["memory".to_string(), "-k".to_string(), self.memory_amount.to_string()]).popen().ok();

                    info!("Memory Process Instantiated at {} PID", self.process.as_ref().unwrap().pid().unwrap());
                }
            }

            ui.label("By @JuanCSUCoder. License: MIT");
            ui.hyperlink("https://github.com/JuanCSUCoder/ViRust");
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        info!("Closing requested");
        if let Some(p) = &mut self.process {
            warn!("Process detected!");
            if p.poll().is_none() {
                p.terminate().unwrap();
                warn!("Cleaning unclear memory");
                info!("Memory cleared! Terminating ...");
            }
        }
    }
}
