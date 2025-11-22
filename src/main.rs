use chrono::prelude::*;
use date_differencer;
use std::env;
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
    context.insert("uptime", &uptime());
    context
}

fn user() -> String {
    env::var("FETCH_USER").expect("FETCH_USER not set")
}

fn host() -> String {
    env::var("FETCH_HOST").expect("FETCH_HOST not set")
}

fn uptime() -> String {
    let today = Local::now();
    let birthday = Local.with_ymd_and_hms(1998, 3, 28, 0, 0, 0).unwrap();
    let date_diff = date_differencer::date_diff(birthday, today);
    return format!(
        "{} years, {} months, {} days",
        date_diff.years, date_diff.months, date_diff.days
    );
}
