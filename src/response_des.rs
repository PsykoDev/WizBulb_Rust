use serde_derive::Deserialize;
use serde_derive::Serialize;
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub method: Option<String>,
    pub env: Option<String>,
    pub result: Result,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub mac: Option<String>,
    pub rssi: Option<i8>,
    pub src: Option<String>,
    pub state: Option<bool>,
    pub scene_id: Option<u8>,
    pub r: Option<u8>,
    pub g: Option<u8>,
    pub b: Option<u8>,
    pub c: Option<u8>,
    pub w: Option<u8>,
    pub dimming: Option<u8>,
    pub schdPsetId: Option<u8>,
    pub speed: Option<u8>,
    pub temp: Option<i32>
}

/*
sceneId - calls one of the predefined scenes (int from 1 to 35)
speed - sets the color changing speed in percent
dimming - sets the dimmer of the bulb in percent
temp - sets the color temperature in kelvins
r - red color range 0-255
g - green color range 0-255
b - blue color range 0-255
c - cold white range 0-255
w - warm white range 0-255
id - the bulb id
state - whether it's on or off
schdPsetId - rhythm id of the room
 */