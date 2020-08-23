use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Footer, Nav};
use crate::pages::{About, Home, ModuleDetail};
use crate::route::Route;

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let render = Router::render(move |switch: Route| match switch {
            Route::HomePage => {
                html! { <Home /> }
            }
            Route::About => {
                html! { <About /> }
            }
            Route::ModuleDetail(id) => {
                html! { <ModuleDetail id=id /> }
            }
        });

        html! {
            <>
                <Nav />
                <Router<Route, ()> render=render />
                <Footer />
            </>
        }
    }
}
