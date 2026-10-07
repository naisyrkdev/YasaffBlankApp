#![cfg_attr(all(windows, not(debug_assertions)), windows_subsystem = "windows")]

mod host;
mod pad;

use egui::{Align2, Color32, FontId, Id, Rect, Sense, Ui, pos2, vec2};
use host::Host;
use pad::{Command, Pad};
use std::time::Duration;

const PAD_B: Color32 = Color32::from_rgb(0xd9, 0x3a, 0x3a);

fn main() -> eframe::Result {
    let host = Host::from_env();
    let options = eframe::NativeOptions {
        viewport: host.viewport(
            egui::ViewportBuilder::default()
                .with_title("Blank App")
                .with_app_id("blank-app")
                .with_inner_size([1280.0, 720.0])
                .with_min_inner_size([640.0, 360.0]),
        ),
        ..Default::default()
    };
    eframe::run_native("Blank App", options, Box::new(|cc| Ok(Box::new(App::new(cc, host)))))
}

struct App {
    pad: Pad,
    host: Host,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>, mut host: Host) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        host.listen(&cc.egui_ctx);
        App { pad: Pad::new(), host }
    }

    fn handle(&mut self, ctx: &egui::Context, cmd: Command) {
        match cmd {
            Command::Quit => ctx.send_viewport_cmd(egui::ViewportCommand::Close),
        }
    }
}

impl eframe::App for App {
    /// Runs before every frame, and on its own while the window is minimized, so
    /// YASAFF can bring the app back.
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.host.update(ctx);
        if ctx.input(|i| i.viewport().minimized) == Some(true) {
            ctx.request_repaint_after(Duration::from_millis(250));
        }
    }

    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        for cmd in self.pad.poll(&ctx) {
            self.handle(&ctx, cmd);
        }

        let screen = ui.max_rect();
        let s = (screen.height() / 1080.0).max(0.4);
        let painter = ui.painter();
        painter.rect_filled(screen, 0, Color32::from_rgb(22, 14, 36));
        painter.text(pos2(screen.left() + 60.0 * s, screen.top() + 55.0 * s), Align2::LEFT_CENTER, "Blank App", FontId::proportional(30.0 * s), Color32::WHITE);
        let note = if self.host.inside_yasaff { "Running inside YASAFF" } else { "Running on its own" };
        painter.text(screen.center(), Align2::CENTER_CENTER, note, FontId::proportional(24.0 * s), Color32::from_gray(150));

        if quit_button(ui, screen, s) {
            self.handle(&ctx, Command::Quit);
        }

        // Gamepad events don't wake egui up, so keep polling for them.
        let every = if ctx.input(|i| i.focused) { 50 } else { 250 };
        ctx.request_repaint_after(Duration::from_millis(every));
    }
}

/// The "(B) Quit" hint in the bottom-right corner; it's also clickable. Returns
/// whether it was clicked.
fn quit_button(ui: &Ui, screen: Rect, s: f32) -> bool {
    let painter = ui.painter();
    let font = FontId::proportional(17.0 * s);
    let label = painter.layout_no_wrap("Quit".to_owned(), font, Color32::from_gray(210));
    let pill_w = 28.0 * s;
    let y = screen.bottom() - 45.0 * s;
    let right = screen.right() - 60.0 * s;
    let rect = Rect::from_min_max(pos2(right - label.size().x - 10.0 * s - pill_w, y - 14.0 * s), pos2(right, y + 14.0 * s));
    let response = ui.interact(rect, Id::new("quit"), Sense::click()).on_hover_cursor(egui::CursorIcon::PointingHand);
    let hot = response.hovered();

    let pill = Rect::from_min_size(rect.min, vec2(pill_w, 28.0 * s));
    painter.rect_filled(pill, (14.0 * s) as u8, if hot { PAD_B.lerp_to_gamma(Color32::WHITE, 0.3) } else { PAD_B });
    painter.text(pill.center(), Align2::CENTER_CENTER, "B", FontId::proportional(14.0 * s), Color32::WHITE);
    let color = if hot { Color32::WHITE } else { Color32::from_gray(210) };
    painter.text(pos2(pill.right() + 10.0 * s, y), Align2::LEFT_CENTER, "Quit", FontId::proportional(17.0 * s), color);
    response.clicked()
}
