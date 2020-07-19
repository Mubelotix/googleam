#![recursion_limit="256"]

use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew::services::FetchService;
use yew::format::Nothing;
use yew::services::fetch::{Request, Response, FetchTask};
use serde_json::{from_str};
use anyhow::Error;
use std::rc::Rc;
use std::cell::RefCell;
use urlencoding::encode;

mod miscellaneous;
use crate::miscellaneous::*;

struct Model {
    link: ComponentLink<Self>,
    results: Rc<RefCell<Vec<Giveaway>>>,
    is_query_empty: bool,
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
            results: Rc::new(RefCell::new(Vec::new())),
            is_query_empty: true,
            tasks: Vec::new()
        }
    }

    #[allow(unused_must_use)]
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(query) => {
                #[cfg(debug_assertions)]
                let request = Request::get(format!("http://localhost:7700/indexes/giveaways/search?q={}", encode(&query)))
                    .body(Nothing)
                    .expect("Failed to build request.");
                #[cfg(not(debug_assertions))]
                let request = Request::get(format!("https://googleam.mubelotix.dev:7700/indexes/giveaways/search?q={}", encode(&query)))
                    .header("X-Meili-API-Key", "321fde49d647cb8dd3ce20f75fb3b3afcd10be5995d560d4d8151abaa57e1580")
                    .body(Nothing)
                    .expect("Failed to build request");

                self.is_query_empty = query.len() == 0;

                let results2 = Rc::clone(&self.results);
                let task = FetchService::fetch(
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

        if self.is_query_empty {
            html! {
                <main>
                    <form autocomplete="off" id="centered_form">
                        <h1>{"Googleam"}</h1>
                        <input type="text" name="q" placeholder="Query" oninput=self.link.callback(|data: InputData| Msg::Input(data.value))/>
                        
                    </form>
                </main>
            }
        } else {
            html! {
                <main>
                    <form autocomplete="off" id="top_form">
                        <h1>{"Googleam"}</h1>
                        <input type="text" name="q" placeholder="Query" oninput=self.link.callback(|data: InputData| Msg::Input(data.value))/>
                    </form>
                    <div id="results">{ for results }</div>
                </main>
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> bool {
        true
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
}