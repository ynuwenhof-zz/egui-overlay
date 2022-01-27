use clap::Parser;
use eframe::egui::containers;
use eframe::egui::{Color32, CtxRef, Rgba, Window};
use eframe::epi;
use eframe::epi::{Frame, Storage};
use eframe::NativeOptions;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, HWND_TOPMOST, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT,
};

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
    fn update(&mut self, ctx: &CtxRef, _frame: &Frame) {
        let overlay = match self.overlay {
            None => return,
            Some(overlay) => overlay,
        };

        let mut rect = RECT::default();
        unsafe {
            WindowsAndMessaging::GetWindowRect(self.target, &mut rect);
        }

        unsafe {
            WindowsAndMessaging::SetWindowPos(
                overlay,
                HWND_TOPMOST,
                rect.left,
                rect.top,
                rect.right - rect.left,
                rect.bottom - rect.top,
                0,
            );
        }

        Window::new(self.name())
            .frame(containers::Frame::window(&ctx.style()).shadow(Default::default()))
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label("Overlay handle: ");

                    ui.colored_label(
                        Color32::from_rgb(110, 255, 110),
                        format!("{:#X}", overlay.0),
                    );
                });

                ui.horizontal_wrapped(|ui| {
                    ui.label("Target handle: ");

                    ui.colored_label(
                        Color32::from_rgb(110, 255, 110),
                        format!("{:#X}", self.target.0),
                    );
                });

                ui.separator();
                ui.vertical_centered(|ui| {
                    ui.hyperlink("https://github.com/ynuwenhof/egui-overlay")
                });
            });
    }

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
