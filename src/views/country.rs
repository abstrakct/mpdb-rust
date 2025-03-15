use loco_rs::prelude::*;

use crate::models::_entities::countries;

/// Render a list view of `countries`.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<countries::Model>) -> Result<Response> {
    format::render().view(v, "country/list.html", data!({"items": items}))
}

/// Render a single `country` view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &countries::Model) -> Result<Response> {
    format::render().view(v, "country/show.html", data!({"item": item}))
}

/// Render a `country` create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer) -> Result<Response> {
    format::render().view(v, "country/create.html", data!({}))
}

/// Render a `country` edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &countries::Model) -> Result<Response> {
    format::render().view(v, "country/edit.html", data!({"item": item}))
}
