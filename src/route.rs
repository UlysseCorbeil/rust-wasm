use yew_router::prelude::*;

#[derive(Switch, Debug, Clone)]
pub enum Route {
    #[to = "/module/{id}"]
    ModuleDetail(i32),
    #[to = "/about"]
    About,
    #[to = "/"]
    HomePage,
}
