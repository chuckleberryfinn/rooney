use separator::Separatable;

pub fn format_currency(value: f32) -> String {
    if value < 1.0 {
        return format!("{:.8}", value);
    }

    let v = (value * 100.0).round() / 100.0;

    let s = v.separated_string();
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 1 {
        return format!("{}.00", s);
    }

    if parts[1].len() == 2 {
        return s;
    }

    format!("{}0", s)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! format_currency_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, format_currency(input));
            }
        )*
        }
    }

    macro_rules! format_change_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, format_change(input));
            }
        )*
        }
    }

    format_currency_tests! {
        format_currency_0: (1.0, "1.00"),
        format_currency_1: (0.012345, "0.01234500"),
        format_currency_2: (1.0, "1.00"),
    }

    format_change_tests! {
        format_change_0: (1.0, "\u{3}03Up: 1.00%"),
        format_change_1: (-50.0512, "\u{3}05Down: 50.05%"),
    }
}