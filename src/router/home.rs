use yew::prelude::*;

pub struct Home;

impl Component for Home { 
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        false
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="container">
                    <div class="section py">
                        <h1>{"Scrolling Down"}</h1>
                        <p>{"Please Scroll Down"}</p>
                    </div>
                </div>
            </>
        }
    }
}