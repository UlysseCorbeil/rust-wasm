use crate::types::ModuleImageText;
use anyhow::Error;
use yew::callback::Callback;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

pub type FetchResponse<T> = Response<Json<Result<T, Error>>>;
type FetchCallback<T> = Callback<FetchResponse<T>>;

pub fn get_module_image_text(callback: FetchCallback<Vec<ModuleImageText>>) -> FetchTask {
    let req = Request::get("/json/ModuleImageText.json")
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}

pub fn get_module_detail(id: i32, callback: FetchCallback<ModuleImageText>) -> FetchTask {
    let req = Request::get(format!("/json/{}.json", id))
        .body(Nothing)
        .unwrap();

    FetchService::fetch(req, callback).unwrap()
}
