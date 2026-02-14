use std::io::{self, IsTerminal, Read};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    // Handle --write-default-themes [--force]
    if args.iter().any(|a| a == "--write-default-themes") {
        let force = args.iter().any(|a| a == "--force");
        let dir = ccxline::config::manager::themes_dir();
        if let Err(e) = std::fs::create_dir_all(&dir) {
            eprintln!("ccxline: cannot create themes dir: {}", e);
            std::process::exit(1);
        }
        match ccxline::config::manager::write_default_themes(&dir, force) {
            Ok(n) => {
                if n > 0 {
                    eprintln!("Wrote {} default theme(s) to {}", n, dir.display());
                } else {
                    eprintln!("All default themes already exist (use --force to overwrite)");
                }
            }
            Err(e) => {
                eprintln!("ccxline: error writing themes: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    // Bootstrap: ensure themes dir exists with starter themes
    if let Err(e) = ccxline::config::manager::bootstrap() {
        eprintln!("ccxline: bootstrap error: {}", e);
        std::process::exit(1);
    }

    let stdin_is_terminal = io::stdin().is_terminal();

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
