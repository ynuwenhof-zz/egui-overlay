use crate::epi::egui::{CtxRef, Rgba};
use crate::epi::{Frame, Storage};
use clap::Parser;
use eframe::epi;
use eframe::NativeOptions;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::{GWL_EXSTYLE, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT};

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

    fn setup(&mut self, _ctx: &CtxRef, _frame: &Frame, _storage: Option<&dyn Storage>) {
        let overlay = unsafe { KeyboardAndMouse::GetActiveWindow() };
        self.overlay = Some(overlay);

        unsafe {
            WindowsAndMessaging::SetWindowLongPtrA(
                overlay,
                GWL_EXSTYLE,
                (WS_EX_TRANSPARENT | WS_EX_NOACTIVATE) as isize,
            );
        }
    }

    fn name(&self) -> &str {
        "Example App"
    }

    fn clear_color(&self) -> Rgba {
        Rgba::TRANSPARENT
    }
}
