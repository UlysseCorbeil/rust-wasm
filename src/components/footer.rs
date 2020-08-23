use yew::prelude::*;

pub struct Footer {}

impl Component for Footer {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="footer">
                <div class="footer-container">
                    <div class="footer-watermark footer-element">{"Code and design by Ulysse Corbeil ©"}</div>
                    <div class="footer-github footer-element">
                        <a href="https://github.com/UlysseCorbeil/rust-wasm" target="_blank">
                            <img src="/images/github.png" />
                        </a>
                    </div>
                </div>
            </div>
        }
    }
}
