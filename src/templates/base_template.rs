use std::collections::HashMap;
use std::fs;

pub fn render(template_name: &str, context: Option<HashMap<&str, &str>>) -> String {
    let template_path = format!("templates/{}", template_name);
    let mut content = fs::read_to_string(template_path).expect("Template file not found");

    if let Some(ctx) = context {
        for (key, value) in ctx {
            let placeholder = format!("{{{{{}}}}}", key);
            content = content.replace(&placeholder, value);
        }
    }

    content
}