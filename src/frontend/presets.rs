// use strum::IntoEnumIterator;
// use strum::EnumIter;

use enum_iterator::{Sequence, all, cardinality};

use super::data;
use super::data::Icon;

#[derive(Debug, Sequence, PartialEq, Eq, Clone, Copy)]
pub enum Preset {
    BarChill1,
    BarChill2,
    BarParty1,
    BarParty2,
    BarRave1,
    BarRave2,
    BarPutzlicht,
    BarOff,

    TresenChill,
    TresenParty,
    TresenRave,
    TresenRainbow,
    TresenPutzlicht,
    TresenOff,
}

impl Preset {
    pub fn from_row_column(row: u8, column: u8) -> Result<Preset, ()> {
        use Preset as P;
        match row {
            0 => match column {
                0 => Ok(P::BarChill1),
                1 => Ok(P::BarChill2),
                2 => Ok(P::BarParty1),
                3 => Ok(P::BarParty2),
                4 => Ok(P::BarRave1),
                5 => Ok(P::BarRave2),
                6 => Ok(P::BarPutzlicht),
                7 => Ok(P::BarOff),
                _ => Err(()),
            },
            1 => match column {
                0 => Ok(P::TresenChill),
                1 => Ok(P::TresenParty),
                2 => Ok(P::TresenRave),
                3 => Ok(P::TresenRainbow),
                4 => Ok(P::TresenPutzlicht),
                5 => Ok(P::TresenOff),
                _ => Err(()),
            },
            _ => Err(()),
        }
    }

    pub fn to_preset<'a>(self) -> data::PresetButton {
        use Preset as P;
        use data::PresetButton as PB;
        match self {
            P::BarChill1 => PB::new("Bar Chill 1", 0, 0, Icon::Chill, "#c06541"),
            P::BarChill2 => PB::new("Bar Chill 2", 0, 1, Icon::Chill, "#c06541"),
            P::BarParty1 => PB::new("Bar Party 1", 0, 2, Icon::Party, "#41c0a6"),
            P::BarParty2 => PB::new("Bar Chill 1", 0, 3, Icon::Party, "#41c0a6"),
            P::BarRave1 => PB::new("Bar Chill 1", 0, 4, Icon::Rave, "#a541d4"),
            P::BarRave2 => PB::new("Bar Chill 1", 0, 5, Icon::Rave, "#a541d4"),
            P::BarPutzlicht => PB::new("Bar Chill 1", 0, 6, Icon::Sun, "#e2d195"),
            P::BarOff => PB::new("Bar Chill 1", 0, 7, Icon::Off, "#38365a"),
            P::TresenChill => PB::new("Bar Chill 1", 1, 0, Icon::Chill, "#c06541"),
            P::TresenParty => PB::new("Bar Chill 1", 1, 1, Icon::Party, "#41c0a6"),
            P::TresenRave => PB::new("Bar Chill 1", 1, 2, Icon::Rave, "#000000"),
            P::TresenRainbow => PB::new("Bar Chill 1", 1, 3, Icon::Rainbow, "#000000"),
            P::TresenPutzlicht => PB::new("Bar Chill 1", 1, 4, Icon::Sun, "#000000"),
            P::TresenOff => PB::new("Bar Chill 1", 1, 5, Icon::Off, "#000000"),
        }
    }

    // pub fn all_presets<'a>() -> &'a [data::PresetButton<'a>] {
    //     let mut presets = Vec::with_capacity(cardinality::<Preset>());
    //     for preset in all::<Preset>() {
    //         presets.push(preset.to_preset());
    //     }
    //     let p: &[data::PresetButton<'a>] = presets.try_into().unwrap()
    // }
    
}

// let PRESET_DEFINITIONS: &[data::PresetButton] = & [
//     data::PresetButton::<'static>::new("Bar Chill 1", 0, 0, Icon::Chill, "#c06541"),
//     data::PresetButton::<'static>::new("Bar Chill 2", 0, 1, Icon::Chill, "#c06541"),
//     data::PresetButton::<'static>::new("Bar Party 1", 0, 2, Icon::Party, "#41c0a6"),
//     data::PresetButton::<'static>::new("Bar Party 2", 0, 3, Icon::Party, "#41c0a6"),
//     data::PresetButton::<'static>::new("Bar Rave 1", 0, 4, Icon::Rave, "#000000"),
//     data::PresetButton::<'static>::new("Bar Rave 2", 0, 5, Icon::Rave, "#000000"),
//     data::PresetButton::<'static>::new("Bar Putzlich", 0, 6, Icon::Sun, "#000000"),
//     data::PresetButton::<'static>::new("Bar Aus", 0, 7, Icon::Off, "#000000"),
//     data::PresetButton::<'static>::new("Tresen Chill", 1, 0, Icon::Chill, "#c06541"),
//     data::PresetButton::<'static>::new("Tresen Party", 1, 1, Icon::Party, "#41c0a6"),
//     data::PresetButton::<'static>::new("Tresen Rave", 1, 2, Icon::Rave, "#000000"),
//     data::PresetButton::<'static>::new("Tresen Rainbow", 1, 3, Icon::Rainbow, "#000000"),
//     data::PresetButton::<'static>::new("Tresen Putzlicht", 1, 4, Icon::Sun, "#000000"),
//     data::PresetButton::<'static>::new("Tresen Aus", 1, 5, Icon::Off, "#000000"),
// ];

#[cfg(test)]
mod tests {
    use super::*;

    use enum_iterator::all;

    #[test]
    fn test_preset_mapping() {
        for preset in all::<Preset>() {
            let p = preset.to_preset();
            assert_eq!(preset, Preset::from_row_column(p.row, p.column).unwrap());
        }
    }
}
