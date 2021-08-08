# color_please
A simple library for coloring terminal and terminal text graphics control.\
It uses ANSI control sequences for setting colors and effects, tested only on MacOS.\
Some of the features in effect controls may not be supported in some systems or some terminal emulators.
## Usage
### Coloring Foreground and Background
```rust
use color_please::*;
fn main(){
    // Setting Foreground color and background color simply.
    set_fg(Color::Red);
    println!("This text is red.");
    set_bg(Color::Yellow);
    println!("And now the background is yellow.");
    // Bright Colors
    set_fg(Color::BrightGreen);
    println!("Why don't use bright green and look like a hacker in a film?");
    set_bg(Color::BrightCyan);
    println!("Walls of my terminal is my favourite color!");
    // Using 256 Colors
    set_fg(Color::Color256(123));
    println!("This text is color 123 of 256 colors.");
    // Using RGB
    set_bg(Color::ColorRGB(0, 200, 256));
    println!("The background is RGB(0,200,256).");
    // Reset the colors
    reset_fg();
    reset_bg();
}
```
### Graphics Control
```rust
use color_please::*;
fn main(){
    // Making text effects on
    bold();
    faint()
    italic();
    underline();
    // To blinking
    blink();
    // Or not to blinking
    blink_off();
    // But this time faster
    fast_blink();
    // Ok, blinking sucks
    blink_off();
    // Reverse the foreground with background and make your eyes dazzle
    reverse();
    // Let's return to our black terminal, white theme sucks
    reverse_off();
    // Make texts invisible and surprise your friend
    invisible();
    println!("You can't see this.");
    // No need to close and open a new terminal
    invisible_off();
    println!("But you can see this.");
    // Strike the last item on your to do list
    strike();
    println!("Add color_please into the dependencies.");
    // Close striking
    strike_off();
    // Turn off the text effects
    bold_and_faint_off();
    underline_off();
    italic_off();
    // Or turn all of the effects and colors off at the same time
    reset_all();
}
```
## LICENSE
This project is distributed under MIT license.
