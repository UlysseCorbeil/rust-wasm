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

pub struct About {
    state: State,
    link: ComponentLink<Self>,
    task: Option<FetchTask>,
}

pub enum Msg {
    GetModuleImageText,
    GetModuleImageTextSuccess(Vec<ModuleImageText>),
    GetModuleImageTextError(Error),
}

impl Component for About {
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
        let _module_image_text: Vec<Html> = self
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
           <div class="about_container">
               <div class="about_content">
                   <div class="about_title"> { " Things I want to improve.. " }</div>
                   <ol>
                       <li> { "Implement better and more secure routing." }
                           <p> { "The routing is pretty basic and is handled is set-up inside a route file where I hard coded my paths, I would like to improve the way routing works and unit test each outcome to prevent 404s or loading errors." } </p>
                       </li>
                       <li> {" Create a small framework to generate HTML, JS, CSS and loading data in a more efficient way." }
                           <p> { " I didn't really spent much time designing an architecture that I could use in the future. My plan is to develop a framework or template around rust to set-up my environement in a way that all I have to do is develop modules and implement new functionalities.. The design would mostly work through inheritance and classic OOP logic." } </p>
                       </li>
                       <li> {" Build a more « robust » framework around component rendering. " }
                           <p> { " I mostly hard coded my components and pages, being able to use pre-made templates would save me time and speed up compiling time. It would also prevent some recursion-limit issues, if done right." } </p>
                       </li>
                   </ol>
               </div>

           </div>
        }
    }
}
