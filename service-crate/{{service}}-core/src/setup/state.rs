use std::sync::Arc;

use {{service}}_sdk::service::{{entity}}::{{entity | pascal_case}}Service;

#[derive(Clone)]
pub struct AppState {
    pub(crate) {{entity}}: Arc<dyn {{entity | pascal_case}}Service>,
}

impl AppState {
    pub fn new({{entity}}: Arc<dyn {{entity | pascal_case}}Service>) -> Self {
        Self { {{entity}} }
    }
}
