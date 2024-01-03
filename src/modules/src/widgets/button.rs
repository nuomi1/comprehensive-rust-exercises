use super::{Label, Widget};

pub struct Button {
    label: Label,
}

impl Button {
    pub fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

impl Button {
    fn inner_width(&self) -> usize {
        self.label.width()
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.inner_width() + 4
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut inner = String::new();
        self.label.draw_into(&mut inner);

        let inner_width = self.inner_width();

        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
        for line in inner.lines() {
            writeln!(buffer, "| {:<inner_width$} |", line).unwrap();
        }
        writeln!(buffer, "+-{:-<inner_width$}-+", "").unwrap();
    }
}
