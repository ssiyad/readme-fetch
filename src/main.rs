mod sources;

use chrono::prelude::*;
use date_differencer;
use std::env;
use tera;

#[tokio::main]
async fn main() {
    output().await;
}

async fn output() {
    let mut tera = tera::Tera::default();
    let context = template_context().await;
    let template_name = "template";
    let template_string = include_str!("template.tera");
    tera.add_raw_template(template_name, template_string).ok();
    let rendered = tera.render(template_name, &context).unwrap();
    print!("{}", rendered);
}

async fn template_context() -> tera::Context {
    let octo = octocrab::instance()
        .user_access_token(github_token())
        .unwrap();
    let mut context = tera::Context::new();
    context.insert("user", &user());
    context.insert("host", &host());
    context.insert("uptime", &uptime());
    context.insert(
        "commit_count",
        &sources::commit_count(octo, &github_user()).await,
    );
    context
}

fn github_user() -> String {
    env::var("GITHUB_USER").expect("GITHUB_USER not set")
}

fn github_token() -> String {
    env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set")
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
    format!(
        "{} years, {} months, {} days",
        date_diff.years, date_diff.months, date_diff.days
    )
}
