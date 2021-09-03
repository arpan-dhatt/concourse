use svg::Node;

pub struct Rectangle {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    fill: String,
    stroke: String,
    stroke_width: f32,
    corner_radius: f32,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            x: 0.0,
            y: 0.0,
            w: 10.0,
            h: 10.0,
            fill: "none".to_string(),
            stroke: "black".to_string(),
            stroke_width: 1.0,
            corner_radius: 1.0,
        }
    }
}

impl Rectangle {
    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn fill(mut self, fill: &str) -> Self {
        self.fill = fill.to_string();
        self
    }

    pub fn stroke(mut self, stroke: &str) -> Self {
        self.stroke = stroke.to_string();
        self
    }

    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn corner_radius(mut self, corner_radius: f32) -> Self {
        self.corner_radius = corner_radius;
        self
    }

    pub fn as_svg(&self) -> impl Node {
        svg::node::element::Rectangle::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("width", self.w)
            .set("height", self.h)
            .set("fill", self.fill.as_str())
            .set("stroke", self.stroke.as_str())
            .set("stroke-width", self.stroke_width)
            .set("rx", self.corner_radius)
    }
}

pub struct Line {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    stroke: String,
    stroke_width: f32,
    stroke_linecap: String,
}

impl Default for Line {
    fn default() -> Self {
        Line {
            x1: 0.0,
            y1: 0.0,
            x2: 20.0,
            y2: 20.0,
            stroke: "black".to_string(),
            stroke_width: 1.0,
            stroke_linecap: "round".to_string(),
        }
    }
}

impl Line {
    pub fn place(mut self, x1: f32, y1: f32, x2: f32, y2: f32) -> Line {
        self.x1 = x1;
        self.y1 = y1;
        self.x2 = x2;
        self.y2 = y2;
        self
    }

    pub fn stroke(mut self, stroke: &str) -> Self {
        self.stroke = stroke.to_string();
        self
    }

    pub fn stroke_width(mut self, stroke_width: f32) -> Self {
        self.stroke_width = stroke_width;
        self
    }

    pub fn stroke_linecap(mut self, stroke_linecap: &str) -> Self {
        self.stroke_linecap = stroke_linecap.to_string();
        self
    }

    pub fn as_svg(&self) -> impl Node {
        svg::node::element::Line::new()
            .set("x1", self.x1)
            .set("y1", self.y1)
            .set("x2", self.x2)
            .set("y2", self.y2)
            .set("stroke", self.stroke.as_str())
            .set("stroke-width", self.stroke_width)
            .set("stroke-linecap", self.stroke_linecap.as_str())
    }
}

pub struct Text {
    text: String,
    x: f32,
    y: f32,
    font_size: i32,
    font_family: String,
    font_weight: String,
    fill: String,
    text_anchor: String,
}

impl Default for Text {
    fn default() -> Self {
        Text {
            text: "Hello, World!".to_string(),
            x: 0.0,
            y: 0.0,
            font_size: 11,
            font_family: "sans-serif".to_string(),
            font_weight: "normal".to_string(),
            fill: "black".to_string(),
            text_anchor: "start".to_string(),
        }
    }
}

impl Text {
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn position(mut self, x: f32, y: f32) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn font_size(mut self, font_size: i32) -> Self {
        self.font_size = font_size;
        self
    }

    pub fn font_family(mut self, font_family: &str) -> Self {
        self.font_family = font_family.to_string();
        self
    }

    pub fn fill(mut self, fill: &str) -> Self {
        self.fill = fill.to_string();
        self
    }

    pub fn text_anchor(mut self, text_anchor: &str) -> Self {
        self.text_anchor = text_anchor.to_string();
        self
    }

    pub fn font_weight(mut self, font_weight: &str) -> Self {
        self.font_weight = font_weight.to_string();
        self
    }

    pub fn as_svg(&self) -> impl Node {
        svg::node::element::Text::new()
            .set("x", self.x)
            .set("y", self.y)
            .set("font-family", self.font_family.as_str())
            .set("font-size", self.font_size)
            .set("fill", self.fill.as_str())
            .set("font-weight", self.font_weight.as_str())
            .set("text-anchor", self.text_anchor.as_str())
            .add(svg::node::Text::new(self.text.to_string()))
    }
}
