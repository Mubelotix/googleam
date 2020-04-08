#![recursion_limit="256"]

use yew::prelude::*;
use wasm_bindgen::{prelude::*, JsCast};
use yew::services::{FetchService, ConsoleService};
use yew::format::{Json, Nothing};
use yew::services::fetch::{Request, Response, FetchTask};
use serde_json::{from_str, Value};
use anyhow::Error;
use std::rc::Rc;
use std::cell::RefCell;
use urlencoding::encode;

mod miscellaneous;
use crate::miscellaneous::*;

struct Model {
    link: ComponentLink<Self>,
    fetch_service: FetchService,
    console_service: ConsoleService,
    results: Rc<RefCell<Vec<Giveaway>>>,
    tasks: Vec<FetchTask>
}

enum Msg {
    Input(String),
    ResultsOk,
    Error
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            fetch_service: FetchService::new(),
            console_service: ConsoleService::new(),
            results: Rc::new(RefCell::new(Vec::new())),
            tasks: Vec::new()
        }
    }

    #[allow(unused_must_use)]
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(query) => {
                let request = Request::get(format!("http://localhost:7700/indexes/giveaways/search?q={}", encode(&query)))
                    .header("X-Meili-API-Key", "321fde49d647cb8dd3ce20f75fb3b3afcd10be5995d560d4d8151abaa57e1580")
                    .body(Nothing)
                    .expect("Failed to build request.");

                let results2 = Rc::clone(&self.results);
                let task = self.fetch_service.fetch(
                    request,
                    self.link.callback(move |response: Response<Result<String, Error>>| {
                        if response.status().is_success() {
                            let (_parts, body) = response.into_parts();
                            let body = body.unwrap();
                            *results2.borrow_mut() = from_str::<MeiliResponse>(&body).unwrap().hits;
                            Msg::ResultsOk
                        } else {
                            Msg::Error
                        }
                    }),
                );
                self.tasks.push(task.unwrap());
            }
            Msg::ResultsOk => (),
            _ => ()

        }
        true
    }

    fn view(&self) -> Html {
        let timestamp = get_timestamp();

        let results = self.results.borrow();
        let results = results.iter().map(|entry| {
            let time_remaining = entry.end_date as i64 - timestamp as i64;
            if time_remaining > 0 {
                html! {
                    <a href=entry.get_url()>
                        <h2>{&entry.name}</h2>
                        <p>{if entry.description.len() > 0 { &entry.description } else {&entry.name}}</p>
                        <div>
                            {
                                if let Some(entry_count) = entry.entry_count {
                                    html! {<span class="entries">{format!("{} entries", entry_count)}</span>}
                                } else {
                                    html! {}
                                }
                            }
                            <span class="remaining_time">{"ending in "}{seconds_to_string(entry.end_date as i64 - timestamp as i64, true)}</span>
                        </div>
                    </a>
                }
            } else {
                html! {}
            }
        });

        html! {
            <main>
                <div id="results">{ for results }</div>
                <form autocomplete="off">
                    <input type="text" name="q" placeholder="Query" oninput=self.link.callback(|data: InputData| Msg::Input(data.value))/>
                    <h1>{"Googleam"}</h1>
                </form>
            </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
}