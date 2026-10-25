use eframe::egui;
use egui::Color32;

pub struct PlotData {
    pub title: String,
    pub x: Vec<f64>,
    pub y: Vec<f64>,
}

pub fn show_plots(plots: Vec<PlotData>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 800.0])
            .with_title("NumLite - Figure"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "NumLite Figure",
        options,
        Box::new(|_cc| Box::new(PlotApp { plots })),
    );
}

struct PlotApp {
    plots: Vec<PlotData>,
}

impl eframe::App for PlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let count = self.plots.len();
            if count == 0 { return; }
            let height_per_plot = ui.available_height() / count as f32;

            for plot in &self.plots {
                if !plot.title.is_empty() {
                    ui.heading(&plot.title);
                } else {
                    ui.heading("NumLite Signal Plot");
                }

                let margin_left = 40.0;
                let margin_bottom = 20.0;
                let (response, painter) = ui.allocate_painter(
                    egui::vec2(ui.available_width(), height_per_plot - 40.0), 
                    egui::Sense::hover()
                );
                
                let full_rect = response.rect;
                let rect = egui::Rect::from_min_max(
                    egui::pos2(full_rect.left() + margin_left, full_rect.top()),
                    egui::pos2(full_rect.right(), full_rect.bottom() - margin_bottom)
                );

                if plot.x.is_empty() || plot.y.is_empty() { continue; }

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

                let font = egui::FontId::proportional(10.0);
                let text_color = Color32::LIGHT_GRAY;
                let grid_pen = egui::Stroke::new(1.0_f32, Color32::from_rgb(50, 50, 50));

                // Grid dan Angka Sumbu Y
                for i in 0..=4 {
                    let ratio = i as f64 / 4.0;
                    let y_val = min_y + ratio * range_y;
                    let py = rect.bottom() - (ratio * rect.height() as f64) as f32;
                    
                    // PERBAIKAN: Bungkus dengan egui::pos2()
                    painter.line_segment([egui::pos2(rect.left(), py), egui::pos2(rect.right(), py)], grid_pen);
                    
                    let text = format!("{:.1}", y_val);
                    painter.text(
                        egui::pos2(rect.left() - 5.0, py),
                        egui::Align2::RIGHT_CENTER,
                        text,
                        font.clone(),
                        text_color
                    );
                }

                // Grid dan Angka Sumbu X
                for i in 0..=4 {
                    let ratio = i as f64 / 4.0;
                    let x_val = min_x + ratio * range_x;
                    let px = rect.left() + (ratio * rect.width() as f64) as f32;
                    
                    // PERBAIKAN: Bungkus dengan egui::pos2()
                    painter.line_segment([egui::pos2(px, rect.top()), egui::pos2(px, rect.bottom())], grid_pen);
                    
                    let text = format!("{:.1}", x_val);
                    painter.text(
                        egui::pos2(px, rect.bottom() + 5.0),
                        egui::Align2::CENTER_TOP,
                        text,
                        font.clone(),
                        text_color
                    );
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
        });
    }
}
