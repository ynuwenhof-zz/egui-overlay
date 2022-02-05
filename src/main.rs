use anyhow::bail;
use egui::epaint::Shadow;
use egui::{Color32, Frame, Window};
use egui_glium::egui_winit::winit::event::Event;
use egui_glium::EguiGlium;
use glium::glutin::dpi::{PhysicalPosition, PhysicalSize};
use glium::glutin::event::WindowEvent;
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use glium::glutin::window::WindowBuilder;
use glium::glutin::ContextBuilder;
use glium::{Display, Surface};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::env;
use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::UI::Input::KeyboardAndMouse;
use windows::Win32::UI::Input::KeyboardAndMouse::VK_RSHIFT;
use windows::Win32::UI::WindowsAndMessaging;
use windows::Win32::UI::WindowsAndMessaging::{
    GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_NOACTIVATE, WS_EX_TRANSPARENT,
};

fn create_display(event_loop: &EventLoop<()>) -> anyhow::Result<Display> {
    let window_builder = WindowBuilder::new()
        .with_decorations(false)
        .with_transparent(true)
        .with_always_on_top(true);

    let context_builder = ContextBuilder::new();

    Ok(Display::new(window_builder, context_builder, event_loop)?)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let target_handle = HWND(
        isize::from_str_radix(
            &args.get(1).expect("no window handle has been provided")[2..],
            16,
        )
        .expect("invalid window handle"),
    );

    let event_loop = EventLoop::with_user_event();
    let display = create_display(&event_loop)?;

    let mut render_gui = true;
    let overlay_handle = HWND(match display.gl_window().window().raw_window_handle() {
        RawWindowHandle::Win32(handle) => handle.hwnd as isize,
        _ => bail!("invalid window handle variant"),
    });

    unsafe {
        WindowsAndMessaging::SetWindowLongPtrA(
            overlay_handle,
            GWL_EXSTYLE,
            (WS_EX_TRANSPARENT | WS_EX_NOACTIVATE) as isize,
        );
    }

    let mut egui_glium = EguiGlium::new(&display);

    event_loop.run(move |event, _, control_flow| {
        if (unsafe { KeyboardAndMouse::GetAsyncKeyState(VK_RSHIFT as i32) } & 1) != 0 {
            let style =
                unsafe { WindowsAndMessaging::GetWindowLongPtrA(overlay_handle, GWL_EXSTYLE) };
            unsafe {
                WindowsAndMessaging::SetWindowLongPtrA(
                    overlay_handle,
                    GWL_EXSTYLE,
                    style ^ WS_EX_LAYERED as isize,
                );
            }

            render_gui = !render_gui;
        }

        let mut redraw = || {
            let (needs_repaint, shapes) = egui_glium.run(&display, |ctx| {
                Window::new("Example App")
                    .frame(Frame::window(&ctx.style()).shadow(Shadow::default()))
                    .show(ctx, |ui| {
                        ui.horizontal_wrapped(|ui| {
                            ui.label("Overlay handle: ");

                            ui.colored_label(
                                Color32::from_rgb(110, 255, 110),
                                format!("{:#X}", overlay_handle.0),
                            );
                        });

                        ui.horizontal_wrapped(|ui| {
                            ui.label("Target handle: ");

                            ui.colored_label(
                                Color32::from_rgb(110, 255, 110),
                                format!("{:#X}", target_handle.0),
                            );
                        });

                        ui.separator();
                        ui.vertical_centered(|ui| {
                            ui.hyperlink("https://github.com/ynuwenhof/egui-overlay")
                        });
                    });
            });

            {
                let mut rect = RECT::default();
                unsafe {
                    WindowsAndMessaging::GetWindowRect(target_handle, &mut rect);
                }

                let gl_window = display.gl_window();
                let window = gl_window.window();

                window.set_outer_position(PhysicalPosition::new(rect.left, rect.top));
                window.set_inner_size(PhysicalSize::new(
                    rect.right - rect.left,
                    rect.bottom - rect.top,
                ));
            }

            *control_flow = if needs_repaint {
                display.gl_window().window().request_redraw();
                ControlFlow::Poll
            } else {
                ControlFlow::Wait
            };

            {
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 0.0, 0.0);

                if render_gui {
                    egui_glium.paint(&display, &mut target, shapes);
                }

                target.finish().unwrap();
            }
        };

        match event {
            Event::WindowEvent { event, .. } => {
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    *control_flow = ControlFlow::Exit;
                }

                if render_gui {
                    egui_glium.on_event(&event);
                }

                display.gl_window().window().request_redraw();
            }
            Event::RedrawRequested(_) => redraw(),
            Event::RedrawEventsCleared => redraw(),
            _ => {}
        }
    });
}
