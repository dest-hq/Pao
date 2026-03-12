use std::{cell::RefCell, rc::Rc};

use crate::features::RenderFeature;

/// Render commands that will be executed by the [`Canvas`]
pub enum Commands {
    FeatureCommand(Rc<RefCell<dyn RenderFeature>>),
}
