use super::executor::{ButtonExecutor, FaderExecutor};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonState {
    Released,
    Pressed,
}

impl From<bool> for ButtonState {
    fn from(value: bool) -> Self {
        if value {
            ButtonState::Pressed
        } else {
            ButtonState::Released
        }
    }
}

impl Into<bool> for ButtonState {
    fn into(self) -> bool {
        match self {
            ButtonState::Released => true,
            ButtonState::Pressed => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ButtonData {
    executor: ButtonExecutor,
    name: String,
    color: String,
    state: ButtonState,
}

impl ButtonData {
    pub fn new(
        executor: ButtonExecutor,
        name: impl Into<String>,
        color: impl Into<String>,
        state: impl Into<ButtonState>,
    ) -> Self {
        Self {
            executor,
            name: name.into(),
            color: color.into(),
            state: state.into(),
        }
    }
    pub fn get_executer(&self) -> &ButtonExecutor {
        &self.executor
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FaderData {
    executor: FaderExecutor,
    name: String,
    color: String,
    value: f32,
    touched: bool,
    button1: ButtonState,
    button2: ButtonState,
    button3: ButtonState,
}

impl FaderData {
    pub fn new(
        executor: FaderExecutor,
        name: impl Into<String>,
        color: impl Into<String>,
        value: f32,
        touched: bool,
        button1: impl Into<ButtonState>,
        button2: impl Into<ButtonState>,
        button3: impl Into<ButtonState>,
    ) -> Self {
        Self {
            executor,
            name: name.into(),
            color: color.into(),
            value,
            touched,
            button1: button1.into(),
            button2: button2.into(),
            button3: button3.into(),
        }
    }
    pub fn get_executer(&self) -> &FaderExecutor {
        &self.executor
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ma2Data {
    pub fader_data: Vec<FaderData>,
    pub button_data: Vec<ButtonData>,
}

impl Ma2Data {
    pub fn new(fader_data: Vec<FaderData>, button_data: Vec<ButtonData>) -> Self {
        Self {
            fader_data,
            button_data,
        }
    }

    pub fn faders(&self) -> &Vec<FaderData> {
        &self.fader_data
    }

    pub fn buttons(&self) -> &Vec<ButtonData> {
        &self.button_data
    }
}