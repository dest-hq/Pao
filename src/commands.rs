// use primit::{Circle, Rect, RoundedRect};

use std::{cell::RefCell, rc::Rc};

use crate::features::RenderFeature;

/// Render commands that will be executed by the [`Canvas`]
pub enum Commands {
    // RectCommand(Rect),
    // CircleCommand(Circle),
    // RoundedRectCommand(RoundedRect),
    FeatureCommand(Rc<RefCell<dyn RenderFeature>>),
}
