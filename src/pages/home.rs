use crate::api;
use crate::components::ModuleImageTextCard;
use crate::types::ModuleImageText;

use anyhow::Error;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    module_image_text: Vec<ModuleImageText>,
    get_module_image_text_error: Option<Error>,
    get_module_image_text_loaded: bool,
}

pub struct Home {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetModuleImageText,
    GetModuleImageTextSuccess(Vec<ModuleImageText>),
    GetModuleImageTextError(Error),
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let module_image_text = vec![];

        link.send_message(Msg::GetModuleImageText);

        Self {
            state: State {
                module_image_text,
                get_module_image_text_error: None,
                get_module_image_text_loaded: false,
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {
            Msg::GetModuleImageText => {
                self.state.get_module_image_text_loaded = false;

                let handler = self.link.callback(
                    move |response: api::FetchResponse<Vec<ModuleImageText>>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(module_image_text) => {
                                Msg::GetModuleImageTextSuccess(module_image_text)
                            }
                            Err(err) => Msg::GetModuleImageTextError(err),
                        }
                    },
                );

                self.task = Some(api::get_module_image_text(handler));
                true
            }

            Msg::GetModuleImageTextSuccess(module_image_text) => {
                self.state.module_image_text = module_image_text;
                self.state.get_module_image_text_loaded = true;
                true
            }

            Msg::GetModuleImageTextError(error) => {
                self.state.get_module_image_text_error = Some(error);
                self.state.get_module_image_text_loaded = true;
                true
            }
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let module_image_text: Vec<Html> = self
            .state
            .module_image_text
            .iter()
            .map(|module_image_text: &ModuleImageText| {
                html! {
                    <ModuleImageTextCard module_image_text = { module_image_text } />
                }
            })
            .collect();

        html! {

            <div class="root">

                <div class="intro_container">
                    <img src="/images/drawkit-full-stack.png"/>

                    <div class="intro_content">
                        <div class="intro_title"> { "Welcome to my Rust & WASM website !" } </div>
                        <div class="intro_text"> { "I made this website out of curiosity, I was curious to see if I could mix Wasm (Web Assembly) with Rust and use that combination to create a website with as little js as possible.. I also used the 'Yew' framework to create re-usable components, for fetching content on JSON files, routing, etc.. Since I was already familiar with React, 'Yew' felt like the appropriate option. \n \n (this version is still pretty rough around the edges, but I had some fun and learned quite a bit)"  } </div>
                    </div>
                </div>

                <div class="module_image_text_card_list">
                    { module_image_text }
                </div>

            </div>
        }
    }
}
