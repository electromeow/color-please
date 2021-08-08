/// The enum includes the default 4-bit colors, bright colors, 256 colors and RGB colors.
#[derive(Clone, Eq, PartialEq)]
pub enum Color{
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  White,
  BrightBlack,
  BrightRed,
  BrightGreen,
  BrightYellow,
  BrightBlue,
  BrightMagenta,
  BrightCyan,
  BrightWhite,
  /// The color in 256 colors.
  Color256(u8),
  /// The color of given R, G, B values.
  ColorRGB(u8,u8,u8)
}
