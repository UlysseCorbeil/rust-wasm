use crate::api;
use crate::types::ModuleImageText;

use anyhow::Error;

use yew::format::Json;
use yew::prelude::*;
use yew::services::fetch::FetchTask;

struct State {
    module_image_text: Option<ModuleImageText>,
    get_module_image_text_error: Option<Error>,
    get_module_image_text_loaded: bool,
}

pub struct ModuleDetail {
    props: Props,
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub id: i32
}

pub enum Msg {
    GetModuleImageText,
    GetModuleImageTextSuccess(ModuleImageText),
    GetModuleImageTextError(Error),
}

impl Component for ModuleDetail {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::GetModuleImageText);

        Self {
            props,
            state: State {
                module_image_text: None,
                get_module_image_text_error: None,
                get_module_image_text_loaded: false
            },
            link,
            task: None,
        }
    }

    fn update(&mut self, message: Self::Message) -> ShouldRender {
        match message {

            Msg::GetModuleImageText => {
                let handler = self
                    .link
                    .callback(move |response: api::FetchResponse<ModuleImageText>| {
                        let (_, Json(data)) = response.into_parts();
                        match data {
                            Ok(module_image_text) => Msg::GetModuleImageTextSuccess(module_image_text),
                            Err(err) => Msg::GetModuleImageTextError(err),
                        }
                    });

                self.task = Some(api::get_module_detail(self.props.id, handler));
                true
            }

            Msg::GetModuleImageTextSuccess(module_image_text) => {
                self.state.module_image_text = Some(module_image_text);
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
        false
    }

    fn view(&self) -> Html {

        if let Some(ref module_image_text) = self.state.module_image_text {
            html! {
                <div class="module_image_text_detail_container">
                    <img class="module_image_text_detail_image" src={&module_image_text.image}/>
                    <div class="module_image_text_detail_name">{&module_image_text.name}</div>
                    <div class="module_image_text_detail_text">{&module_image_text.text}</div>
                </div>
            }
        } else if !self.state.get_module_image_text_loaded {
            html! {
                <div>
                    <span>{"Loading..."}</span>
                </div>
            }
        } else {
            html! {
                <div>
                    <span>{"Error loading product! :("}</span>
                </div>
            }
        }

    }
}