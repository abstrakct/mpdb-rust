use crate::models::_entities::cities;
use loco_rs::prelude::*;

pub fn list(v: &impl ViewRenderer, items: &Vec<cities::Model>) -> Result<Response> {
    format::render().view(v, "cities.html", data!({"items": items}))
}
