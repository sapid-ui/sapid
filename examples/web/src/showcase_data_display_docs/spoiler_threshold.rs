use glory::prelude::*;
pub fn demo(theme: sapid::Theme) -> impl Widget {
    sapid::stack().theme(theme.clone()).gap(20.).fill((
        sapid::spoiler("short-guide")
            .theme(theme.clone())
            .max_height(80.)
            .content("A short walk around the lakeside village."),
        sapid::spoiler("long-guide")
            .theme(theme.clone())
            .max_height(60.)
            .content(TEXT_1),
    ))
}

const TEXT_1: &str = "The trail begins beside the lake, where a short boardwalk leads into the old village. Follow the stone steps to a quiet lookout above the rooftops.\n\nIn the morning the mountains reflect in the water and small boats cross toward the opposite shore. Leave time for the shaded woodland path and a stop at the village bakery.\n\nThe final stretch climbs gently through the pines before returning to the lakeside. Comfortable shoes, a refillable water bottle and a light jacket are useful for the changing mountain weather.";
