mod utils;

pub use ansi_term::Color;
use std::{
    convert::TryInto,
    io::{self, Write},
};

#[derive(Clone)]
pub struct Data {
    pub label: String,
    pub value: f32,
    pub color: Option<Color>,
    pub fill: char,
}
impl Data {
    fn format_label(&self, total: f32) -> String {
        let fill = match self.color {
            Some(c) => c.paint(self.fill.to_string()).to_string(),
            None => self.fill.to_string(),
        };
        format!("{} {} {:.2}%", fill, self.label, self.value / total * 100.0)
    }
}

pub struct Chart {
    radius: u16,
    aspect_ratio: u16,
    legend: bool,
}
impl Default for Chart {
    fn default() -> Self {
        Self {
            radius: 8,
            aspect_ratio: 2,
            legend: false,
        }
    }
}

impl Chart {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn radius(&mut self, radius: u16) -> &mut Self {
        self.radius = radius;
        self
    }
    pub fn aspect_ratio(&mut self, aspect_ratio: u16) -> &mut Self {
        assert!(aspect_ratio > 0, "aspect ratio has to be greater than zero");
        self.aspect_ratio = aspect_ratio;
        self
    }
    pub fn legend(&mut self, legend: bool) -> &mut Self {
        self.legend = legend;
        self
    }
}

impl Chart {
    pub fn draw(&self, data: &[Data]) {
        self.draw_into(io::stdout(), data)
            .expect("failed to write to stdout")
    }

    pub fn draw_into(&self, f: impl io::Write, data: &[Data]) -> io::Result<()> {
        assert!(!data.is_empty(), "chart data cannot be empty");
        let total: f32 = data.iter().map(|d| d.value).sum();
        let data_angles = utils::data_angles(total, data);

        let radius = self.radius as i32;
        let aspect_ratio = self.aspect_ratio as i32;

        let center_x = (radius as f32 * (aspect_ratio as f32).sqrt()).round() as i32;

        let circle = (-radius..=radius).map(|y| {
            let width = utils::calculate_width(radius, y, aspect_ratio);

            let mut output = " ".repeat((center_x - width).try_into().unwrap());

            (-width..=width).for_each(|x| {
                let (x, y) = (x as f32, y as f32);
                let angle = x.atan2(y).to_degrees();

                let idx = data_angles
                    .iter()
                    .position(|a| 360.0 / 2.0 - angle <= *a)
                    .unwrap();
                let item = &data[idx];

                match item.color {
                    None => output.push(item.fill),
                    Some(c) => output.push_str(&c.paint(item.fill.to_string()).to_string()),
                }
            });

            if self.legend {
                let label_padding = 2;

                let padding = " ".repeat((center_x - width + label_padding).try_into().unwrap());
                output.push_str(&padding);

                let max_label_idx = (data.len() - 1) as i32;

                let mut iter = (0..=max_label_idx)
                    .map(|x| x * 2)
                    .map(|x| x - max_label_idx);

                if let Some(idx) = iter.position(|i| i == y) {
                    let item = &data[idx];
                    output.push_str(&item.format_label(total));
                }
            }

            output
        });

        let mut buf = io::BufWriter::new(f);
        for line in circle {
            writeln!(buf, "{}", line)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct NullWriter;
    impl io::Write for NullWriter {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    #[rustfmt::skip]
    fn dummy_data() -> Data {
        Data { label: "".into(), value: 1.0, color: None, fill: ' ' }
    }

    #[test]
    #[should_panic]
    fn empty_data() {
        Chart::new().draw_into(NullWriter, &[]).unwrap();
    }

    #[test]
    fn data_len() {
        let chart = Chart::new();
        for len in 1..=16 {
            println!("{}", len);
            let data = vec![dummy_data(); len];
            chart.draw_into(NullWriter, &data).unwrap();
        }
    }

    #[test]
    fn very_much_data() {
        let chart = Chart::new();
        let data = vec![dummy_data(); 2048];
        chart.draw_into(NullWriter, &data).unwrap();
    }

    #[test]
    fn config_combinations() {
        let mut chart = Chart::new();
        for radius in 0..=5 {
            chart.radius(radius);
            for aspect_ratio in 1..=5 {
                chart.aspect_ratio(aspect_ratio);
                for legend in &[true, false] {
                    chart.legend(*legend);
                    for data_len in 1..=5 {
                        let data = vec![dummy_data(); data_len];
                        chart.draw_into(NullWriter, &data).unwrap();
                    }
                }
            }
        }
    }
}
