use yew::prelude::*;
use web_sys::HtmlSelectElement;
use wasm_bindgen::JsCast;
use reqwest::Client;
use super::types::*;

pub struct Model {
    values: Vec<i32>,
    show_dropdown: bool,
    file_content: String,
}

pub enum Msg {
    Add,
    Select(i32),
    ReadFile,
    FileLoaded(String),
    Error(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            values: Vec::new(),
            show_dropdown: false,
            file_content: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Add => {
                self.show_dropdown = true;
            }
            Msg::Select(value) => {
                self.values.push(value);
                self.show_dropdown = false;
            }
            Msg::ReadFile => {
                if let Some(value) = self.values.last() {
                    let file_name = format!("{}.txt", value);
                    let link = ctx.link().clone();
                    wasm_bindgen_futures::spawn_local(async move {
                        let client = Client::new();
                        match client.get(&format!("http://localhost:8081/files/{}", file_name))
                            .send()
                            .await
                        {
                            Ok(response) => {
                                match response.json::<FileResp>().await {
                                    Ok(res) => link.send_message(Msg::FileLoaded(res.content)),
                                    Err(err) => link.send_message(
                                        Msg::Error(format!("Failed to parse JSON: {}", err))
                                    ),
                                }
                            },
                            Err(err) => link.send_message(Msg::Error(format!("Request failed: {}", err))),
                        }
                    });
                }
            }
            Msg::FileLoaded(content) => {
                self.file_content = content;
            }
            Msg::Error(error) => {
                self.file_content = error;
            }
        }
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Add)}>{ "Add" }</button>
                { self.view_dropdown(ctx) }
                <div style="margin-top: 20px; border: 1px solid black; padding: 10px;">
                    { for self.values.iter().map(|value| html! { <p>{ value }</p> }) }
                </div>
                <button onclick={ctx.link().callback(|_| Msg::ReadFile)}>{ "Read File" }</button>
                <textarea value={self.file_content.clone()} readonly=true style="width: 100%; height: 200px;" />
            </div>
        }
    }
}

impl Model {
    fn view_dropdown(&self, ctx: &Context<Self>) -> Html {
        if self.show_dropdown {
            html! {
                <select onchange={ctx.link().callback(|e: Event| {
                    let input: HtmlSelectElement = e.target().unwrap().dyn_into().unwrap();
                    let value = input.value().parse::<i32>().unwrap();
                    Msg::Select(value)
                })}>
                    { for (1..=10).map(|i| html! { <option value={i.to_string()}>{ i }</option> }) }
                </select>
            }
        } else {
            html! {}
        }
    }
}
