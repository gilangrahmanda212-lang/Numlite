use eframe::egui;
use crate::eval::{exec_stmt, eval, Env};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use crate::gui::PlotData;
use egui::{Color32, Rounding};
use std::fs;
use std::path::PathBuf;

pub struct CodeTab {
    pub name: String,
    pub code: String,
    pub is_saved: bool,
    pub file_path: Option<PathBuf>,
}

pub struct NumLiteApp {
    tabs: Vec<CodeTab>,
    active_tab: usize,
    output: String,
    env: Env,
    plots: Vec<PlotData>,
    popup_plot: Option<usize>,
    menu_open: bool, 
}

impl Default for NumLiteApp {
    fn default() -> Self {
        Self {
            tabs: vec![CodeTab {
                name: "unsaved-code-1".to_string(),
                code: String::from("% Selamat datang di NumLite!\n% Tulis kode MATLAB Anda di sini.\n"),
                is_saved: false,
                file_path: None,
            }],
            active_tab: 0,
            output: String::from("Selamat datang di NumLite GUI!\n"),
            env: Env::new(),
            plots: Vec::new(),
            popup_plot: None,
            menu_open: false,
        }
    }
}

impl NumLiteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = Color32::from_rgb(25, 25, 25); 
        visuals.window_fill = Color32::from_rgb(30, 30, 30);
        _cc.egui_ctx.set_visuals(visuals);
        
        Self::default()
    }

    fn get_next_unsaved_name(&self) -> String {
        let mut i = 1;
        loop {
            let name = format!("unsaved-code-{}", i);
            if !self.tabs.iter().any(|t| t.name == name) {
                return name;
            }
            i += 1;
        }
    }

    fn new_tab(&mut self) {
        let name = self.get_next_unsaved_name();
        self.tabs.push(CodeTab {
            name,
            code: String::new(),
            is_saved: false,
            file_path: None,
        });
        self.active_tab = self.tabs.len() - 1;
    }

    fn save_file(&mut self) {
        if self.tabs.is_empty() { return; }
        let tab = &mut self.tabs[self.active_tab];

        if tab.is_saved {
            if let Some(path) = &tab.file_path {
                let _ = fs::write(path, &tab.code);
            }
        } else {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("MATLAB Script", &["m"])
                .set_file_name("script.m")
                .save_file() 
            {
                let _ = fs::write(&path, &tab.code);
                tab.file_path = Some(path.clone());
                if let Some(stem) = path.file_stem() {
                    tab.name = stem.to_string_lossy().to_string();
                }
                tab.is_saved = true;
            }
        }
    }

    fn open_file(&mut self) {
        if let Some(path) = rfd::FileDialog::new()
            .add_filter("MATLAB Script", &["m"])
            .pick_file() 
        {
            if let Ok(content) = fs::read_to_string(&path) {
                let mut name = String::from("unknown");
                if let Some(stem) = path.file_stem() {
                    name = stem.to_string_lossy().to_string();
                }
                
                self.tabs.push(CodeTab {
                    name,
                    code: content,
                    is_saved: true,
                    file_path: Some(path),
                });
                self.active_tab = self.tabs.len() - 1;
                self.menu_open = false; 
            }
        }
    }

    fn run_code(&mut self) {
        self.output.clear();
        self.plots.clear();
        self.popup_plot = None;
        
        if self.tabs.is_empty() { return; }
        let code = self.tabs[self.active_tab].code.clone();
        
        let mut lexer = Lexer::new(&code);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => { self.output = format!("Lexer Error: {}\n", e); return; }
        };

        let mut parser = Parser::new(tokens);
        let mut captured_output = String::new();

        while !parser.is_at_eof() {
            match parser.parse_stmt() {
                Ok(stmt) => {
                    if let crate::ast::Stmt::Plot { title, x, y } = stmt {
                        let x_val = eval(&x, &self.env).unwrap_or(Value::Scalar(0.0)).as_matrix();
                        let y_val = eval(&y, &self.env).unwrap_or(Value::Scalar(0.0)).as_matrix();
                        self.plots.push(PlotData { title, x: x_val.data, y: y_val.data });
                    } else {
                        if let Err(e) = exec_stmt(&stmt, &mut self.env, &mut captured_output, false) {
                            captured_output.push_str(&format!("Error: {}\n", e));
                            break;
                        }
                    }
                }
                Err(e) => {
                    if e != "Eof" { captured_output.push_str(&format!("Parser Error: {}\n", e)); }
                    break;
                }
            }
        }
        
        self.output = captured_output;
        if self.output.is_empty() {
            self.output = String::from("Eksekusi berhasil.\n");
        }
    }

    fn draw_plot(&self, ui: &mut egui::Ui, plot: &PlotData, height: f32) {
        // PERBAIKAN: Menambahkan margin_top agar teks atas tidak terpotong
        let margin_left = 35.0; 
        let margin_top = 15.0; 
        let margin_bottom = 20.0;
        
        let (response, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), height), egui::Sense::hover());
        let full_rect = response.rect;
        
        let rect = egui::Rect::from_min_max(
            egui::pos2(full_rect.left() + margin_left, full_rect.top() + margin_top),
            egui::pos2(full_rect.right(), full_rect.bottom() - margin_bottom)
        );

        if plot.x.is_empty() || plot.y.is_empty() { return; }
        let min_x = plot.x.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_x = plot.x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_y = plot.y.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = plot.y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let range_x = if (max_x - min_x).abs() < 1e-9 { 1.0 } else { max_x - min_x };
        let range_y = if (max_y - min_y).abs() < 1e-9 { 1.0 } else { max_y - min_y };

        painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 30));
        let axis_pen = egui::Stroke::new(1.0_f32, Color32::GRAY);
        painter.line_segment([rect.left_bottom(), rect.right_bottom()], axis_pen);
        painter.line_segment([rect.left_bottom(), rect.left_top()], axis_pen);

        let font = egui::FontId::proportional(9.0);
        let text_color = Color32::LIGHT_GRAY;
        let grid_pen = egui::Stroke::new(1.0_f32, Color32::from_rgb(50, 50, 50));

        for i in 0..=4 {
            let ratio = i as f64 / 4.0;
            let y_val = min_y + ratio * range_y;
            let py = rect.bottom() - (ratio * rect.height() as f64) as f32;
            painter.line_segment([egui::pos2(rect.left(), py), egui::pos2(rect.right(), py)], grid_pen);
            painter.text(egui::pos2(rect.left() - 3.0, py), egui::Align2::RIGHT_CENTER, format!("{:.1}", y_val), font.clone(), text_color);
            
            let x_val = min_x + ratio * range_x;
            let px = rect.left() + (ratio * rect.width() as f64) as f32;
            painter.line_segment([egui::pos2(px, rect.top()), egui::pos2(px, rect.bottom())], grid_pen);
            painter.text(egui::pos2(px, rect.bottom() + 3.0), egui::Align2::CENTER_TOP, format!("{:.1}", x_val), font.clone(), text_color);
        }

        let mut points = Vec::new();
        for i in 0..plot.x.len() {
            let px = rect.left() + ((plot.x[i] - min_x) / range_x * rect.width() as f64) as f32;
            let py = rect.bottom() - ((plot.y[i] - min_y) / range_y * rect.height() as f64) as f32;
            points.push(egui::pos2(px, py));
        }
        if points.len() > 1 {
            painter.add(egui::Shape::line(points, egui::Stroke::new(2.0_f32, Color32::GREEN)));
        }
    }
}

impl eframe::App for NumLiteApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let bg_editor = Color32::from_rgb(30, 30, 30); 
        let bg_tab_inactive = Color32::from_rgb(15, 15, 15); 
        let bg_bar = Color32::from_rgb(20, 20, 20); 
        
        let margin_header = egui::Margin { left: 10.0, right: 10.0, top: 10.0, bottom: 10.0 };
        let margin_konten = egui::Margin { left: 10.0, right: 10.0, top: 8.0, bottom: 8.0 };

        // 1. TOP PANEL
        egui::TopBottomPanel::top("top_panel")
            .frame(egui::Frame::none().fill(bg_bar).inner_margin(margin_header))
            .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let burger_color = if self.menu_open { Color32::GREEN } else { Color32::LIGHT_GRAY };
                let burger_btn = egui::Button::new(egui::RichText::new("☰").size(24.0).color(burger_color)).frame(false);
                
                if ui.add_sized([36.0, 36.0], burger_btn).clicked() { 
                    self.menu_open = !self.menu_open; 
                }
                
                ui.label(egui::RichText::new("NumLite").color(Color32::GREEN).size(28.0).strong());
                ui.separator();
                
                if let Some(tab) = self.tabs.get(self.active_tab) {
                    ui.label(egui::RichText::new(&tab.name).color(Color32::WHITE).size(18.0));
                }
            });
        });

        // 2. SIDEBAR BURGER MENU
        if self.menu_open {
            egui::SidePanel::left("burger_menu")
                .exact_width(240.0)
                .resizable(false)
                .frame(egui::Frame::none().fill(Color32::from_rgb(35, 35, 35)).inner_margin(15.0))
                .show(ctx, |ui| {
                    ui.heading(egui::RichText::new("File Menu").size(18.0).color(Color32::LIGHT_GRAY));
                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(15.0);

                    let text_style = |text: &str| egui::RichText::new(text).size(16.0);
                    let btn_rounding = Rounding::same(8.0);

                    if ui.add_sized([210.0, 40.0], egui::Button::new(text_style("➕  New File")).rounding(btn_rounding)).clicked() {
                        self.new_tab();
                        self.menu_open = false;
                    }
                    ui.add_space(8.0); 
                    if ui.add_sized([210.0, 40.0], egui::Button::new(text_style("📂  Open File")).rounding(btn_rounding)).clicked() {
                        self.open_file();
                    }
                });
        }

        // 3. TAB BAR
        egui::TopBottomPanel::top("tab_panel")
            .frame(egui::Frame::none().fill(bg_bar).inner_margin(egui::Margin { left: 10.0, right: 10.0, top: 5.0, bottom: 0.0 }))
            .show(ctx, |ui| {
            ui.horizontal(|ui| {
                let mut tab_to_close = None;
                for (i, tab) in self.tabs.iter().enumerate() {
                    let is_active = i == self.active_tab;
                    let bg_color = if is_active { bg_editor } else { bg_tab_inactive };
                    
                    let rounding = if is_active { Rounding { nw: 8.0, ne: 8.0, sw: 0.0, se: 0.0 } } else { Rounding::default() };
                    
                    let frame = egui::Frame::none()
                        .fill(bg_color)
                        .inner_margin(egui::Margin { left: 12.0, right: 8.0, top: 8.0, bottom: 8.0 })
                        .rounding(rounding);

                    frame.show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if is_active { ui.label(egui::RichText::new("|").color(Color32::GREEN)); }
                            if ui.add(egui::Button::new(egui::RichText::new(&tab.name).color(Color32::WHITE)).frame(false)).clicked() {
                                self.active_tab = i;
                            }
                            if ui.add(egui::Button::new(egui::RichText::new("×").color(Color32::LIGHT_GRAY)).frame(false)).clicked() {
                                tab_to_close = Some(i);
                            }
                        });
                    });
                }
                if let Some(idx) = tab_to_close {
                    self.tabs.remove(idx);
                    if self.active_tab >= self.tabs.len() && self.active_tab > 0 {
                        self.active_tab = self.tabs.len() - 1;
                    }
                }
            });
        });

        // 4. PANEL KANAN (Figure)
        egui::SidePanel::right("plot_panel").resizable(true).default_width(400.0).show(ctx, |ui| {
            ui.heading("Figure");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.plots.is_empty() {
                    ui.label("Belum ada grafik.");
                } else {
                    for (i, plot) in self.plots.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.label(if plot.title.is_empty() { format!("Grafik {}", i + 1) } else { plot.title.clone() });
                            if ui.small_button("🔍").clicked() { self.popup_plot = Some(i); }
                        });
                        self.draw_plot(ui, plot, 150.0);
                        ui.add_space(10.0);
                    }
                }
            });
        });

        // 5. JENDELA POP-UP
        if let Some(idx) = self.popup_plot {
            let mut is_open = true;
            egui::Window::new("NumLite - Figure Viewer")
                .open(&mut is_open).resizable(true).default_size([800.0, 500.0]).collapsible(false)
                .show(ctx, |ui| {
                    if let Some(plot) = self.plots.get(idx) { self.draw_plot(ui, plot, ui.available_height() - 10.0); }
                });
            if !is_open { self.popup_plot = None; }
        }

        // 6. PANEL BAWAH (Console Output)
        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(true)
            .default_height(150.0)
            .frame(egui::Frame::none().fill(Color32::from_rgb(25, 25, 25)).inner_margin(margin_konten))
            .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Console Output");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_sized([100.0, 30.0], egui::Button::new("🗑 Clear")).clicked() {
                        self.output.clear();
                    }
                });
            });
            
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.output)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .interactive(false));
            });
        });

        // 7. PANEL TENGAH (Editor)
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(bg_editor).inner_margin(margin_konten))
            .show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Editor");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add_sized([90.0, 30.0], egui::Button::new("▶ Run")).clicked() {
                        self.run_code();
                    }
                    if ui.add_sized([90.0, 30.0], egui::Button::new("💾 Save")).clicked() {
                        self.save_file();
                    }
                });
            });

            ui.separator();

            if let Some(tab) = self.tabs.get_mut(self.active_tab) {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    let text_edit = egui::TextEdit::multiline(&mut tab.code)
                        .font(egui::TextStyle::Monospace)
                        .desired_width(f32::INFINITY)
                        .desired_rows(20);
                    ui.add(text_edit);
                });
            }
        });
    }
}