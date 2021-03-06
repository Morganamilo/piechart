use piechart::{Chart, Color, Data};

fn main() {
    #[rustfmt::skip]
    let data = vec![
        Data { label: "dd1".into(), value: 4.0, color: Some(Color::Red), fill: '•' },
        Data { label: "dd2".into(), value: 2.0, color: Some(Color::Green), fill: '•' },
        Data { label: "dd3".into(), value: 2.6, color: Some(Color::Blue), fill: '•' },
    ];

    Chart::new()
        .radius(9)
        .aspect_ratio(2)
        .legend(true)
        .draw(&data);
}
