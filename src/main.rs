use std::io::{self, IsTerminal, Read};

fn main() {
    let stdin_is_terminal = io::stdin().is_terminal();

    // Bootstrap: ensure themes dir exists with at least Default.toml
    if let Err(e) = ccxline::config::manager::bootstrap() {
        eprintln!("ccxline: bootstrap error: {}", e);
        std::process::exit(1);
    }

    if stdin_is_terminal {
        // No stdin data → launch TUI editor
        if let Err(e) = ccxline::tui::run() {
            eprintln!("ccxline: TUI error: {}", e);
            std::process::exit(1);
        }
    } else {
        // Stdin has data → statusline rendering mode
        run_statusline();
    }
}

fn run_statusline() {
    // Read JSON from stdin
    let mut input_str = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input_str) {
        eprintln!("ccxline: stdin read error: {}", e);
        std::process::exit(1);
    }

    let input: ccxline::core::input::InputData = match serde_json::from_str(&input_str) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("ccxline: JSON parse error: {}", e);
            std::process::exit(1);
        }
    };

    // Load active theme
    let (_name, _path, theme) = match ccxline::config::manager::load_active_theme() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("ccxline: theme load error: {}", e);
            std::process::exit(1);
        }
    };

    // Collect component data and render
    let components = ccxline::core::statusline::collect_all_components(&theme, &input);
    let generator = ccxline::core::statusline::StatusLineGenerator::new(&theme);
    let statusline = generator.generate(components);

    print!("{}\x1b[0m", statusline);
}
