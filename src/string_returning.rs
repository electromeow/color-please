use crate::Color;

/// Returns the given text's foreground dyed with color given.
pub fn make_colored_fg(text: &str, color: Color) -> String {
    let mut string = String::new();
    string.push_str(&match color {
        Color::Black => String::from("\x1b[30m"),
        Color::Red => String::from("\x1b[31m"),
        Color::Green => String::from("\x1b[32m"),
        Color::Yellow => String::from("\x1b[33m"),
        Color::Blue => String::from("\x1b[34m"),
        Color::Magenta => String::from("\x1b[35m"),
        Color::Cyan => String::from("\x1b[36m"),
        Color::White => String::from("\x1b[37m"),
        Color::BrightBlack => String::from("\x1b[90m"),
        Color::BrightRed => String::from("\x1b[91m"),
        Color::BrightGreen => String::from("\x1b[92m"),
        Color::BrightYellow => String::from("\x1b[93m"),
        Color::BrightBlue => String::from("\x1b[94m"),
        Color::BrightMagenta => String::from("\x1b[95m"),
        Color::BrightCyan => String::from("\x1b[96m"),
        Color::BrightWhite => String::from("\x1b[97m"),
        Color::Color256(code) => format!("\x1b[38;5;{}m", code),
        Color::ColorRGB(r, g, b) => format!("\x1b[38;2;{};{};{}m", r, g, b),
    }[..]);
    string.push_str(text);
    string.push_str("\x1b[39m");
    string
}

/// Returns the given text's background dyed with the color given.
pub fn make_colored_bg(text: &str, color: Color) -> String {
    let mut string = String::new();
    string.push_str(match color {
        Color::Black => String::from("\x1b[40m"),
        Color::Red => String::from("\x1b[41m"),
        Color::Green => String::from("\x1b[42m"),
        Color::Yellow => String::from("\x1b[43m"),
        Color::Blue => String::from("\x1b[44m"),
        Color::Magenta => String::from("\x1b[45m"),
        Color::Cyan => String::from("\x1b[46m"),
        Color::White => String::from("\x1b[47m"),
        Color::BrightBlack => String::from("\x1b[100m"),
        Color::BrightRed => String::from("\x1b[101m"),
        Color::BrightGreen => String::from("\x1b[102m"),
        Color::BrightYellow => String::from("\x1b[103m"),
        Color::BrightBlue => String::from("\x1b[104m"),
        Color::BrightMagenta => String::from("\x1b[105m"),
        Color::BrightCyan => String::from("\x1b[106m"),
        Color::BrightWhite => String::from("\x1b[107m"),
        Color::Color256(code) => format!("\x1b[48;5;{}m", code),
        Color::ColorRGB(r, g, b) => format!("\x1b[48;2;{};{};{}m", r, g, b),
    }.as_str());
    string.push_str(text);
    string.push_str("\x1b[49m");
    string
}

/// Makes the text given bold.
pub fn make_bold(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[1m");
    string.push_str(text);
    string.push_str("\x1b[22m");
    string
}

/// Applies the faint effect on the text given.
pub fn make_faint(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[2m");
    string.push_str(text);
    string.push_str("\x1b[22m");
    string
}

/// Makes the text given italic.
pub fn make_italic(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[3m");
    string.push_str(text);
    string.push_str("\x1b[23m");
    string
}

/// Makes the text given underlined.
pub fn make_underlined(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[4m");
    string.push_str(text);
    string.push_str("\x1b[24m");
    string
}

/// Switches the background and foreground colors of the text given.
pub fn make_reversed(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[7m");
    string.push_str(text);
    string.push_str("\x1b[27m");
    string
}

/// Makes the text given invisible.
pub fn make_invisible(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[8m");
    string.push_str(text);
    string.push_str("\x1b[28m");
    string
}

/// Makes the text given striked.
pub fn make_striked(text: &str) -> String {
    let mut string = String::new();
    string.push_str("\x1b[9m");
    string.push_str(text);
    string.push_str("\x1b[29m");
    string
}
