enum Prefix {
    // MAGENTA,
    // BLUE,
    GREEN,
    YELLOW,
    RED,
    // CYAN,
    // BLACK,
    // BOLD,
    // UNDERLINE,
    ENDC
}

impl std::fmt::Display for Prefix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::MAGENTA => "\x1b[95m".fmt(f),
            // Self::BLUE => "\x1b[94m".fmt(f),
            Self::GREEN => "\x1b[92m".fmt(f),
            Self::YELLOW => "\x1b[93m".fmt(f),
            Self::RED => "\x1b[91m".fmt(f),
            // Self::CYAN => "\x1b[96m".fmt(f),
            // Self::BLACK => "\x1b[30m".fmt(f),
            // Self::BOLD => "\x1b[1m".fmt(f),
            // Self::UNDERLINE => "\x1b[4m".fmt(f),
            Self::ENDC => "\x1b[0m".fmt(f),
        } 
    }
}

#[allow(unused_variables)]
fn print_release(msg: &str, prefix: &Prefix) {
    #[cfg(not(debug_assertions))]
    println!("{}{}{}", prefix, msg, Prefix::ENDC);
}

fn print(msg: &str, prefix: &Prefix) {
    println!("{}{}{}", prefix, msg, Prefix::ENDC);
}

pub fn info(msg: &str) { print(msg, &Prefix::GREEN); }
pub fn warn(msg: &str) { print(msg, &Prefix::YELLOW); }
pub fn error(msg: &str) { print(msg, &Prefix::RED); }
pub fn info_release(msg: &str) { print_release(msg, &Prefix::GREEN); }
pub fn warn_release(msg: &str) { print_release(msg, &Prefix::YELLOW); }
pub fn error_release(msg: &str) { print_release(msg, &Prefix::RED); }

pub fn job_info(msg: &str, background: bool) { if !background {print(msg, &Prefix::GREEN);} }
pub fn job_warn(msg: &str, background: bool) { if !background {print(msg, &Prefix::YELLOW);} }
pub fn job_error(msg: &str, background: bool) { if !background {print(msg, &Prefix::RED);} }