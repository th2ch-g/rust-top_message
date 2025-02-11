use crate::components::navbar::Navbar;
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props;

pub struct NotFound;

impl Component for NotFound {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! { <Navbar /> }
    }
}
