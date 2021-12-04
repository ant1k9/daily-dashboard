use tui::style::Color;

pub fn default_true() -> bool {
    true
}

pub fn color_from_str(color: &Option<String>) -> Color {
    match color {
        Some(value) => match value.as_str() {
            "black" => Color::Black,
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            "magenta" => Color::Magenta,
            "cyan" => Color::Cyan,
            "gray" => Color::Gray,
            "darkgray" => Color::DarkGray,
            "lightred" => Color::LightRed,
            "lightgreen" => Color::LightGreen,
            "lightyellow" => Color::LightYellow,
            "lightblue" => Color::LightBlue,
            "lightmagenta" => Color::LightMagenta,
            "lightcyan" => Color::LightCyan,
            "white" => Color::White,
            _ => Color::Yellow,
        },
        None => Color::Yellow,
    }
}
