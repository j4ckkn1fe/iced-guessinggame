#![windows_subsystem = "windows"]

mod main_window;
use iced::{Settings, Application, window};


pub fn main() -> iced::Result {
    main_window::MainWindow::run(Settings {
        window: window::Settings {
            size: (285, 375),
            position: (iced::window::Position::Centered),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
