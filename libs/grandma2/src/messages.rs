use serde::{Deserialize, Serialize};

use crate::item::{MaData, MaDataResponse};

#[derive(Debug, PartialEq, Serialize)]
#[serde(untagged)]
pub enum SendMsg {
    #[serde(rename_all = "camelCase")]
    Session {
        session: i8,
    },
    Request(Request),
}

#[derive(Debug, PartialEq, Serialize)]
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

#[derive(Debug, PartialEq, Deserialize)]
#[serde(untagged)]
pub enum ReceiveMsg {
    #[serde(rename_all = "camelCase")]
    Session {
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
    ForceLogin {
        force_login: bool,
    },
    #[serde(rename_all = "camelCase")]
    Text {
        text: Box<str>,
    },
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
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
        item_groups: MaData,
    },
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use serde_json::json;

    use crate::item::MaChannel;

    use super::*;

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
                item_groups: MaData::new(vec![MaChannel::new(1, "BARS", "#00FFFF")]),
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
    }
}
