use crate::ast::PlotType;

pub struct LineData {
    pub x: Vec<f64>,
    pub y: Vec<f64>,
    pub plot_type: PlotType,
}

pub struct PlotData {
    pub title: String,
    pub xlabel: String,
    pub ylabel: String,
    pub lines: Vec<LineData>,
}