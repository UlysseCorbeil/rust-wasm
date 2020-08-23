use crate::route::Route;
use crate::types::ModuleImageText;
use yew::prelude::*;
use yew_router::components::RouterAnchor;

pub struct ModuleImageTextCard {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub module_image_text: ModuleImageText,
}

impl Component for ModuleImageTextCard {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        type Anchor = RouterAnchor<Route>;

        html! {
            <div class="module_image_text_card_container">
                <img class="module_image_text_image" src={&self.props.module_image_text.image}/>

                <div class="module_image_text_content_ctn">
                    <div class="module_image_text_name">{&self.props.module_image_text.name}</div>
                    <div class="module_image_text_description">{&self.props.module_image_text.description}</div>

                    <Anchor route=Route::ModuleDetail(self.props.module_image_text.id) classes="module_image_text_anchor">
                        <div class="module_image_text_button_ctn">
                            <div class="module_image_text_button"> { "Learn More" } </div>
                        </div>
                    </Anchor>

                </div>

            </div>
        }
    }
}
