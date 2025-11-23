mod sources;

use chrono::prelude::*;
use clap::Parser;
use date_differencer;
use tera;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Username to show in the output
    #[arg(long)]
    user: String,

    /// Hostname to show in the output
    #[arg(long)]
    host: String,

    /// GitHub Username to fetch data for
    #[arg(long)]
    github_user: String,

    /// GitHub Token for API access
    #[arg(long)]
    github_token: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    output(args).await;
}

async fn output(args: Args) {
    let mut tera = tera::Tera::default();
    let context = template_context(args).await;
    let template_name = "template";
    let template_string = include_str!("template.tera");
    tera.add_raw_template(template_name, template_string).ok();
    let rendered = tera.render(template_name, &context).unwrap();
    print!("{}", rendered);
}

async fn template_context(args: Args) -> tera::Context {
    let octo = octocrab::OctocrabBuilder::default()
        .personal_token(args.github_token)
        .build()
        .unwrap();
    let mut context = tera::Context::new();
    context.insert("user", &args.user);
    context.insert("host", &args.host);
    context.insert("uptime", &uptime());
    context.insert(
        "commit_count",
        &sources::commit_count(&octo, &args.github_user).await,
    );
    context.insert(
        "pr_count",
        &sources::pr_count(&octo, &args.github_user).await,
    );
    context
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
