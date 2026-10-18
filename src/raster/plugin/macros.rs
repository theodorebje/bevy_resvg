macro_rules! opt_read_events {
    ($svg_events:expr, $($asset_event:path) | +) => {{
        let mut ids = None;
        for event in $svg_events.read() {
            match event {
                $($asset_event { id }) | + => {
                    ids.get_or_insert_with(::std::collections::HashSet::new).insert(*id);
                }
                _ => (),
            }
        }
        ids
    }};
}

macro_rules! read_events {
    ($svg_events:expr, $($asset_event:path) | +) => {{
        let Some(ids) = crate::raster::plugin::macros::opt_read_events!($svg_events, $($asset_event) | +) else {
            return;
        };
        ids
    }};
}

pub(super) use {opt_read_events, read_events};
