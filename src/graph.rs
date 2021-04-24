use crate::error::CheckerError;
use plotters::prelude::*;
use std::path::Path;

pub struct Graph<'a> {
    output_path: &'a Path,
    caption: String,
    x_label: String,
    y_label: String,
}

impl<'a> Graph<'a> {
    pub fn new(output_path: &'a Path, caption: String, x_label: String, y_label: String) -> Self {
        Self {
            output_path,
            caption,
            x_label,
            y_label,
        }
    }

    pub fn draw(&self, sorted_vals: Vec<u64>) -> Result<(), CheckerError> {
        let root_area = BitMapBackend::new(self.output_path, (600, 400)).into_drawing_area();
        let max: &u64 = sorted_vals.iter().max().expect("It should never be empty");
        root_area.fill(&WHITE).map_err(|e| {
            CheckerError::GraphPlotterDrawingError(format!("Root area creation failed: {}", e))
        })?;

        let mut ctx = ChartBuilder::on(&root_area)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .caption(&self.caption, ("sans-serif", 40))
            // Added offset of 10, so that Y-axis looks nice
            .build_cartesian_2d((1..9).into_segmented(), 0..max + 10)
            .map_err(|e| {
                CheckerError::GraphPlotterDrawingError(format!("Chart creation failed: {}", e))
            })?;

        ctx.configure_mesh()
            .y_desc(&self.y_label)
            .x_desc(&self.x_label)
            .draw()
            .map_err(|e| {
                CheckerError::GraphPlotterDrawingError(format!("Chart configuration failed: {}", e))
            })?;

        ctx.draw_series((1..).zip(sorted_vals.iter()).map(|(x, y)| {
            let x0 = SegmentValue::Exact(x);
            let x1 = SegmentValue::Exact(x + 1);
            let mut bar = Rectangle::new([(x0, 0), (x1, *y)], RED.filled());
            bar.set_margin(0, 0, 5, 5);
            bar
        }))
        .map_err(|e| {
            CheckerError::GraphPlotterDrawingError(format!("Data drawing failed: {}", e))
        })?;
        Ok(())
    }
}
