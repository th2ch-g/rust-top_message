use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;

struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <main>
                    <Switch<Routes> render={switch} />
                </main>
            </BrowserRouter>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
enum Routes {
    #[at("/rust-top_message")]
    Index,
    #[at("/404")]
    #[not_found]
    NotFound,
}

fn switch(routes: Routes) -> Html {
    match routes {
        Routes::Index => html! { <pages::index::IndexPage /> },
        Routes::NotFound => html! { <pages::notfound::NotFound /> },
    }
}
