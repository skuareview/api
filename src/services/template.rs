use handlebars::Handlebars;
use serde_json::json;

pub fn confirmation_email(code: i32) -> String {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file("alert", "templates/confirmation_email.hbs")
        .unwrap();

    let data0 = json!({
        "title": "example 0",
        "text": "base0",
        "code": code.to_string()
    });

    return handlebars.render("alert", &data0).unwrap();
}
