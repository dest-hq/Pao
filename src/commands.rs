use primit::{Circle, Rect, RoundedRect};

pub enum Commands {
    RectCommand(Rect),
    CircleCommand(Circle),
    RoundedRectCommand(RoundedRect),
}
