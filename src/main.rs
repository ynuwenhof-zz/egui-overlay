use crate::epi::egui::CtxRef;
use crate::epi::Frame;
use clap::Parser;
use eframe::{epi, NativeOptions};
use windows::Win32::Foundation::HWND;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Opts {
    /// Set the target window handle
    #[clap(short, long)]
    target: String,
}

fn main() {
    let opts = Opts::parse();

    let target =
        isize::from_str_radix(&opts.target.trim()[2..], 16).expect("failed to parse target handle");

    let app = App::new(target);

    let options = NativeOptions {
        decorated: false,
        transparent: true,
        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}

#[derive(Default)]
struct App {
    overlay: Option<HWND>,
    target: HWND,
}

impl App {
    pub fn new(target: isize) -> Self {
        Self {
            target: HWND(target),
            ..Default::default()
        }
    }
}

impl epi::App for App {
    fn update(&mut self, _ctx: &CtxRef, _frame: &Frame) {}

    fn name(&self) -> &str {
        "Example App"
    }
}
