mod components;
mod ui;
mod utils;

#[cfg(test)]
mod tests {
    use svg::Document;

    use crate::ui::{
        render_class, render_command_text, render_horizontal_lines, render_name_text,
        render_vertical_lines, render_weekday_texts, ClassInformation, VIEWPORT_HEIGHT,
        VIEWPORT_WIDTH,
    };

    #[test]
    fn it_works() {
        let mut doc = Document::new().set(
            "viewBox",
            (0, 0, VIEWPORT_WIDTH as i32, VIEWPORT_HEIGHT as i32),
        );
        doc = render_command_text(doc, "/ccviz");
        doc = render_name_text(doc, "SamosaGuru");
        doc = render_weekday_texts(doc);
        doc = render_horizontal_lines(doc);
        doc = render_vertical_lines(doc);
        doc = render_class(
            ClassInformation {
                code: 69420,
                name: "CS314".to_string(),
                detail: "Lecture".to_string(),
                time: (1, 480, 570),
                instructor: Some("instructor".to_string()),
                room: None,
            },
            doc,
        );
        svg::save("image.svg", &doc).unwrap();
    }
}
