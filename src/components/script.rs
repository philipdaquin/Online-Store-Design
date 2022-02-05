use yew::prelude::*;

#[function_component(Script)]
pub fn script() -> Html {
    html! { 
        <>
            <script src="./assets/js/main.js"></script> 
        </>
    }
}