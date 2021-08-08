use crate::colorcodes::*;

/// Sets the foreground color of the terminal.
/// # Example
/// ```
/// use color_please::*;
/// set_fg(Color::Green);
/// ```
pub fn set_fg(color: Color) {
  match color {
    Color::Black => print!("\x1b[30m"),
    Color::Red => print!("\x1b[31m"),
    Color::Green => print!("\x1b[32m"),
    Color::Yellow => print!("\x1b[33m"),
    Color::Blue => print!("\x1b[34m"),
    Color::Magenta => print!("\x1b[35m"),
    Color::Cyan => print!("\x1b[36m"),
    Color::White => print!("\x1b[37m"),
    Color::BrightBlack => print!("\x1b[90m"),
    Color::BrightRed => print!("\x1b[91m"),
    Color::BrightGreen => print!("\x1b[92m"),
    Color::BrightYellow=> print!("\x1b[93m"),
    Color::BrightBlue => print!("\x1b[94m"),
    Color::BrightMagenta => print!("\x1b[95m"),
    Color::BrightCyan => print!("\x1b[96m"),
    Color::BrightWhite => print!("\x1b[97m"),
    Color::Color256(code) => print!("\x1b[38;5;{}m", code),
    Color::ColorRGB(r,g,b) => print!("\x1b[38;2;{};{};{}m", r, g, b)
  };
}

/// Sets the background color of the terminal.
/// # Example
/// ```
/// use color_please::*;
/// set_bg(Color::Red);
/// ```
pub fn set_bg(color: Color) {
  match color {
    Color::Black => print!("\x1b[40m"),
    Color::Red => print!("\x1b[41m"),
    Color::Green => print!("\x1b[42m"),
    Color::Yellow => print!("\x1b[43m"),
    Color::Blue => print!("\x1b[44m"),
    Color::Magenta => print!("\x1b[45m"),
    Color::Cyan => print!("\x1b[46m"),
    Color::White => print!("\x1b[47m"),
    Color::BrightBlack => print!("\x1b[100m"),
    Color::BrightRed => print!("\x1b[101m"),
    Color::BrightGreen => print!("\x1b[102m"),
    Color::BrightYellow=> print!("\x1b[103m"),
    Color::BrightBlue => print!("\x1b[104m"),
    Color::BrightMagenta => print!("\x1b[105m"),
    Color::BrightCyan => print!("\x1b[106m"),
    Color::BrightWhite => print!("\x1b[107m"),
    Color::Color256(code) => print!("\x1b[48;5;{}m", code),
    Color::ColorRGB(r,g,b) => print!("\x1b[48;2;{};{};{}m", r, g, b)
  };
}
