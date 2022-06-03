use chrono::{DateTime, NaiveDateTime, Utc};
use katex;
use pulldown_cmark::{html, Options, Parser};
use regex::Regex;
use rocket::serde::json;
use rocket::serde::{Deserialize, Serialize};
use rocket::tokio;
use rocket::tokio::io::AsyncReadExt;
use std::io::Result as IoResult;
use std::io::Write;
use std::path::Path;

pub fn compile_markdown(path: impl AsRef<Path>) -> IoResult<()> {
    lazy_static! {
        static ref OPTIONS: Options = {
            let mut o = Options::empty();
            o.insert(Options::ENABLE_STRIKETHROUGH);
            o.insert(Options::ENABLE_TABLES);
            o
        };
    }

    let input = std::fs::read_to_string(path)?;

    // skim off the metadata
    let mut iter = input.splitn(3, '\n');
    let title = iter.next().expect("Invalid Article Format");
    let timestamp = iter
        .next()
        .expect("Invalid Article Format")
        .parse()
        .expect("Invalid Timestamp");

    let mut rest = iter.next().expect("Invalid Article Format").to_string();
    replace_latex(&mut rest);

    let parser = Parser::new_ext(&rest, *OPTIONS);
    let mut output = String::new();
    html::push_html(&mut output, parser);

    let article = Article::new(title.to_string(), output, timestamp);

    article.persist()
}

pub fn replace_latex(input: &mut String) {
    lazy_static! {
        static ref PAT: Regex = Regex::new(r"\$\$(.+)\$\$").unwrap();
    }

    // this is probably the dumbest way to do this, but it should be fast enough
    // n^3 :grimace:
    while let Some(m) = (*PAT).find(&input.clone()) {
        let s = m.as_str().trim_matches('$');
        input.replace_range(m.range(), &katex::render(s).expect("Invalid Latex"));
    }
}

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

    fn persist(&self) -> IoResult<()> {
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(Path::new("articles/json").join(format!("{}.json", self.filename)))?;

        file.write_all(serde_json::to_string(self)?.as_bytes())?;

        Ok(())
    }

    pub async fn read_article(path: impl AsRef<Path>) -> IoResult<Self> {
        let mut file = tokio::fs::OpenOptions::new()
            .read(true)
            .open(Path::new("articles/json").join(path.as_ref().with_extension("json")))
            .await?;

        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        let article = json::from_str(&contents).unwrap();

        Ok(article)
    }

    pub async fn read_articles() -> IoResult<Vec<Self>> {
        let mut iter = tokio::fs::read_dir("articles/json").await?;
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
                ' ' | '-' => Some('-'),
                _ => None,
            })
            .collect()
    }

    fn truncate_body(body: String) -> String {
        // manually iterate instead of using `take(120)` because we want to ignore
        // html tags in our character count
        let first_line = body.splitn(2, '\n').next().unwrap().to_string();
        let mut shortened = Vec::new();
        let mut in_brackets = false;
        let mut i = 0;
        for ch in first_line.trim_end().chars() {
            if i == 200 {
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

        if shortened.len() < 200 {
            // if it's less than 120 chars, we didn't truncate anything,
            // so we know don't need to do any more work
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
            while let Some(ch) = shortened.pop() {
                if ch == ' ' {
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
