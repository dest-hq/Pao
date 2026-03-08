// use primit::{Circle, Rect, RoundedRect};

use crate::features::RenderFeature;

/// Render commands that will be executed by the [`Canvas`]
pub enum Commands {
    // RectCommand(Rect),
    // CircleCommand(Circle),
    // RoundedRectCommand(RoundedRect),
    FeatureCommand(Box<dyn RenderFeature>),
}
