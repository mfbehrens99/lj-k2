use serde::{Deserialize, Serialize};

use crate::types::Ma2Data;

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SendMsg {
    #[serde(rename_all = "camelCase")]
    Session {
        session: i8,
    },
    Request(Request),
}

#[derive(Debug, Clone, PartialEq, Serialize)]
#[serde(tag = "requestType")]
#[serde(rename_all = "lowercase")]
pub enum Request {
    #[serde(rename_all = "camelCase")]
    GetData {
        data: String,
        max_requests: u16,
        session: i8,
    },
    #[serde(rename_all = "camelCase")]
    Login {
        username: String,
        password: String,
        max_requests: u16,
        session: i8,
    },
    #[serde(rename_all = "camelCase")]
    Playbacks {
        start_index: Vec<u16>,
        items_count: Vec<u16>,
        page_index: u8,
        items_type: Vec<u8>,
        view: u8,
        exec_button_view_mode: u8,
        buttons_view_mode: u8,
        max_requests: u16,
        session: i8,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "playbacks_userInput")]
    ButtonInput {
        exec_index: u8,
        page_index: u8,
        button_id: u16,
        #[serde(rename = "type")]
        input_type: u8,
        pressed: bool,
        released: bool,
        max_requests: u16,
        session: i8,
    },
    #[serde(rename_all = "camelCase")]
    #[serde(rename = "playbacks_userInput")]
    FaderInput {
        exec_index: u8,
        page_index: u8,
        fader_value: f32,
        #[serde(rename = "type")]
        input_type: u8,
        max_requests: u16,
        session: i8,
    },
    #[serde(rename_all = "camelCase")]
    Close { session: i8, max_requests: u16 },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ReceiveMsg {
    #[serde(rename_all = "camelCase")]
    Session {
        force_login: Option<bool>,
        realtime: bool,
        world_index: u8,
        session: i8,
    },
    Response(Response),
    #[serde(rename_all = "camelCase")]
    Status {
        status: Box<str>,
        app_type: Box<str>,
    },
    #[serde(rename_all = "camelCase")]
    Text {
        text: Box<str>,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(tag = "responseType")]
#[serde(rename_all = "lowercase")]
pub enum Response {
    #[serde(rename_all = "camelCase")]
    GetData {
        realtime: bool,
        world_index: u8,
        data: MaDataResponse,
    },
    #[serde(rename_all = "camelCase")]
    Login {
        realtime: bool,
        world_index: u8,
        result: bool,
    },
    #[serde(rename_all = "camelCase")]
    Playbacks {
        realtime: bool,
        world_index: u8,
        response_sub_type: u8,
        i_page: u8,
        item_groups: Ma2Data,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaDataResponse {
    pub set: String,
    pub clear: String,
    pub solo: String,
    pub high: String,
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use serde_json::{json, Value};

    use super::*;
    use crate::{
        types::FaderData,
        types::{ButtonExecutor, Executor, FaderExecutor},
    };

    #[test]
    fn test_send_msg() {
        let msg = SendMsg::Session { session: 26 };
        let msg_json = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            Value::from_str(&msg_json).unwrap(),
            json!({
                "session": 26,
            })
        );

        let msg = SendMsg::Request(Request::GetData {
            data: "set,clear,solo,high".to_owned(),
            max_requests: 5,
            session: 14,
        });
        let msg_json = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            Value::from_str(&msg_json).unwrap(),
            json!({
                "requestType": "getdata",
                "session": 14,
                "data": "set,clear,solo,high",
                "maxRequests": 5,
            })
        );

        let msg = SendMsg::Request(Request::Login {
            username: "admin".to_owned(),
            password: "admin".to_owned(),
            max_requests: 10,
            session: 63,
        });
        let msg_json = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            Value::from_str(&msg_json).unwrap(),
            json!({
                "requestType": "login",
                "session": 63,
                "username": "admin",
                "password": "admin",
                "maxRequests": 10,
            })
        );

        let msg = SendMsg::Request(Request::ButtonInput {
            exec_index: 136,
            page_index: 0,
            button_id: 0,
            input_type: 0,
            pressed: false,
            released: true,
            max_requests: 0,
            session: 5,
        });
        let msg_json = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            Value::from_str(&msg_json).unwrap(),
            json!({
                "requestType":"playbacks_userInput",
                "execIndex":136,
                "pageIndex":0,
                "buttonId":0,
                "pressed":false,
                "released":true,
                "type":0,
                "session":5,
                "maxRequests":0,
            })
        );

        let msg = SendMsg::Request(Request::FaderInput {
            exec_index: 6,
            page_index: 0,
            fader_value: 0.62068963,
            input_type: 1,
            max_requests: 0,
            session: 6,
        });
        let msg_json = serde_json::to_string(&msg).unwrap();

        assert_eq!(
            Value::from_str(&msg_json).unwrap(),
            json!({
                "requestType":"playbacks_userInput",
                "execIndex":6,
                "pageIndex":0,
                "faderValue":0.62068963,
                "type":1,
                "session":6,
                "maxRequests":0,
            })
        );
    }

    #[test]
    fn test_receive_msg() {
        let msg = r#"{"realtime":false,"session":3,"worldIndex":0}"#;
        let msg_parsed: ReceiveMsg = serde_json::from_str(msg).unwrap();
        assert_eq!(
            ReceiveMsg::Session {
                force_login: None,
                realtime: false,
                world_index: 0,
                session: 3
            },
            msg_parsed
        );

        let msg = r###"{"realtime":false,"responseType":"playbacks","responseSubType":2,"iPage":1,"itemGroups":[{"itemsType":2,"cntPages":10000,"items":[[{"i":{"t":"1","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"1","c":"#FFFFFF"},"tt":{"t":"BARS","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":0,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FFFF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"2","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"13","c":"#FFFFFF"},"tt":{"t":"SSALL","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":1,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF7F00","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}}}]},{"i":{"t":"3","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"7","c":"#FFFFFF"},"tt":{"t":"shitheads","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF00","cues":{"bC":"#003F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":2,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FF00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FF00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FF00","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FF00","leftLED":{},"rightLED":{}}}]},{"i":{"t":"4","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"5","c":"#FFFFFF"},"tt":{"t":"Tresen","c":"#FFFFFF"},"bC":"#000000","bdC":"#0000FF","cues":{"bC":"#00003F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":3,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#0000FF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#0000FF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#0000FF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#0000FF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"5","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"26","c":"#FFFFFF"},"tt":{"t":"PIX3L","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":4,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF7F00","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}}}]}],[{"i":{"t":"6","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"10","c":"#FFFFFF"},"tt":{"t":"STRBS","c":"#FFFFFF"},"bC":"#000000","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":5,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FFFFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FFFFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FFFFFF","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FFFFFF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"7","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"29","c":"#FFFFFF"},"tt":{"t":"HPBAR","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF7F","cues":{"bC":"#003F1F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":6,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FF7F","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FF7F","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FF7F","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FF7F","leftLED":{},"rightLED":{}}}]},{"i":{"t":"8","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"36","c":"#FFFFFF"},"tt":{"t":"pointes","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF0000","cues":{"bC":"#3F0000","items":[{"pgs":{}}]},"combinedItems":1,"iExec":7,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF0000","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF0000","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF0000","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF0000","leftLED":{},"rightLED":{}}}]},{"i":{"t":"9","c":"#FFFFFF"},"oType":{"t":"  ","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"Grand","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF007F","cues":{"bC":"#3F001F","items":[{"t":"100%","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":8,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Black","s":false,"c":"#FFFF00","bdC":"#FF007F","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF007F","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF007F","tt":"Grand","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#FFFFFF","bdC":"#FF007F","leftLED":{},"rightLED":{}}}]},{"i":{"t":"10","c":"#000000"},"oType":{"t":"","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"","c":"#FFFFFF"},"bC":"#404040","bdC":"#404040","cues":{},"combinedItems":1,"iExec":9,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","bdC":"#404040","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","bdC":"#404040","leftLED":{},"rightLED":{}},"fader":{"bdC":"#404040","v":0.000,"vT":"","min":0.000,"max":1.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","bdC":"#404040","leftLED":{},"rightLED":{}}}]}],[{"i":{"t":"11","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":10,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"12","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":11,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"13","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":12,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"14","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":13,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"15","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":14,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]}]]}],"worldIndex":0}"###;
        let _msg_parsed: ReceiveMsg = serde_json::from_str(msg).unwrap();

        let msg = r###"{"realtime":false,"responseType":"playbacks","responseSubType":2,"iPage":1,"itemGroups":[{"itemsType":2,"cntPages":10000,"items":[[{"i":{"t":"1","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"1","c":"#FFFFFF"},"tt":{"t":"BARS","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":0,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FFFF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}}}]}]]}],"worldIndex": 0}"###;
        let msg_parsed: ReceiveMsg = serde_json::from_str(msg).unwrap();
        assert_eq!(
            ReceiveMsg::Response(Response::Playbacks {
                realtime: false,
                world_index: 0,
                response_sub_type: 2,
                i_page: 1,
                item_groups: Ma2Data::new(
                    vec![FaderData::new(
                        Executor::new(1, 1).into_fader().unwrap(),
                        "BARS",
                        "#00FFFF",
                        1.0,
                        false,
                        false,
                        false,
                        false
                    )],
                    Vec::new()
                ),
            }),
            msg_parsed
        );

        let msg = r##"{"realtime":false,"responseType":"getdata","data":[{"set":"1"},{"clear":"1"},{"solo":"0"},{"high":"0"}],"worldIndex":0}"##;
        let msg_parsed: ReceiveMsg = serde_json::from_str(msg).unwrap();
        let vector = MaDataResponse {
            set: "1".to_owned(),
            clear: "1".to_owned(),
            solo: "0".to_owned(),
            high: "0".to_owned(),
        };
        assert_eq!(
            ReceiveMsg::Response(Response::GetData {
                realtime: false,
                world_index: 0,
                data: vector,
            }),
            msg_parsed
        );

        let msg = r#"{"realtime":false,"responseType":"login","result":true,"worldIndex":0}"#;
        let msg_parsed: ReceiveMsg = serde_json::from_str(msg).unwrap();
        assert_eq!(
            ReceiveMsg::Response(Response::Login {
                realtime: false,
                world_index: 0,
                result: true
            }),
            msg_parsed
        );

        let msg = r##"{"realtime":false,"responseType":"playbacks","responseSubType":2,"iPage":1,"itemGroups":[{"itemsType":3,"iExecOff":100,"cntPages":10000,"items":[[{"i":{"t":"1","c":"#FFFFFF"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"1","c":"#FFFFFF"},"tt":{"t":"Edit BARS","c":"#FFFFFF"},"bC":"#800000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":0,"isRun":0},{"i":{"t":"2","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"13","c":"#FFFFFF"},"tt":{"t":"SSALL","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":1,"isRun":1},{"i":{"t":"3","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"7","c":"#FFFFFF"},"tt":{"t":"shitheads","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF00","cues":{"bC":"#003F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":2,"isRun":1},{"i":{"t":"4","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"5","c":"#FFFFFF"},"tt":{"t":"Tresen","c":"#FFFFFF"},"bC":"#000000","bdC":"#0000FF","cues":{"bC":"#00003F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":3,"isRun":0},{"i":{"t":"5","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"26","c":"#FFFFFF"},"tt":{"t":"PIX3L","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":4,"isRun":1}],[{"i":{"t":"6","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"10","c":"#FFFFFF"},"tt":{"t":"STRBS","c":"#FFFFFF"},"bC":"#000000","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":5,"isRun":1},{"i":{"t":"7","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"29","c":"#FFFFFF"},"tt":{"t":"HPBAR","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF7F","cues":{"bC":"#003F1F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":6,"isRun":1},{"i":{"t":"8","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"36","c":"#FFFFFF"},"tt":{"t":"pointes","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF0000","cues":{"bC":"#3F0000","items":[{"pgs":{}}]},"combinedItems":1,"iExec":7,"isRun":1},{"i":{"t":"9","c":"#FFFFFF"},"oType":{"t":"  ","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"Grand","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF007F","cues":{"bC":"#3F001F","items":[{"t":"98%","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":8,"isRun":0},{"i":{"t":"10","c":"#000000"},"oType":{"t":"","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"","c":"#FFFFFF"},"bC":"#404040","bdC":"#404040","cues":{},"combinedItems":1,"iExec":9,"isRun":0}],[{"i":{"t":"11","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":10,"isRun":0},{"i":{"t":"12","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":11,"isRun":0},{"i":{"t":"13","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":12,"isRun":0},{"i":{"t":"14","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":13,"isRun":0},{"i":{"t":"15","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":14,"isRun":0}],[{"i":{"t":"16","c":"#FFFFFF"},"oType":{"t":"Sp","c":"#FFFFFF"},"oI":{"t":"3.1","c":"#FFFFFF"},"tt":{"t":"Spd 1","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF0000","cues":{"bC":"#3F0000","items":[{"t":"80.6 BPM","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":15,"isRun":1},{"i":{"t":"17","c":"#FFFFFF"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.8","c":"#FFFFFF"},"tt":{"t":"RESOLUME","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#0000FF","cues":{"bC":"#00003F","items":[{"t":"1:1","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":16,"isRun":0},{"i":{"t":"18","c":"#FFFFFF"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.9","c":"#FFFFFF"},"tt":{"t":"ABLETON","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF007F","cues":{"bC":"#3F001F","items":[{"t":"1:1","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":17,"isRun":0},{"i":{"t":"19","c":"#FFFFFF"},"oType":{"t":"Pl","c":"#FFFFFF"},"oI":{"t":"5.1","c":"#FFFFFF"},"tt":{"t":"STEP BARS","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#C0C0C0","cues":{"bC":"#303030","items":[{"t":"0","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":18,"isRun":0},{"i":{"t":"20","c":"#FFFFFF"},"oType":{"t":"Pl","c":"#FFFFFF"},"oI":{"t":"5.2","c":"#FFFFFF"},"tt":{"t":"STEP SH","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#C0C0C0","cues":{"bC":"#303030","items":[{"t":"0","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":19,"isRun":0}],[{"i":{"t":"21","c":"#FFFFFF"},"oType":{"t":"Pl","c":"#FFFFFF"},"oI":{"t":"5.3","c":"#FFFFFF"},"tt":{"t":"STEP TRESEN","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#0000FF","cues":{"bC":"#00003F","items":[{"t":"0","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":20,"isRun":0},{"i":{"t":"22","c":"#FFFFFF"},"oType":{"t":"Pl","c":"#FFFFFF"},"oI":{"t":"5.4","c":"#FFFFFF"},"tt":{"t":"STEP SS","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"t":"0","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":21,"isRun":0},{"i":{"t":"23","c":"#FFFFFF"},"oType":{"t":"LT","c":"#FFFFFF"},"oI":{"t":"13","c":"#FFFFFF"},"tt":{"t":"BARS FULL","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"t":"    1 Cue","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":22,"isRun":0},{"i":{"t":"24","c":"#FFFFFF"},"oType":{"t":"LT","c":"#FFFFFF"},"oI":{"t":"11","c":"#FFFFFF"},"tt":{"t":"SH FULL","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#00FF00","cues":{"bC":"#003F00","items":[{"t":"    1 Cue","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":23,"isRun":0},{"i":{"t":"25","c":"#FFFFFF"},"oType":{"t":"LT","c":"#FFFFFF"},"oI":{"t":"18","c":"#FFFFFF"},"tt":{"t":"SS BCK","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"t":"    1 Cue","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":24,"isRun":0}],[{"i":{"t":"26","c":"#FFFFFF"},"oType":{"t":"LT","c":"#FFFFFF"},"oI":{"t":"20","c":"#FFFFFF"},"tt":{"t":"STROBE","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"t":"    1 Cue","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":25,"isRun":0},{"i":{"t":"27","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":26,"isRun":0},{"i":{"t":"28","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":27,"isRun":0},{"i":{"t":"29","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":28,"isRun":0},{"i":{"t":"30","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":29,"isRun":0}],[{"i":{"t":"31","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.1","c":"#FFFFFF"},"tt":{"t":"BARS","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"t":"2.00","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":30,"isRun":1},{"i":{"t":"32","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.6","c":"#FFFFFF"},"tt":{"t":"SS","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"t":"1.00","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":31,"isRun":0},{"i":{"t":"33","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.5","c":"#FFFFFF"},"tt":{"t":"shitheads","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF00","cues":{"bC":"#003F00","items":[{"t":"1.00","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":32,"isRun":0},{"i":{"t":"34","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.4","c":"#FFFFFF"},"tt":{"t":"MOV","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF00","cues":{"bC":"#003F00","items":[{"t":"1:1","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":33,"isRun":0},{"i":{"t":"35","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.10","c":"#FFFFFF"},"tt":{"t":" PIX3L","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"t":"1.00","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":34,"isRun":0}],[{"i":{"t":"36","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.7","c":"#FFFFFF"},"tt":{"t":"STROBE","c":"#FFFFFF"},"bC":"#000000","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"t":"1:1","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":35,"isRun":0},{"i":{"t":"37","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.13","c":"#FFFFFF"},"tt":{"t":"PMOV","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF0000","cues":{"bC":"#3F0000","items":[{"t":"1:1","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":36,"isRun":0},{"i":{"t":"38","c":"#C0C0C0"},"oType":{"t":"Ra","c":"#FFFFFF"},"oI":{"t":"4.12","c":"#FFFFFF"},"tt":{"t":"POINTES","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF0000","cues":{"bC":"#3F0000","items":[{"t":"1:1","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":37,"isRun":0},{"i":{"t":"39","c":"#000000"},"oType":{"t":"","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"","c":"#FFFFFF"},"bC":"#404040","bdC":"#404040","cues":{},"combinedItems":1,"iExec":38,"isRun":0},{"i":{"t":"40","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":39,"isRun":0}],[{"i":{"t":"41","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":40,"isRun":0},{"i":{"t":"42","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":41,"isRun":0},{"i":{"t":"43","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":42,"isRun":0},{"i":{"t":"44","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":43,"isRun":0},{"i":{"t":"45","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":44,"isRun":0}],[{"i":{"t":"46","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":45,"isRun":0},{"i":{"t":"47","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":46,"isRun":0},{"i":{"t":"48","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":47,"isRun":0},{"i":{"t":"49","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":48,"isRun":0},{"i":{"t":"50","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":49,"isRun":0}],[{"i":{"t":"51","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":50,"isRun":0},{"i":{"t":"52","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":51,"isRun":0},{"i":{"t":"53","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":52,"isRun":0},{"i":{"t":"54","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":53,"isRun":0},{"i":{"t":"55","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":54,"isRun":0}],[{"i":{"t":"56","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":55,"isRun":0},{"i":{"t":"57","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":56,"isRun":0},{"i":{"t":"58","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":57,"isRun":0},{"i":{"t":"59","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":58,"isRun":0},{"i":{"t":"60","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":59,"isRun":0}],[{"i":{"t":"61","c":"#FFFFFF"},"oType":{"t":"  LAS","c":"#FFFFFF"},"oI":{"t":"60","c":"#FFFFFF"},"tt":{"t":"STRB_EO","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"t":"80.6 BPM","c":"#FFFFFF","pgs":{"v":0.358,"bC":"#808080"}},{"t":"0.0 s","c":"#FFFFFF","pgs":{"v":1.000,"bC":"#808080"}},{"pgs":{"bC":"#808080"}}]},"combinedItems":1,"iExec":60,"isRun":0},{"i":{"t":"62","c":"#FFFFFF"},"oType":{"t":"  LAS","c":"#FFFFFF"},"oI":{"t":"61","c":"#FFFFFF"},"tt":{"t":"STRB .3","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"t":"80.6 BPM","c":"#FFFFFF","pgs":{"v":0.358,"bC":"#808080"}},{"t":"0.0 s","c":"#FFFFFF","pgs":{"v":1.000,"bC":"#808080"}},{"pgs":{"bC":"#808080"}}]},"combinedItems":1,"iExec":61,"isRun":0},{"i":{"t":"63","c":"#FFFFFF"},"oType":{"t":"HT","c":"#FFFFFF"},"oI":{"t":"22","c":"#FFFFFF"},"tt":{"t":"PIX3L FULL","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"t":"    1 Cue","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":62,"isRun":0},{"i":{"t":"64","c":"#FFFFFF"},"oType":{"t":"HT","c":"#FFFFFF"},"oI":{"t":"19","c":"#FFFFFF"},"tt":{"t":"SS FULL","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"t":"    1 Cue","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":63,"isRun":0},{"i":{"t":"65","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":64,"isRun":0}],[{"i":{"t":"66","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":65,"isRun":0},{"i":{"t":"67","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":66,"isRun":0},{"i":{"t":"68","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":67,"isRun":0},{"i":{"t":"69","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":68,"isRun":0},{"i":{"t":"70","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":69,"isRun":0}],[{"i":{"t":"71","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":70,"isRun":0},{"i":{"t":"72","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":71,"isRun":0},{"i":{"t":"73","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":72,"isRun":0},{"i":{"t":"74","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":73,"isRun":0},{"i":{"t":"75","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":74,"isRun":0}],[{"i":{"t":"76","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":75,"isRun":0},{"i":{"t":"77","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":76,"isRun":0},{"i":{"t":"78","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":77,"isRun":0},{"i":{"t":"79","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":78,"isRun":0},{"i":{"t":"80","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":79,"isRun":0}],[{"i":{"t":"81","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":80,"isRun":0},{"i":{"t":"82","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":81,"isRun":0},{"i":{"t":"83","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":82,"isRun":0},{"i":{"t":"84","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":83,"isRun":0},{"i":{"t":"85","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":84,"isRun":0}],[{"i":{"t":"86","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":85,"isRun":0},{"i":{"t":"87","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":86,"isRun":0},{"i":{"t":"88","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":87,"isRun":0},{"i":{"t":"89","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":88,"isRun":0},{"i":{"t":"90","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":89,"isRun":0}]]}],"worldIndex":0}"##;
    }
}
