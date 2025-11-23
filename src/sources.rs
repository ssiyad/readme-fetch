use chrono::{Days, NaiveDate, Utc};
use octocrab::Octocrab;

pub async fn commit_count(octo: &Octocrab, user: &str) -> u64 {
    let date_format = "%Y-%m-%dT%H:%M:%SZ";
    let date_start = (Utc::now() - Days::new(30)).format(date_format).to_string();
    let date_end = Utc::now().format(date_format).to_string();

    let payload = serde_json::json!({
        "query": r#"
            query($user: String!, $from: DateTime!, $to: DateTime!) {
              user(login: $user) {
                contributionsCollection(from: $from, to: $to) {
                  totalCommitContributions
                }
              }
            }
        "#,
        "variables": {
            "user": user,
            "from": date_start,
            "to": date_end,
        },
    });

    let response: serde_json::Value = octo.graphql(&payload).await.unwrap();
    response["data"]["user"]["contributionsCollection"]["totalCommitContributions"]
        .as_u64()
        .unwrap_or(0)
}

pub async fn pr_count(octo: &Octocrab, user: &str) -> u64 {
    let date_format = "%Y-%m-%dT%H:%M:%SZ";
    let date_start = (Utc::now() - Days::new(30)).format(date_format).to_string();
    let date_end = Utc::now().format(date_format).to_string();

    let payload = serde_json::json!({
        "query": r#"
            query($user: String!, $from: DateTime!, $to: DateTime!) {
              user(login: $user) {
                contributionsCollection(from: $from, to: $to) {
                  totalPullRequestContributions
                }
              }
            }
        "#,
        "variables": {
            "user": user,
            "from": date_start,
            "to": date_end,
        },
    });

    let response: serde_json::Value = octo.graphql(&payload).await.unwrap();
    response["data"]["user"]["contributionsCollection"]["totalPullRequestContributions"]
        .as_u64()
        .unwrap_or(0)
}

pub fn uptime(birthday: &str) -> String {
    let now = Utc::now().naive_utc();
    let birthday_date = NaiveDate::parse_from_str(birthday, "%Y-%m-%d")
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap();
    let diff = date_differencer::date_diff(birthday_date, now);
    format!(
        "{} years, {} months, {} days",
        diff.years, diff.months, diff.days,
    )
}
