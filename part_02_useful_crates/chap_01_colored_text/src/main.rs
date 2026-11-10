/*
colored: print terminal text with colors, backgrounds, and styles.

1. Create styled text and print it
2. Standard and bright foreground colors
3. Background colors and chained styles
4. Text attributes
5. RGB foreground and background colors
6. Mix styled fragments and formatted values
7. Multiline colored text
8. Choose colors dynamically; remove styling
9. Automatic color detection and optional overrides

Setup (inside your Cargo package):
    cargo add colored@3.1

Save as src/bin/01_colored.rs and run:
    cargo run --bin 01_colored

No [[bin]] entry is needed for src/bin/.
Only colored and the standard library are used.

Python termcolor: colored(text, color, on_color, attrs)
Rust colored:    text.cyan().on_red().bold().underline()
Python cprint:  Rust println!("{}", styled_text)

Color/style methods construct a ColoredString. They do not print by themselves.
Colorize is the extension trait providing these methods; import it first.
The terminal controls the appearance of ANSI colors and supported attributes.
Docs: https://docs.rs/colored/latest/colored/
*/

use colored::{Color, ColoredString, Colorize};

fn main() {
    println!();

    demo_create_and_print();
    println!("\n============================\n");

    demo_foreground();
    println!("\n============================\n");

    demo_background_and_chaining();
    println!("\n============================\n");

    demo_attributes();
    println!("\n============================\n");

    demo_rgb();
    println!("\n============================\n");

    demo_mixed_output();
    println!("\n============================\n");

    demo_multiline();
    println!("\n============================\n");

    demo_dynamic_and_clear();
    println!("\n============================\n");

    demo_color_control();
}

// 01. Construct a styled value, then display it
// --------------------------------------------
fn demo_create_and_print() {
    println!("01. Create colored text and print it");

    let message: ColoredString = "This text is colorful!".green();
    println!("{message}");
    println!("Print the same value again: {message}");

    println!("{}", "Print colored text directly".cyan());
    // No separate cprint function is needed: format the styled value with {}.
    // Using a method such as .green() alone produces no terminal output.
    // ColoredString carries text and style; it is not an ordinary String.
    // Display renders the styling when coloring is enabled for the environment.
}

// 02. Named foreground colors
// ----------------------------
fn demo_foreground() {
    println!("02. Standard and bright foreground colors");

    println!("{} {} {} {}", "black".black(), "red".red(), "green".green(), "yellow".yellow());
    println!("{} {} {} {}", "blue".blue(), "magenta".magenta(), "cyan".cyan(), "white".white());

    println!();

    println!("{} {} {} {}", "bright black".bright_black(), "bright red".bright_red(),
        "bright green".bright_green(), "bright yellow".bright_yellow());
    println!("{} {} {} {}", "bright blue".bright_blue(), "bright magenta".bright_magenta(),
        "bright cyan".bright_cyan(), "bright white".bright_white());
    // Python termcolor light_cyan -> colored's bright_cyan().
    // purple() is an alias for magenta(); bright_purple() also exists.
    // Named ANSI colors depend on the terminal palette; black may disappear on
    // a black background. Bright black is commonly shown as gray.
}

// 03. Combine a foreground, background, and several attributes
// ------------------------------------------------------------
fn demo_background_and_chaining() {
    println!("03. Backgrounds and chaining");

    let message = "This text is colorful!"
        .bright_cyan()
        .on_bright_red()
        .bold()
        .underline();
    println!("{message}");
    // Equivalent idea to the Python example:
    // colored(text, color='light_cyan', on_color='on_light_red',
    //         attrs=['bold', 'underline'])

    println!("{}", " White on blue ".white().on_blue());
    println!("{}", " Black on bright white ".black().on_bright_white());
    // Background methods begin with on_: on_red(), on_green(), etc.
    // Padding spaces inside the styled text extend the colored background.
    // Background styling affects text cells, not the entire terminal window.

    println!("{}", "Foreground/background order can vary".on_blue().white());
    println!("{}", "Last foreground wins: red".green().blue().red());
    // Different attributes combine; setting the same color category replaces it.
}

// 04. Text attributes (support varies between terminals)
// -------------------------------------------------------
fn demo_attributes() {
    println!("04. Text attributes");

    println!("{}", "Bold".bold());
    println!("{}", "Dimmed".dimmed());
    println!("{}", "Italic".italic());
    println!("{}", "Underlined".underline());
    println!("{}", "Strikethrough".strikethrough());
    println!("{}", "Reversed foreground/background".reversed());
    println!("{}", "Bold and underlined cyan".cyan().bold().underline());

    // Also available: .blink() and .hidden(). Many terminals ignore blink;
    // hidden intentionally conceals text, so neither is needed for this demo.
    // Use reversed(), not the deprecated reverse().
    // Colors and these attributes are separate settings; bright is not just bold.
}

// 05. Truecolor: three u8 values per RGB color
// --------------------------------------------
fn demo_rgb() {
    println!("05. RGB text and backgrounds");

    let foreground = (200_u8, 150_u8, 25_u8);
    let background = (255_u8, 255_u8, 255_u8);
    let text = "Custom RGB foreground on a white background"
        .truecolor(foreground.0, foreground.1, foreground.2)
        .on_truecolor(background.0, background.1, background.2)
        .bold();
    println!("{text}");

    println!("{}", " Custom dark-green background ".black().on_truecolor(1, 95, 64).bold());
    println!("{}", "Coral foreground".truecolor(255, 127, 80));
    // Each channel ranges from 0 through 255 (u8), giving 256^3 RGB choices.
    // Rust takes three channel arguments, not a Python (r, g, b) tuple directly.
    // Actual truecolor rendering depends on terminal support/configuration.
}

// 06. Style individual pieces, or style an entire formatted message
// ----------------------------------------------------------------
fn demo_mixed_output() {
    println!("06. Mixed colors and formatted values");

    println!("{} {} {}", "Success:".green().bold(), "saved", "report.csv".cyan().underline());
    println!("{} {} {}", "INFO".blue(), "WARNING".yellow(), "ERROR".red().bold());
    println!("{} followed by ordinary text", "Only this fragment is green".green());
    // Each fragment's styling is scoped to its displayed output; you normally
    // do not need to print a manual ANSI reset after every println!.

    let completed = 42;
    let total = 50;
    println!("{}", format!("Processed {completed}/{total} items").green().bold());
    println!("Processed {}/{} items", completed.to_string().green(), total.to_string().cyan());
    // Numeric values do not have Colorize methods directly: format! or to_string
    // turns them into text. Style the complete message or selected fragments.
    // This is ordinary formatted output, not a logging framework or progress bar.
}

// 07. Multiline content: one style for the block, or styles per line
// ------------------------------------------------------------------
fn demo_multiline() {
    println!("07. Multiline colored text");

    let report = "Task started.\nInput loaded.\nTask completed.";
    println!("{}", report.green().bold());

    println!();

    println!("{}\n{}\n{}",
        "Stage 1: read input".cyan(),
        "Stage 2: processing".yellow(),
        "Stage 3: complete".green().bold(),
    );
    // Newlines remain ordinary text; the chosen style applies to the block.
    // For different styles on different lines, format separate styled fragments.
}

// 08. Runtime choices and resetting a styled value
// ------------------------------------------------
fn demo_dynamic_and_clear() {
    println!("08. Dynamic colors and clearing styles");

    let selected = Color::Cyan;
    println!("{}", "Color chosen using an enum".color(selected).on_color(Color::Black));
    println!("{}", "Color chosen using a string".color("blue").on_color("white"));
    // Prefer Color enum values for a known set of choices instead of unchecked
    // strings from users. Both color(...) and on_color(...) accept Color values.

    let ok = true;
    let status = if ok { "Ready".green() } else { "Failed".red() };
    println!("Status: {status}");

    let styled = "Reusable message".red().bold().underline();
    println!("Styled: {styled}");
    println!("Cleared: {}", styled.clone().clear());
    println!("Still styled: {styled}");
    // clear() removes foreground, background, and attributes from that value.
    // clone() keeps the original available because styling consumes ColoredString.
    // clear() is not a terminal-screen clearing command.
}

// 09. Color is optional; let terminal/environment detection handle the default
// -----------------------------------------------------------------------------
fn demo_color_control() {
    println!("09. Automatic color control");

    println!("{}", "Automatic coloring when appropriate".green());
    // Redirected output or a no-color environment may display plain text.
    // Run in a supporting terminal to see the colors. NO_COLOR can disable color.
    // A ColoredString is displayed through ANSI styling, not HTML/CSS.

    colored::control::set_override(false);
    println!("{}", "Explicitly disabled: this should be plain text".red().bold());
    colored::control::unset_override();
    println!("{}", "Environment-based behavior restored".green());
    // Overrides are process-wide: decide policy at your application's boundary.
    // This standalone demo removes its override immediately afterward.
    // Optional for testing a compatible terminal:
    // colored::control::set_override(true);  // force ANSI styling
    // ... print colored text ...
    // colored::control::unset_override();
    // Forced ANSI output can leave escape codes in logs or unsupported viewers.
}
