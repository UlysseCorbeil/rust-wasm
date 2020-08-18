use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct ModuleImageText {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub text: String,
    pub image: String
}