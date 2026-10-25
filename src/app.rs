use eframe::egui;
use crate::eval::{exec_stmt, eval, Env};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::value::Value;
use crate::gui::PlotData;
use egui::Color32;

pub struct NumLiteApp {
    code: String,
    output: String,
    env: Env,
    plots: Vec<PlotData>,
    // State untuk jendela melayang (pop-up)
    popup_plot: Option<usize>,
}

impl Default for NumLiteApp {
    fn default() -> Self {
        Self {
            code: String::from("% Tulis kode MATLAB Anda di sini\n% Contoh:\nt = 0:0.1:5\nx = sin(t)\nplot(t, x)\n"),
            output: String::from("Selamat datang di NumLite GUI!\nKlik 'Run' untuk mengeksekusi kode.\n"),
            env: Env::new(),
            plots: Vec::new(),
            popup_plot: None,
        }
    }
}

impl NumLiteApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn run_code(&mut self) {
        self.output.clear();
        self.plots.clear();
        self.popup_plot = None;
        let code = self.code.clone();
        
        let mut lexer = Lexer::new(&code);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                self.output = format!("Lexer Error: {}\n", e);
                return;
            }
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
                    if e != "Eof" {
                        captured_output.push_str(&format!("Parser Error: {}\n", e));
                    }
                    break;
                }
            }
        }
        
        self.output = captured_output;
        if self.output.is_empty() {
            self.output = String::from("Eksekusi berhasil. (Tidak ada output teks)\n");
        } else {
            self.output.push_str("Eksekusi selesai.\n");
        }
    }

    // Fungsi bantuan untuk menggambar grafik (agar tidak duplikat kode)
    fn draw_plot(&self, ui: &mut egui::Ui, plot: &PlotData, height: f32) {
        let margin_left = 30.0;
        let margin_bottom = 15.0;
        let (response, painter) = ui.allocate_painter(egui::vec2(ui.available_width(), height), egui::Sense::hover());
        let full_rect = response.rect;
        let rect = egui::Rect::from_min_max(
            egui::pos2(full_rect.left() + margin_left, full_rect.top()),
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
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("NumLite");
                ui.separator();
                if ui.button("▶ Run Script").clicked() {
                    self.run_code();
                }
                if ui.button("🗑 Clear Console").clicked() {
                    self.output.clear();
                }
            });
        });

        egui::SidePanel::right("plot_panel").resizable(true).default_width(400.0).show(ctx, |ui| {
            ui.heading("Figure");
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.plots.is_empty() {
                    ui.label("Belum ada grafik. Jalankan 'plot()'.");
                } else {
                    for (i, plot) in self.plots.iter().enumerate() {
                        ui.horizontal(|ui| {
                            if !plot.title.is_empty() {
                                ui.label(&plot.title);
                            } else {
                                ui.label(format!("Grafik {}", i + 1));
                            }
                            // TOMBOL POP-OUT
                            if ui.small_button("🔍").on_hover_text("Buka di jendela baru").clicked() {
                                self.popup_plot = Some(i);
                            }
                        });
                        
                        self.draw_plot(ui, plot, 150.0);
                        ui.add_space(10.0);
                    }
                }
            });
        });

        // JENDELA MELAYANG (POP-UP)
        if let Some(idx) = self.popup_plot {
            let mut is_open = true;
            egui::Window::new("NumLite - Figure Viewer")
                .open(&mut is_open)
                .resizable(true)
                .default_size([800.0, 500.0])
                .collapsible(false)
                .show(ctx, |ui| {
                    if let Some(plot) = self.plots.get(idx) {
                        if !plot.title.is_empty() {
                            ui.heading(&plot.title);
                        }
                        self.draw_plot(ui, plot, ui.available_height() - 10.0);
                    }
                });
            
            if !is_open {
                self.popup_plot = None;
            }
        }

        egui::TopBottomPanel::bottom("bottom_panel").resizable(true).default_height(150.0).show(ctx, |ui| {
            ui.heading("Console Output");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.output)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .interactive(false));
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Editor");
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.code)
                    .font(egui::TextStyle::Monospace)
                    .desired_width(f32::INFINITY)
                    .desired_rows(20));
            });
        });
    }
}
