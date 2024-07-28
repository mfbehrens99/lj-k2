use serde::{de, Deserialize, Deserializer};
use serde_json::Value;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaChannel {
    channel: u64,
    name: String,
    color: String,
}

impl MaChannel {
    pub fn new<S>(channel: u64, name: S, color: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            channel: channel,
            name: name.into(),
            color: color.into(),
        }
    }
}

impl<'de> Deserialize<'de> for MaChannel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;

        let channel_str = v["i"]["t"]
            .as_str()
            .ok_or(de::Error::missing_field("i.t"))?;
        let channel = channel_str.parse::<u64>().unwrap();
        let name = v["tt"]["t"]
            .as_str()
            .ok_or(de::Error::missing_field("tt.t"))?
            .to_owned();
        let color = v["bdC"]
            .as_str()
            .ok_or(de::Error::missing_field("bdC"))?
            .to_owned();

        Ok(MaChannel {
            channel,
            name,
            color,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaData {
    data: Vec<MaChannel>,
}

impl MaData {
    pub fn new(data: Vec<MaChannel>) -> Self {
        Self { data: data }
    }
}

impl<'de> Deserialize<'de> for MaData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;

        let data = {
            let mut vec = Vec::<MaChannel>::new();

            let first = v.as_array().unwrap();

            for ele in first {
                let seconde = ele["items"].as_array().unwrap();

                for ele2 in seconde {
                    let third = ele2.as_array().unwrap();
                    for ele3 in third {
                        let parsed = MaChannel::deserialize(ele3).unwrap();
                        vec.push(parsed);
                    }
                }
            }

            vec
        };

        Ok(MaData::new(data))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaDataResponse {
    pub set: String,
    pub clear: String,
    pub solo: String,
    pub high: String,
}

impl<'de> Deserialize<'de> for MaDataResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let v = Value::deserialize(deserializer)?;

        let arr = v.as_array().ok_or(de::Error::missing_field("i.t"))?;

        let mut set = None;
        let mut clear = None;
        let mut solo = None;
        let mut high = None;

        for ele in arr {
            let map = ele.as_object().ok_or(de::Error::missing_field("i.t"))?;

            for (i, j) in map.iter() {
                match i as &str {
                    "set" => set = j.as_str(),
                    "clear" => clear = j.as_str(),
                    "solo" => solo = j.as_str(),
                    "high" => high = j.as_str(),
                    _ => return Err(de::Error::missing_field("i.t")),
                }
            }
        }

        Ok(MaDataResponse {
            set: set.ok_or(de::Error::missing_field("set"))?.to_owned(),
            clear: clear.ok_or(de::Error::missing_field("clear"))?.to_owned(),
            solo: solo.ok_or(de::Error::missing_field("solo"))?.to_owned(),
            high: high.ok_or(de::Error::missing_field("high"))?.to_owned(),
        })
    }
}

#[cfg(test)]
mod test {

    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_item() {
        let msg = r###"{"i":{"t":"1","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"1","c":"#FFFFFF"},"tt":{"t":"BARS","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":0,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FFFF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}}}]}"###;
        let msg_parsed: MaChannel = serde_json::from_str(msg).unwrap();
        assert_eq!(MaChannel::new(1, "BARS", "#00FFFF"), msg_parsed);

        let msg_value = Value::from_str(msg).unwrap();
        let msg_parsed: MaChannel = serde_json::from_value(msg_value).unwrap();
        assert_eq!(MaChannel::new(1, "BARS", "#00FFFF"), msg_parsed);
    }

    #[test]
    fn test_ma_data() {
        let msg = r###"[{"itemsType":2,"cntPages":10000,"items":[[{"i":{"t":"1","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"1","c":"#FFFFFF"},"tt":{"t":"BARS","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":0,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FFFF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"2","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"13","c":"#FFFFFF"},"tt":{"t":"SSALL","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":1,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF7F00","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}}}]},{"i":{"t":"3","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"7","c":"#FFFFFF"},"tt":{"t":"shitheads","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF00","cues":{"bC":"#003F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":2,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FF00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FF00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FF00","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FF00","leftLED":{},"rightLED":{}}}]},{"i":{"t":"4","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"5","c":"#FFFFFF"},"tt":{"t":"Tresen","c":"#FFFFFF"},"bC":"#000000","bdC":"#0000FF","cues":{"bC":"#00003F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":3,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#0000FF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#0000FF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#0000FF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#0000FF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"5","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"26","c":"#FFFFFF"},"tt":{"t":"PIX3L","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":4,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF7F00","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}}}]}],[{"i":{"t":"6","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"10","c":"#FFFFFF"},"tt":{"t":"STRBS","c":"#FFFFFF"},"bC":"#000000","bdC":"#FFFFFF","cues":{"bC":"#3F3F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":5,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FFFFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FFFFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FFFFFF","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FFFFFF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"7","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"29","c":"#FFFFFF"},"tt":{"t":"HPBAR","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FF7F","cues":{"bC":"#003F1F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":6,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FF7F","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FF7F","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FF7F","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FF7F","leftLED":{},"rightLED":{}}}]},{"i":{"t":"8","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"36","c":"#FFFFFF"},"tt":{"t":"pointes","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF0000","cues":{"bC":"#3F0000","items":[{"pgs":{}}]},"combinedItems":1,"iExec":7,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF0000","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF0000","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF0000","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF0000","leftLED":{},"rightLED":{}}}]},{"i":{"t":"9","c":"#FFFFFF"},"oType":{"t":"  ","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"Grand","c":"#FFFFFF"},"bC":"#E8A901","bdC":"#FF007F","cues":{"bC":"#3F001F","items":[{"t":"100%","c":"#FFFFFF","pgs":{}}]},"combinedItems":1,"iExec":8,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Black","s":false,"c":"#FFFF00","bdC":"#FF007F","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF007F","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF007F","tt":"Grand","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#FFFFFF","bdC":"#FF007F","leftLED":{},"rightLED":{}}}]},{"i":{"t":"10","c":"#000000"},"oType":{"t":"","c":"#FFFFFF"},"oI":{"t":"","c":"#FFFFFF"},"tt":{"t":"","c":"#FFFFFF"},"bC":"#404040","bdC":"#404040","cues":{},"combinedItems":1,"iExec":9,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","bdC":"#404040","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","bdC":"#404040","leftLED":{},"rightLED":{}},"fader":{"bdC":"#404040","v":0.000,"vT":"","min":0.000,"max":1.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","bdC":"#404040","leftLED":{},"rightLED":{}}}]}],[{"i":{"t":"11","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":10,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"12","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":11,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"13","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":12,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"14","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":13,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]},{"i":{"t":"15","c":"#000000"},"oType":{"t":""},"oI":{"t":""},"tt":{"t":""},"bC":"#404040","bdC":"#3D3D3D","cues":{},"combinedItems":1,"iExec":14,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}},"fader":{"v":0.000,"min":0.000,"max":0.000},"button3":{"id":2,"t":"Empty","s":false,"c":"#808080","leftLED":{},"rightLED":{}}}]}]]}]"###;
        let _msg_parsed: MaData = serde_json::from_str(msg).unwrap();

        let msg = r###"[{"itemsType":2,"cntPages":10000,"items":[[{"i":{"t":"1","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"1","c":"#FFFFFF"},"tt":{"t":"BARS","c":"#FFFFFF"},"bC":"#000000","bdC":"#00FFFF","cues":{"bC":"#003F3F","items":[{"pgs":{}}]},"combinedItems":1,"iExec":0,"isRun":0,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}},"fader":{"bdC":"#00FFFF","tt":"Mstr","v":1.000,"vT":"100%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#00FFFF","leftLED":{},"rightLED":{}}}]},{"i":{"t":"2","c":"#C0C0C0"},"oType":{"t":" P","c":"#FFFFFF"},"oI":{"t":"13","c":"#FFFFFF"},"tt":{"t":"SSALL","c":"#FFFFFF"},"bC":"#000000","bdC":"#FF7F00","cues":{"bC":"#3F1F00","items":[{"pgs":{}}]},"combinedItems":1,"iExec":1,"isRun":1,"executorBlocks":[{"button1":{"id":0,"t":"Flash","s":false,"c":"#FFFF00","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"button2":{"id":1,"t":"Black","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}},"fader":{"bdC":"#FF7F00","tt":"Mstr","v":0.000,"vT":"00%","min":0.000,"max":1.000},"button3":{"id":2,"t":"SelFix","s":false,"c":"#FFFFFF","bdC":"#FF7F00","leftLED":{},"rightLED":{}}}]}]]}]"###;
        let msg_parsed: MaData = serde_json::from_str(msg).unwrap();

        assert_eq!(
            MaData {
                data: vec![
                    MaChannel::new(1, "BARS", "#00FFFF"),
                    MaChannel::new(2, "SSALL", "#FF7F00"),
                ]
            },
            msg_parsed
        );
    }
}
