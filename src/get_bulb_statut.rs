use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root{
    pub method: String,
    pub env: String,
    pub result: Result,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result{
    pub mac: String,
    pub rssi: i32,
    pub src: String,
    pub state: bool,
    pub sceneId: i32,
    pub speed: u8,
    pub dimming: u8
}

 // {"method":"getPilot","env":"pro","result":{"mac":"6c2990df22da","rssi":-63,"src":"","state":true,"sceneId":2,"speed":100,"dimming":25}}