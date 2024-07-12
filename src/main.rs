#![allow(non_snake_case)]

mod content;
mod lints;
mod message;

use dioxus::prelude::*;
use std::str::FromStr;
use tracing::Level;

fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

#[component]
fn App() -> Element {
    let mut s = use_signal(String::new);

    let upload = move |evt: FormEvent| async move {
        if let Some(engine) = evt.files() {
            if let Some(file) = engine.files().first() {
                let content = engine.read_file_to_string(file).await.unwrap();
                s.set(content);
            }
        }
    };

    let results = use_memo(move || {
        let mut errors = Vec::new();

        let r = content::Content::from_str(&s())
            .map(|c| c.check(&mut errors))
            .inspect_err(|e| tracing::error!("Error: {}", e))
            .unwrap();

        rsx! {
            if let Ok(r) = r {
                if r != 0 {
                    p { "🎉 No error found. Good job ! " }
                }
            } else {
                for cell in errors {
                    p {
                        span { class: "lint-type", {format!("Cell {}", cell.0)} }
                    }
                    for e in cell.1.iter() {
                        p {
                            span { class: "lint-message", {e.1.to_string()} }
                        }
                    }
                }
            }
        }
    });

    rsx! {
        link { rel: "stylesheet", href: "main.css" }
        div { id: "inputs",
            button  {
                onclick: |_| eval("document.getElementById('open-file-input').click()").send(().into()).unwrap(),
                "🔬 Load subtitle file"
            }
            input {
                id: "open-file-input",
                r#type: "file",
                accept: ".txt",
                style: "display: none",
                onchange: upload,
            }
        }
        div { id: "outputs",
            div { id: "results", {results} }
        }
    }
}
