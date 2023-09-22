use serde_derive::Serialize;

#[derive(Serialize)]
pub(crate) struct Params {
    pub state: bool,
    pub dimming: u8,
    pub sceneId: i32,
    pub r:u8,
    pub g:u8,
    pub b:u8,
    pub c:u8,
    pub w:u8
}