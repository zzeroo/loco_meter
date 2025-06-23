#![allow(clippy::result_large_err)]
use loco_rs::prelude::*;

use crate::models::_entities::units;

/// Render a list view of `units`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<units::Model>) -> Result<Response> {
    format::render().view(v, "units/list.html", data!({"items": items}))
}

/// Render a single `units` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &units::Model) -> Result<Response> {
    format::render().view(v, "units/show.html", data!({"item": item}))
}

/// Render a `units` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "units/create.html", data!({}))
}

/// Render a `units` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &units::Model) -> Result<Response> {
    format::render().view(v, "units/edit.html", data!({"item": item}))
}
