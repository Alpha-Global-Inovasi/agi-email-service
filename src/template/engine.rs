use crate::error::EmailError;
use handlebars::Handlebars;
use serde_json::Value;

pub struct TemplateEngine {
    registry: Handlebars<'static>,
}

impl TemplateEngine {
    pub fn new() -> Self {
        Self {
            registry: Handlebars::new(),
        }
    }

    pub fn register_template(
        &mut self,
        name: &str,
        template_string: &str,
    ) -> Result<(), EmailError> {
        self.registry
            .register_template_string(name, template_string)
            .map_err(|e| EmailError::TemplateError(e.to_string()))
    }

    pub fn render(&self, name: &str, data: &Value) -> Result<String, EmailError> {
        self.registry
            .render(name, data)
            .map_err(|e| EmailError::TemplateError(e.to_string()))
    }
}
