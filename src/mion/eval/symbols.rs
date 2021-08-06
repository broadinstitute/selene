use crate::mion::eval::eval::Expression;
use crate::util::error::Error;
use crate::mion::eval::values::Value;
use crate::mion::eval::identifier::Identifier;

pub(crate) struct Symbols {
    pub(crate) var_entries: VarEntries
}

pub(crate) enum VarEntry {
    Uninitialized,
    Value(Value)
}

pub(crate) enum VarEntries {
    Nil,
    Entry(Box<VarEntries>, Identifier, VarEntry)
}

impl Symbols {
    pub(crate) fn new() -> Symbols {
        let var_entries = VarEntries::new();
        Symbols { var_entries }
    }
    pub(crate) fn with_var_uninitialized_entry(self, identifier: &Identifier) -> Symbols {
        let var_entries = self.var_entries.with_uninitialized_entry(identifier);
        Symbols { var_entries }
    }
    pub(crate) fn with_var_value_entry(self, identifier: &Identifier, value: &Value) -> Symbols {
        let var_entries = self.var_entries.with_value_entry(identifier, value);
        Symbols { var_entries }
    }
}

impl VarEntries {
    pub(crate) fn new() -> VarEntries { VarEntries::Nil }
    pub(crate) fn get(&self, identifier: &Identifier) -> Option<&VarEntry> {
        match self {
            VarEntries::Nil => { None }
            VarEntries::Entry(parent, entry_identifier, entry) => {
                if identifier == entry_identifier {
                    Some(entry)
                } else {
                    parent.get(identifier)
                }
            }
        }
    }
    pub(crate) fn with_uninitialized_entry(self, identifier: &Identifier) -> VarEntries {
        VarEntries::Entry(Box::new(self), identifier.clone(),
                          VarEntry::Uninitialized)
    }
    pub(crate) fn with_value_entry(self, identifier: &Identifier, value: &Value) -> VarEntries {
        VarEntries::Entry(Box::new(self), identifier.clone(),
                          VarEntry::Value(value.clone()))
    }
}

