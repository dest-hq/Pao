// use primit::{Circle, Rect, RoundedRect};

use crate::features::RenderFeature;

#[allow(unused)]

pub enum Commands {
    // RectCommand(Rect),
    // CircleCommand(Circle),
    // RoundedRectCommand(RoundedRect),
    FeatureCommand(Box<dyn RenderFeature>),
}
