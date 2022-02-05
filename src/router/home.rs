use yew::prelude::*;
use crate::components::{script::Script,
    footer::footer::Footer
};

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
                <div data-scroll-container="true">
                    <div class="section py" data-scroll-section="true">
                        <h2 data-scroll="true" data-scroll-speed="4"  data-scroll-position="top">{"Online Store"}</h2>
                            <p data-scroll="true" data-scroll-speed="-4" data-scroll-direction="horizontal">
                            {"Please scroll down"}
                            </p>
                    </div>
                    <div id="gallery" data-scroll-section="true">
                        <div class="grid">
                            <div class="img">
                                <img src="./assets/img/4.jpg" alt=""/>
                                <img src="./assets/img/8.jpg" alt=""/>
                                <img src="./assets/img/03.jpg" alt=""/>
                                <img src="./assets/img/7.jpg" alt=""/>
                            </div>
                            <div>
                                <img src="./assets/img/01.jpg" alt="" 
                                data-scroll="true"
                                data-scroll-sticky="true"
                                data-scroll-target="#gallery"
                                data-scroll-speed="1.2"
                                />
                            </div>
                        </div>
                    </div>
                    // <div class="threeD" id="threeD" data-scroll-section="">
                    //     <h2>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="4.5">{"3"}</span>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="4.1">{"D"}</span>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="3.4">{"V"}</span>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="3.5">{"I"}</span>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="3.7">{"B"}</span>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="3.1">{"E"}</span>
                    //         <span data-scroll="" data-scroll-repeat="" data-scroll-speed="2.4">{"S"}</span>
                    //     </h2>
                    
                    //     <div class="skewsec" data-scroll="" data-scroll-direction="horizontal" data-scroll-speed="20" data-scroll-target="#threeD">
                    //         <span>{"This is infact an Ecommerce Website"}</span>
                    //     </div>
                    //     <div class="skewsec"
                    //         data-scroll=""
                    //         data-scroll-direction="horizontal"
                    //         data-scroll-speed="20"
                    //         data-scroll-target="#threeD">
                    //         <span>{"Made by Philip Daquim"}</span>
                    //     </div>
                    // </div>
                </div>
                <script src="./assets/js/main.js"></script>
                // <script crossorigin="false" src="./assets/js/main.js"></script>
                <Script/>
                <Footer/>
            </>
        }
    }
}