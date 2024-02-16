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
}
