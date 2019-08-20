use chrono::Local;
use std::io::{stdout, Stdout, Write};
use termion::color::{Fg, Rgb};
use termion::style::{Bold, Reset, Underline};

pub const GRAY: Fg<Rgb> = Fg(Rgb(153, 153, 153));
pub const LIGHT_RED: Fg<Rgb> = Fg(Rgb(100, 53, 48));
pub const PURPLE: Fg<Rgb> = Fg(Rgb(68, 54, 74));
pub const GREEN: Fg<Rgb> = Fg(Rgb(62, 75, 14));
pub const RED: Fg<Rgb> = Fg(Rgb(100, 42, 39));
pub const _YELLOW: Fg<Rgb> = Fg(Rgb(255, 255, 102));

pub struct Logger {
    writer: Stdout,
}

#[allow(dead_code)]
impl Logger {
    pub fn new() -> Logger {
        Logger { writer: stdout() }
    }

    pub fn started(&mut self) {
        let now = Local::now();
        let (year, month, day, hour, minutes, seconds) = (
            now.format("%Y").to_string(),
            now.format("%m").to_string(),
            now.format("%d").to_string(),
            now.format("%H").to_string(),
            now.format("%M").to_string(),
            now.format("%S").to_string(),
        );

        let message = format!(
            "{}{}[{}{}{}-{}{}{}-{}{} {}{}{}:{}{}{}:{}{}{}] {}START{} : {}Started recording\n",
            Bold,
            GRAY,
            LIGHT_RED,
            year,
            GRAY,
            LIGHT_RED,
            month,
            GRAY,
            LIGHT_RED,
            day,
            PURPLE,
            hour,
            GRAY,
            PURPLE,
            minutes,
            GRAY,
            PURPLE,
            seconds,
            GRAY,
            GREEN,
            GRAY,
            Reset
        );

        self.writer
            .write(message.as_bytes())
            .expect("Failed to write to stdout");
    }

    pub fn stopped(&mut self) {
        let now = Local::now();
        let (year, month, day, hour, minutes, seconds) = (
            now.format("%Y").to_string(),
            now.format("%m").to_string(),
            now.format("%d").to_string(),
            now.format("%H").to_string(),
            now.format("%M").to_string(),
            now.format("%S").to_string(),
        );

        let message = format!(
            "{}{}[{}{}{}-{}{}{}-{}{} {}{}{}:{}{}{}:{}{}{}] {}STOP{}  : {}Encrypting and saving\n",
            Bold,
            GRAY,
            LIGHT_RED,
            year,
            GRAY,
            LIGHT_RED,
            month,
            GRAY,
            LIGHT_RED,
            day,
            PURPLE,
            hour,
            GRAY,
            PURPLE,
            minutes,
            GRAY,
            PURPLE,
            seconds,
            GRAY,
            RED,
            GRAY,
            Reset
        );

        self.writer
            .write(message.as_bytes())
            .expect("Failed to write to stdout");
    }

    pub fn info<S: Into<String>>(&mut self, message: S) {
        let now = Local::now();
        let (year, month, day, hour, minutes, seconds) = (
            now.format("%Y").to_string(),
            now.format("%m").to_string(),
            now.format("%d").to_string(),
            now.format("%H").to_string(),
            now.format("%M").to_string(),
            now.format("%S").to_string(),
        );

        let message = format!(
            "{}{}[{}{}{}-{}{}{}-{}{} {}{}{}:{}{}{}:{}{}{}] {}{}INFO{}{}  : {}{}\n",
            Bold,
            GRAY,
            LIGHT_RED,
            year,
            GRAY,
            LIGHT_RED,
            month,
            GRAY,
            LIGHT_RED,
            day,
            PURPLE,
            hour,
            GRAY,
            PURPLE,
            minutes,
            GRAY,
            PURPLE,
            seconds,
            GRAY,
            GREEN,
            Underline,
            Reset,
            GRAY,
            Reset,
            message.into()
        );

        self.writer
            .write(message.as_bytes())
            .expect("Failed to write to stdout");
    }

    pub fn error<S: Into<String>>(&mut self, message: S) {
        let now = Local::now();
        let (year, month, day, hour, minutes, seconds) = (
            now.format("%Y").to_string(),
            now.format("%m").to_string(),
            now.format("%d").to_string(),
            now.format("%H").to_string(),
            now.format("%M").to_string(),
            now.format("%S").to_string(),
        );

        let message = format!(
            "{}{}[{}{}{}-{}{}{}-{}{} {}{}{}:{}{}{}:{}{}{}] {}{}ERROR{}{} : {}{}\n",
            Bold,
            GRAY,
            LIGHT_RED,
            year,
            GRAY,
            LIGHT_RED,
            month,
            GRAY,
            LIGHT_RED,
            day,
            PURPLE,
            hour,
            GRAY,
            PURPLE,
            minutes,
            GRAY,
            PURPLE,
            seconds,
            GRAY,
            RED,
            Underline,
            Reset,
            GRAY,
            Reset,
            message.into()
        );

        self.writer
            .write(message.as_bytes())
            .expect("Failed to write to stdout");
    }
}
