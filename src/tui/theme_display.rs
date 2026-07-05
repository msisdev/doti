use crate::tui::toggle_button::ToggleButton;
use crate::tui::ThemeState;
use iocraft::prelude::*;

#[component]
pub fn ThemeDisplay(hooks: Hooks) -> impl Into<AnyElement<'static>> {
    let theme_state = hooks.use_context::<ThemeState>();
    let is_dark = theme_state.0.get();
    
    let text = if is_dark {
        "Currently in Dark Mode"
    } else {
        "Currently in Light Mode"
    };
    let color = if is_dark {
        Color::White
    } else {
        Color::Black
    };

    element! {
        View(flex_direction: FlexDirection::Column, align_items: AlignItems::Center) {
            Text(content: "Shared Theme Logic", weight: Weight::Bold, color: color)
            Text(content: text, color: color)
            ToggleButton
        }
    }
}
