use crate::components::navbar::Navbar;
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct IndexPage {
    value: String,
    result: String,
}

pub enum Msg {
    InputChanged(String),
    ShowResult,
}

impl Component for IndexPage {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: String::from(""),
            result: String::from(""),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(new_value) => {
                self.value = new_value;
                true
            }
            Msg::ShowResult => {
                // self.result = match bf_core::run(&self.value) {
                //     Ok(s) => s,
                //     Err(e) => {
                //         format!("{:?}", e)
                //     }
                // };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let input: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::InputChanged(input.value())
        });

        // let onclick = ctx.link().callback(|_| Msg::ShowResult);

        html! {
            <>
                <Navbar />
            </>
        }
    }
}
