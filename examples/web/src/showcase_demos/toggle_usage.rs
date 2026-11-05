use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::toggle().theme(theme).label("Pin this document")
}
