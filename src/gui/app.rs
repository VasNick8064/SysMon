#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(rustdoc::missing_crate_level_docs)]

use egui::Color32;
use eframe::egui;
use egui::{FontFamily, FontId, RichText, TextStyle};
use std::collections::BTreeMap;
use sysinfo::System;
use crate::get_info;

pub fn run_overlay(const_info: String) -> eframe::Result {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "SysMon",
        options,
        Box::new(move |cc| Ok(Box::new(MyApp::new(cc, const_info.clone())))),
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
    ].into();
    ctx.all_styles_mut(move |style| style.text_styles = text_styles.clone());
}

// Основная структура приложения
struct MyApp {
    sys: System,
    last_update: std::time::Instant,

    const_info: String,
    cpu_info: String,
    disk_info: String,
    gpu_info: String,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>, const_info: String) -> Self {
        configure_text_styles(&cc.egui_ctx);

        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_info = get_info::get_cpu_info(&sys);
        let disk_info = get_info::get_disk_info(&sys);
        let gpu_info = get_info::display_gpu_info();

        Self {
            sys,
            last_update: std::time::Instant::now(),
            const_info,
            cpu_info,
            disk_info,
            gpu_info,
        }
    }
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

    ui.label(RichText::new("Disk Info").text_style(heading2()).strong());
    ui.monospace(&app.disk_info);

    ui.label(RichText::new("GPU Info").text_style(heading2()).strong());
    ui.monospace(&app.gpu_info);
    ui.add_space(15.);
}

// Каждый кадр обновляем метрики, кроме const_info
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Раз в 2 секунду обновляем инфу
        if self.last_update.elapsed().as_secs() >= 2 {
            self.sys.refresh_all();

            self.cpu_info = get_info::get_cpu_info(&self.sys);
            self.disk_info = get_info::get_disk_info(&self.sys);
            self.gpu_info = get_info::display_gpu_info();

            self.last_update = std::time::Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| content(ui, self));
        // Просим egui редерить интерфейс постоянно (чтобы таймер работал)
        ctx.request_repaint();
    }
}
