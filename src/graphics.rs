/// Resets all the graphical effects and colorings.
pub fn reset_all(){
  print!("\x1b[0m");
}

/// Sets the terminal font bold.
pub fn bold(){
  print!("\x1b[1m");
}

/// Makes the faint effect, decreases the density of colors.
pub fn faint(){
  print!("\x1b[2m");
}

/// Makes the terminal font italic.
pub fn italic(){
  print!("\x1b[3m");
}

/// Makes the text underlined.
pub fn underline(){
  print!("\x1b[4m");
}

/// Turns on the blink effect.
pub fn blink(){
  print!("\x1b[5m");
}

/// Turns on the rapid blink effect.
pub fn fast_blink(){
  print!("\x1b[6m");
}

/// Switches the background color and the foreground color.
pub fn reverse(){
  print!("\x1b[7m");
}

/// Turns the blink/rapid blink effect off.
pub fn blink_off(){
  print!("\x1b[25m");
}

/// Turns the reverse effect off.
pub fn reverse_off(){
  print!("\x1b[27m");
}

/// Makes the text invisible.
pub fn invisible(){
  print!("\x1b[8m");
}

/// Makes the text appear if invisible.
pub fn invisible_off(){
  print!("\x1b[28m");
}

/// Resets the foreground color to default.
pub fn reset_fg(){
  print!("\x1b[39m");
}

/// Resets the background color to default.
pub fn reset_bg(){
  print!("\x1b[49m");
}

/// Turns the bold and faint effects off.
pub fn bold_and_faint_off(){
  print!("\x1b[22m");
}

/// Turns off the underlining.
pub fn underline_off(){
  print!("\x1b[24m");
}

/// Turns off italic font.
pub fn italic_off(){
  print!("\x1b[23m");
}

/// Strikes the text.
pub fn strike(){
  print!("\x1b[9m");
}

/// Turns off the strike effect.
pub fn strike_off(){
  print!("\x1b[29m");
}