use tera;

fn main() {
    output();
}

fn output() {
    let mut tera = tera::Tera::default();
    let context = template_context();
    let template_name = "template";
    let template_string = include_str!("template.tera");
    tera.add_raw_template(template_name, template_string).ok();
    let rendered = tera.render(template_name, &context).unwrap();
    println!("{}", rendered);
}

fn template_context() -> tera::Context {
    let mut context = tera::Context::new();
    context.insert("user", &user());
    context.insert("host", &host());
    context
}

fn user() -> String {
    "ssiyad".to_string()
}

fn host() -> String {
    "baghdad".to_string()
}
