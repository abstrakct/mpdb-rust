use crate::views;
use loco_rs::prelude::*;

pub async fn render_home(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    views::dashboard::home(v)
}

pub fn routes() -> Routes {
    Routes::new().prefix("home").add("/", get(render_home))
}
