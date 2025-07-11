use colored::Colorize;

use crate::commands::ls::Flag;

pub enum Color {
    Red,
    Orange,
    Green,
    Blue,
    SkyBlue,
}

pub fn colorize(text: &str, color: Color, bold: bool) -> String {
    let result = match color {
        Color::Red => text.red(),
        Color::Green => text.green(),
        Color::Blue => text.blue(),
        Color::Orange => text.truecolor(255, 165, 0),
        Color::SkyBlue => text.truecolor(135, 206, 235),
    };

    if bold {
        result.bold().to_string()
    } else {
        result.to_string()
    }
}

pub fn colorize_dir(file_name: &mut String, flags: &Flag) {
    *file_name = colorize(file_name, Color::Blue, true);
    if flags.f {
        file_name.push('/');
    }
}

pub fn colorize_executable(file_name: &mut String, flags: &Flag) {
    *file_name = colorize(file_name, Color::Green, true);
    if flags.f {
        file_name.push('*');
    }
}

pub fn colorize_symlink(file_name: &mut String, is_broken: bool, flags: &Flag) {
    let color = if is_broken {
        Color::Red
    } else {
        Color::SkyBlue
    };
    *file_name = colorize(file_name, color, true);

    if flags.f && !flags.l {
        file_name.push('@');
    }
}
