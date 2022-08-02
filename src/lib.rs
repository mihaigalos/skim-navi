extern crate skim;
use skim::prelude::*;
use std::io::{Cursor, Error};

use std::future::Future;

#[derive(Debug, PartialEq)]
pub enum Navigation {
    _Unknown,
    Running,
    OutOf,
    Finished,
}

pub struct Navi;

impl Navi {
    pub async fn run<Fut>(base_url: String, handler: impl Fn(&str) -> Fut)
    where
        Fut: Future<Output = Result<Vec<String>, Error>>,
    {
        let options = SkimOptionsBuilder::default()
            .height(Some("50%"))
            .multi(false)
            .bind(vec!["/:accept", "Enter:accept", "Esc:abort", "Tab:accept"])
            .build()
            .unwrap();
        let mut subpath = "/".to_string();
        loop {
            let path = base_url.clone() + &subpath;
            let items = handler(&path).await.unwrap().join("\n");

            let item_reader = SkimItemReader::default();
            let items = item_reader.of_bufread(Cursor::new(items));
            let selected_items = Skim::run_with(&options, Some(items))
                .map(|out| match out.final_key {
                    Key::Char('/') => out
                        .selected_items
                        .iter()
                        .map(|i| Self::navigate_into(&i.text()))
                        .collect(),
                    Key::Tab => out
                        .selected_items
                        .iter()
                        .map(|i| Self::navigate_into(&i.text()))
                        .collect(),
                    Key::Enter => out
                        .selected_items
                        .iter()
                        .map(|i| Self::navigate_enter(&i.text()))
                        .collect(),
                    _ => Vec::new(),
                })
                .unwrap();

            let item = &selected_items[0];
            if item.0 == Navigation::Finished {
                break;
            }

            if item.0 == Navigation::OutOf {
                subpath = Self::up(&subpath).to_string();
            } else {
                subpath = "/".to_string() + &item.1;
            }
        }
    }

    fn up(path: &str) -> &str {
        if path.matches('/').count() > 0 {
            return &path[..path.rfind('/').unwrap()];
        }
        path
    }

    fn navigate_outof(item: &str) -> (Navigation, String) {
        println!("{}", item);
        (Navigation::OutOf, item.to_string())
    }

    fn navigate_into(item: &str) -> (Navigation, String) {
        println!("/{}", item);
        (Navigation::Running, item.to_string())
    }

    fn navigate_enter(item: &str) -> (Navigation, String) {
        if item == ".." {
            return Self::navigate_outof(item);
        }
        println!("Navigation finished: {}", item);
        (Navigation::Finished, item.to_string())
    }
}
