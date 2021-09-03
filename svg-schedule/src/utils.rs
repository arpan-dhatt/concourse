use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

const COLORS: [&str; 18] = [
    "darkmagenta",
    "darkred",
    "darkslategrey",
    "darkblue",
    "darkgreen",
    "darkcyan",
    "black",
    "darkorange",
    "darkviolet",
    "darkslateblue",
    "darkorchid",
    "darkkhaki",
    "darkgoldenrod",
    "mediumblue",
    "midnightblue",
    "maroon",
    "firebrick",
    "dimgrey",
];

pub fn choose_color(s: &str) -> &'static str {
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    let hash = hasher.finish();
    COLORS[hash as usize % COLORS.len()]
}
