use crate::route::Route;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct Nav { }

impl Component for Nav {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <nav class="nav">
                <div class="nav-container">
                    <div class="nav-inner-ctn">
                        <Anchor route=Route::HomePage classes="nav_home_button">
                            <div class="nav_title">{"Home"}</div>
                        </Anchor>
                        <Anchor route=Route::About classes="nav_home_button">
                            <div class="nav_title">{"About"}</div>
                        </Anchor>
                    </div>
                </div>
            </nav>
        }
    }
}