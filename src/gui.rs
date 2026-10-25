use eframe::egui;
use egui::Color32;

pub fn show_plot(x_data: Vec<f64>, y_data: Vec<f64>) {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 400.0])
            .with_title("NumLite - Plot"),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "NumLite Plot",
        options,
        Box::new(|_cc| Box::new(PlotApp { x: x_data, y: y_data })),
    );
}

struct PlotApp {
    x: Vec<f64>,
    y: Vec<f64>,
}

impl eframe::App for PlotApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("NumLite Signal Plot");
            
            let (response, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::hover());
            let rect = response.rect;

            if self.x.is_empty() || self.y.is_empty() { return; }

            let min_x = self.x.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_x = self.x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let min_y = self.y.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_y = self.y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            
            let range_x = if (max_x - min_x).abs() < 1e-9 { 1.0 } else { max_x - min_x };
            let range_y = if (max_y - min_y).abs() < 1e-9 { 1.0 } else { max_y - min_y };

            painter.rect_filled(rect, 0.0, Color32::from_rgb(20, 20, 30));
            let axis_pen = egui::Stroke::new(1.0_f32, Color32::GRAY);
            painter.line_segment([rect.left_bottom(), rect.right_bottom()], axis_pen);
            painter.line_segment([rect.left_bottom(), rect.left_top()], axis_pen);

            let mut points = Vec::new();
            for i in 0..self.x.len() {
                let px = rect.left() + ((self.x[i] - min_x) / range_x * rect.width() as f64) as f32;
                let py = rect.bottom() - ((self.y[i] - min_y) / range_y * rect.height() as f64) as f32;
                points.push(egui::pos2(px, py));
            }

            if points.len() > 1 {
                painter.add(egui::Shape::line(points, egui::Stroke::new(2.0_f32, Color32::GREEN)));
            }
        });
    }
}
