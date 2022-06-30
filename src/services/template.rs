use handlebars::Handlebars;
use serde_json::json;

pub fn alert_template() -> String {
    let mut handlebars = Handlebars::new();
    handlebars.register_template_file("alert", "templates/alert.hbs");

    let data0 = json!({
        "title": "example 0",
        "text": "base0"
    });

    return handlebars.render("alert", &data0).unwrap();
}
