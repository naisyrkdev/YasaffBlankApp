use egui::{Context, Key};
use gilrs::{Button, EventType, Gilrs};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Command {
    Quit,
}

pub struct Pad {
    gilrs: Option<Gilrs>,
}

impl Pad {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().map_err(|e| eprintln!("no controller support: {e}")).ok();
        Pad { gilrs }
    }

    pub fn poll(&mut self, ctx: &Context) -> Vec<Command> {
        let focused = ctx.input(|i| i.focused);
        let mut out = keyboard(ctx);
        let Some(gilrs) = &mut self.gilrs else { return out };
        // Drain every event, but ignore them while another window (YASAFF) has focus.
        while let Some(event) = gilrs.next_event() {
            if let EventType::ButtonPressed(button, _) = event.event
                && focused
                && let Some(cmd) = button_command(button)
            {
                out.push(cmd);
            }
        }
        out
    }
}

fn button_command(button: Button) -> Option<Command> {
    Some(match button {
        Button::East => Command::Quit,
        _ => return None,
    })
}

fn keyboard(ctx: &Context) -> Vec<Command> {
    const KEYS: [(Key, Command); 1] = [(Key::Escape, Command::Quit)];
    ctx.input(|i| KEYS.iter().filter(|(key, _)| i.key_pressed(*key)).map(|(_, cmd)| *cmd).collect())
}
