#![allow(clippy::result_large_err)]
use loco_rs::prelude::*;

use crate::models::_entities::meter_types;

/// Render a list view of `meter_types`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<meter_types::Model>) -> Result<Response> {
    format::render().view(v, "meter_types/list.html", data!({"items": items}))
}

/// Render a single `meter_types` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &meter_types::Model) -> Result<Response> {
    format::render().view(v, "meter_types/show.html", data!({"item": item}))
}

/// Render a `meter_types` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "meter_types/create.html", data!({}))
}

/// Render a `meter_types` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &meter_types::Model) -> Result<Response> {
    format::render().view(v, "meter_types/edit.html", data!({"item": item}))
}
