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
    pub time: i64,
}

impl Article {
    pub async fn read_article(n: u32) -> IoResult<Article> {
        let mut file = fs::OpenOptions::new()
            .read(true)
            .open(Path::new("articles").join(format!("{}.json", n)))
            .await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let article = json::from_str(&contents).unwrap();

        Ok(article)
    }

    pub fn truncate_body(&self) -> String {
        let s = self.body.clone();

        // manually iterate instead of using `take(120)` because we want to ignore
        // html tags in our character count
        let mut shortened = Vec::new();
        let mut in_brackets = false;
        let mut i = 0;
        for byte in s.trim_end().bytes() {
            if i == 120 {
                break;
            }
            // this isn't very robust, but we can just try to avoid writing <>
            // in the first 120 bytes
            match byte {
                b'<' => in_brackets = true,
                b'>' => in_brackets = false,
                _ => {
                    if !in_brackets {
                        i += 1;
                    }
                }
            }
            shortened.push(byte);
        }

        if shortened.len() < 120 {
            // if it's less than 120 bytes, we didn't truncate anything,
            // so we know it's valid
            String::from_utf8(shortened).unwrap()
        } else if let Some(i) = shortened.iter().rev().position(|&b| b == b'.') {
            shortened.truncate(shortened.len() - i);

            String::from_utf8(shortened).unwrap()
        } else {
            // assume that the first 120 chars are one big word
            // pop bytes until we reach a space
            while let Some(b) = shortened.pop() {
                if b == b' ' {
                    break;
                }
            }

            String::from_utf8(shortened).unwrap() + "..."
        }
    }

    pub fn parse_timestamp(&self) -> String {
        let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(self.time, 0), Utc);
        format!("{}", dt.format("%B %e, %Y"))
    }
}
