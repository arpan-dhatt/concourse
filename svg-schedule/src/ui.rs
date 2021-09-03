use crate::{
    components::{Line, Rectangle, Text},
    utils::choose_color,
};
use svg::Document;

pub const VIEWPORT_WIDTH: f32 = 1200.0;
pub const VIEWPORT_HEIGHT: f32 = 900.0;

const COMMAND_FONT_SIZE: i32 = 20;
const NAME_FONT_SIZE: i32 = 40;
const PADDING: f32 = 30.0;
const DAYS: [&str; 5] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday"];
const HORIZONTAL_INTERVAL: f32 = (VIEWPORT_WIDTH - PADDING * 2.0) / DAYS.len() as f32;
const START_TIME: i32 = 480;
const END_TIME: i32 = 1260;
const TIME_INTERVAL: i32 = 30;
const TIME_SEGMENT_AMOUNT: i32 = (END_TIME - START_TIME) / TIME_INTERVAL;
const VERTICAL_INTERVAL: f32 = (VIEWPORT_HEIGHT - PADDING * 6.6) / TIME_SEGMENT_AMOUNT as f32;

pub fn render_command_text(mut doc: Document, text: &str) -> Document {
    doc.add(
        Text::default()
            .position(PADDING, PADDING + COMMAND_FONT_SIZE as f32)
            .font_family("monospace")
            .font_size(COMMAND_FONT_SIZE)
            .text(text)
            .as_svg(),
    )
}

pub fn render_name_text(mut doc: Document, name: &str) -> Document {
    doc.add(
        Text::default()
            .position(PADDING, PADDING * 3.0)
            .font_size(NAME_FONT_SIZE)
            .text(name)
            .as_svg(),
    )
}

pub fn render_weekday_texts(mut doc: Document) -> Document {
    for (i, day) in DAYS.iter().enumerate() {
        doc = doc.add(
            weekday_text_element(
                day,
                HORIZONTAL_INTERVAL * i as f32 + PADDING * 1.5,
                PADDING * 4.6,
            )
            .as_svg(),
        );
    }
    doc
}

fn weekday_text_element(s: &str, x: f32, y: f32) -> Text {
    const FONT_SIZE: i32 = 20;
    Text::default()
        .position(x, y)
        .fill("grey")
        .font_size(FONT_SIZE)
        .text(s)
}

pub fn render_vertical_lines(mut doc: Document) -> Document {
    const AMOUNT: usize = DAYS.len();
    for i in 0..AMOUNT {
        doc = doc.add(
            Line::default()
                .place(
                    HORIZONTAL_INTERVAL * i as f32 + PADDING * 1.4,
                    PADDING * 4.0,
                    HORIZONTAL_INTERVAL * i as f32 + PADDING * 1.4,
                    VIEWPORT_HEIGHT - PADDING,
                )
                .stroke("grey")
                .stroke_width(2.0)
                .as_svg(),
        );
    }
    doc
}

pub fn render_horizontal_lines(mut doc: Document) -> Document {
    for i in 0..=TIME_SEGMENT_AMOUNT {
        doc = doc.add(
            Line::default()
                .place(
                    0.0,
                    i as f32 * VERTICAL_INTERVAL + PADDING * 4.85,
                    VIEWPORT_WIDTH - PADDING * 1.0,
                    i as f32 * VERTICAL_INTERVAL + PADDING * 4.85,
                )
                .stroke("lightgrey")
                .stroke_width(2.0)
                .as_svg(),
        );
        if i % 2 == 0 {
            let time_min = START_TIME + i * TIME_INTERVAL;
            let fmt = format!("{}:{:02}", time_min / 60, time_min % 60);
            doc = doc.add(
                Text::default()
                    .text(&fmt)
                    .text_anchor("end")
                    .font_size(13)
                    .position(
                        PADDING * 1.2,
                        i as f32 * VERTICAL_INTERVAL + PADDING * 4.85 - 5.0,
                    )
                    .fill("grey")
                    .as_svg(),
            );
        }
    }
    doc
}

pub struct ClassInformation {
    pub code: i32,
    pub name: String,
    pub detail: String,
    pub time: (i32, i32, i32),
    pub instructor: Option<String>,
    pub room: Option<String>,
}

pub fn render_class(class_info: ClassInformation, mut doc: Document) -> Document {
    const OUTER_MARGIN: f32 = 2.5;
    const INNER_MARGIN: f32 = 7.5;
    let position = (
        class_info.time.0 as f32 * HORIZONTAL_INTERVAL + PADDING * 1.4 + OUTER_MARGIN,
        ((class_info.time.1 - START_TIME) / TIME_INTERVAL) as f32 * VERTICAL_INTERVAL
            + PADDING * 4.85
            + OUTER_MARGIN,
    );
    let size = (
        HORIZONTAL_INTERVAL - OUTER_MARGIN * 2.0,
        ((class_info.time.2 - class_info.time.1) / TIME_INTERVAL) as f32 * VERTICAL_INTERVAL
            - OUTER_MARGIN * 2.0,
    );
    let top_left_corner = position;
    let top_right_corner = (position.0 + size.0, position.1);
    let bot_left_corner = (position.0, position.1 + size.1);
    let bot_right_corner = (position.0 + size.0, position.1 + size.1);
    let top_left_text = format!("{} {}", class_info.name, class_info.detail);
    const CLASS_FONT_SIZE: i32 = 16;
    doc.add(
        Rectangle::default()
            .position(position.0, position.1)
            .size(size.0, size.1)
            .fill(choose_color(&class_info.name))
            .stroke("none")
            .corner_radius(10.0)
            .as_svg(),
    )
    .add(
        Text::default()
            .text(&format!("{}", class_info.code))
            .fill("white")
            .text_anchor("end")
            .font_size(CLASS_FONT_SIZE)
            .position(
                bot_right_corner.0 - INNER_MARGIN,
                bot_right_corner.1 - INNER_MARGIN,
            )
            .font_weight("bold")
            .as_svg(),
    )
    .add(
        Text::default()
            .text(&class_info.instructor.as_ref().unwrap_or(&("–".to_string())))
            .fill("white")
            .font_size(CLASS_FONT_SIZE)
            .position(
                bot_left_corner.0 + INNER_MARGIN,
                bot_left_corner.1 - INNER_MARGIN,
            )
            .font_weight("bold")
            .as_svg(),
    )
    .add(
        Text::default()
            .text(&top_left_text)
            .fill("white")
            .font_size(CLASS_FONT_SIZE)
            .position(
                top_left_corner.0 + INNER_MARGIN,
                top_left_corner.1 + INNER_MARGIN + CLASS_FONT_SIZE as f32,
            )
            .font_weight("bold")
            .as_svg(),
    )
    .add(
        Text::default()
            .text(&class_info.room.as_ref().unwrap_or(&("–".to_string())))
            .fill("white")
            .font_size(CLASS_FONT_SIZE)
            .position(
                top_right_corner.0 - INNER_MARGIN,
                top_right_corner.1 + INNER_MARGIN + CLASS_FONT_SIZE as f32,
            )
            .text_anchor("end")
            .font_weight("bold")
            .as_svg(),
    )
}
