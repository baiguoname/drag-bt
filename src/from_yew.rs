use yew::prelude::*;

pub struct Counter {
    count: i32,
}

pub enum Msg {
    Increment,
    Decrement,
}

impl Component for Counter {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { count: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.count += 1;
                true
            }
            Msg::Decrement => {
                self.count -= 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <button onclick={ctx.link().callback(|_| Msg::Decrement)}>{ "-1" }</button>
                <p>{ self.count }</p>
                <button onclick={ctx.link().callback(|_| Msg::Increment)}>{ "+1" }</button>
            </div>
        }
    }
}