/// The enum includes the default 4-bit colors, bright colors, 256 colors and RGB colors.
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
  Color256(u8),
  ColorRGB(u8,u8,u8)
}
