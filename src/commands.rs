#[cfg(feature = "shapes")]
use primit::{Circle, Rect, RoundedRect};

use crate::features::RenderFeature;

#[allow(unused)]

pub enum Commands {
    #[cfg(feature = "shapes")]
    RectCommand(Rect),
    #[cfg(feature = "shapes")]
    CircleCommand(Circle),
    #[cfg(feature = "shapes")]
    RoundedRectCommand(RoundedRect),
    FeatureCommand(Box<dyn RenderFeature>),
}
