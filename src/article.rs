use chrono::{DateTime, NaiveDateTime, Utc};
use rocket::serde::json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio::fs;
use rocket::tokio::io::AsyncReadExt;
use std::io::Result as IoResult;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
#[serde(crate = "rocket::serde")]
pub struct Article {
    pub title: String,
    pub body: String,
    pub truncated_body: String,
    timestamp: i64,
    pub formatted_time: String,
    pub filename: String,
}

impl Article {
    pub fn new(title: String, body: String, timestamp: i64) -> Self {
        Article {
            title: title.clone(),
            body: body.clone(),
            truncated_body: Self::truncate_body(body),
            timestamp,
            formatted_time: Self::parse_timestamp(timestamp),
            filename: Self::filename(title),
        }
    }

    pub async fn read_article(path: impl AsRef<Path>) -> IoResult<Self> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(Path::new("articles").join(path.as_ref().with_extension("json")))
            .await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let article = json::from_str(&contents).unwrap();

        Ok(article)
    }

    pub async fn read_articles() -> IoResult<Vec<Self>> {
        let mut iter = fs::read_dir("articles").await?;
        let mut articles = Vec::new();

        while let Ok(Some(entry)) = iter.next_entry().await {
            articles.push(Self::read_article(entry.path().file_name().unwrap()).await?)
        }

        articles.sort_by_key(|a| -a.timestamp);

        Ok(articles)
    }

    fn filename(s: String) -> String {
        s.chars()
            .filter_map(|ch| match ch {
                x if x.is_ascii_alphanumeric() => Some(x.to_ascii_lowercase()),
                ' ' => Some('-'),
                _ => None,
            })
            .collect()
    }

    fn truncate_body(body: String) -> String {
        // manually iterate instead of using `take(120)` because we want to ignore
        // html tags in our character count
        let mut shortened = Vec::new();
        let mut in_brackets = false;
        let mut i = 0;
        for ch in body.trim_end().chars() {
            if i == 120 {
                break;
            }
            // this isn't very robust, but we can just try to avoid writing <>
            // in the first 120 chars
            match ch {
                '<' => in_brackets = true,
                '>' => in_brackets = false,
                _ => {
                    if !in_brackets {
                        i += 1;
                    }
                }
            }
            shortened.push(ch);
        }

        if shortened.len() < 120 {
            // if it's less than 120 chars, we didn't truncate anything,
            // so we know it's valid
        } else if let Some(i) = shortened
            .iter()
            .rev()
            .position(|&ch| ch == '.' || ch == '!' || ch == '?')
        {
            // truncate to the last complete sentence
            // assume these punctuation marks will end a sentence
            shortened.truncate(shortened.len() - i);
        } else {
            // assume that the first 120 chars are not one big word
            // pop chars until we reach a space
            while let Some(b) = shortened.pop() {
                if b == ' ' {
                    break;
                }
            }
            shortened.extend("...".chars());
        }

        shortened.into_iter().collect()
    }

    fn parse_timestamp(timestamp: i64) -> String {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp, 0), Utc);
        format!("{}", dt.format("%B %e, %Y"))
    }
}
