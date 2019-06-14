use separator::Separatable;

pub fn format_currency(value: f32) -> String {
    if value < 1.0 {
        return format!("{:.8}", value);
    }

    let v = (value * 100.0).round() / 100.0;

    v.separated_string()
}

pub fn format_change(diff: f32) -> String {
    if diff < 0.0 {
        return format!("\x0305Down: {:.2}%", diff.abs());
    }

    format!("\x0303Up: {:.2}%", diff)
}