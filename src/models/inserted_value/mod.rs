/// Represent the state of a value that is about to be inserted.  
#[derive(Default, Clone, Debug)]
pub enum InsertedValue<T> {
    Null,
    #[default]
    Default,
    Some(T),
}

impl<T> InsertedValue<T> {
    pub fn is_default(&self) -> bool {
        match &self {
            Self::Default => true,
            _ => false,
        }
    }

    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Some(val) => Some(val),
            _ => None,
        }
    }
}

impl<T> From<T> for InsertedValue<T> {
    fn from(value: T) -> Self {
        Self::Some(value)
    }
}
