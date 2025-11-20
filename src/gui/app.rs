#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example
use egui::Color32;
use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};
use std::collections::BTreeMap;

pub fn run_overlay(cpu: String, disk: String, gpu: String, cons: String) -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "SysMon",
        options,
        Box::new(move |cc| Ok(Box::new(MyApp::new(cc, cpu.clone(), disk.clone(), gpu.clone(), cons.clone())))),
    )
}


#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(12.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

fn content(ui: &mut egui::Ui, app: &MyApp) {
    ui.heading(
        RichText::new("~~~ SysMon ~~~")
            .color(Color32::DARK_RED)
            .size(14.0),
    );
    ui.add_space(5.);
    ui.label(RichText::new("Const:").text_style(heading2()).strong());
    ui.monospace(&app.const_info);
    ui.label(RichText::new("CPU Info").text_style(heading2()).strong());
    ui.monospace(&app.cpu_info);
    ui.add_space(10.);
    ui.label(RichText::new("Disk Info").text_style(heading2()).strong());
    ui.monospace(&app.disk_info);
    ui.add_space(10.);
    ui.label(RichText::new("GPU Info").text_style(heading2()).strong());
    ui.monospace(&app.gpu_info);
    ui.add_space(15.);
}


struct MyApp {
    cpu_info: String,
    const_info: String,
    disk_info: String,
    gpu_info: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>, cpu: String, cons: String, disk: String, gpu: String) -> Self {
        configure_text_styles(&cc.egui_ctx);
        Self {
            cpu_info: cpu,
            const_info: cons,
            disk_info: disk,
            gpu_info: gpu,
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| content(ui, self));
    }
}


