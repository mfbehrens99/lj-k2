use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Executor {
    typ: ExecutorType,
    page: u8,
    id: u16,
}

impl Executor {
    pub fn new(page: u8, id: u16) -> Self {
        Self {
            page,
            id,
            typ: match id {
                0..=99 => ExecutorType::Fader,
                100..=199 => ExecutorType::Button,
                _ => ExecutorType::Invalid,
            },
        }
    }

    pub fn id(&self) -> u16 {
        self.id
    }

    pub fn into_button(self) -> Option<ButtonExecutor> {
        if self.typ == ExecutorType::Button {
            return Some(ButtonExecutor(self));
        }
        println!("{}", self);
        None
    }

    pub fn into_fader(self) -> Option<FaderExecutor> {
        if self.typ == ExecutorType::Fader {
            return Some(FaderExecutor(self));
        }
        println!("{}", self);
        None
    }
}

impl Display for Executor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.page, self.id)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum ExecutorType {
    Invalid,
    Button,
    Fader,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ButtonExecutor(Executor);

impl ButtonExecutor {
    pub fn new(page: u8, id: u16) -> Self {
        Executor::new(page, id).into_button().unwrap()
    }

    pub fn id(&self) -> u16 {
        self.0.id()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct FaderExecutor(Executor);

impl FaderExecutor {
    pub fn new(page: u8, id: u16) -> Self {
        Executor::new(page, id).into_fader().unwrap()
    }

    pub fn id(&self) -> u16 {
        self.0.id()
    }
}