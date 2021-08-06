use std::fmt::{Display, Formatter};

#[derive(PartialEq)]
pub(crate) struct Identifier {
    name: String
}

impl Identifier {
    pub(crate) fn new(name: String) -> Identifier { Identifier { name }}
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.name.fmt(f)
    }
}

impl Clone for Identifier {
    fn clone(&self) -> Self {
        Identifier::new(self.name.clone())
    }
}

