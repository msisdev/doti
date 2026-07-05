use iocraft::prelude::*;
use crate::tui::ThemeState;

#[component]
pub fn ToggleButton(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme_state = hooks.use_context::<ThemeState>();
    let dark_mode = theme_state.0;
    let is_dark = dark_mode.get();

    hooks.use_terminal_events({
        let mut dark_mode = dark_mode.clone();
        move |event| {
            if let TerminalEvent::Key(KeyEvent {
                code: KeyCode::Char(' '),
                kind: KeyEventKind::Press,
                ..
            }) = event
            {
                dark_mode.set(!is_dark);
            }
        }
    });

    let (bg, fg, label) = if is_dark {
        (Color::Black, Color::White, "Switch to Light Mode")
    } else {
        (Color::White, Color::Black, "Switch to Dark Mode")
    };

    element! {
        View(
            background_color: bg,
            border_style: BorderStyle::Round,
            border_color: Color::Blue,
            padding: 1,
            margin_top: 1,
        ) {
            Text(color: fg, content: format!("[ Space ] {}", label))
        }
    }
}
