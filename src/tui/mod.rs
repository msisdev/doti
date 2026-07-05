pub mod theme_display;
pub mod toggle_button;

use iocraft::prelude::*;
use theme_display::ThemeDisplay;

#[derive(Clone, Copy)]
pub struct ThemeState(pub State<bool>);

#[component]
pub fn App(mut hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let mut should_exit = hooks.use_state(|| false);
    let dark_mode = hooks.use_state(|| true);

    hooks.use_terminal_events({
        move |event| {
            if let TerminalEvent::Key(KeyEvent {
                code: KeyCode::Char('q'),
                kind: KeyEventKind::Press,
                ..
            }) = event
            {
                should_exit.set(true);
            }
        }
    });

    if should_exit.get() {
        hooks.use_context_mut::<SystemContext>().exit();
    }

    let bg_color = if dark_mode.get() {
        Color::Black
    } else {
        Color::White
    };

    element! {
        ContextProvider(value: Context::owned(ThemeState(dark_mode))) {
            View(
                width: 100pct,
                height: 100pct,
                background_color: bg_color,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
            ) {
                Text(color: Color::Grey, content: "Press 'q' to quit")
                View(
                    padding: 2,
                    border_style: BorderStyle::Double,
                    border_color: Color::Magenta,
                ) {
                    ThemeDisplay
                }
            }
        }
    }
}


