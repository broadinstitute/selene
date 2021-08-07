use crate::mion::eval::values::Value;
use crate::mion::eval::identifier::Identifier;
use std::rc::Rc;
use crate::mion::eval::expressions::Function;

pub(crate) struct Symbols {
    pub(crate) var_entries: VarEntries
}

pub(crate) enum VarEntry {
    Uninitialized,
    Value(Value)
}

pub(crate) enum VarEntries {
    Nil,
    Entry(Rc<VarEntries>, Identifier, VarEntry)
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
        println!("Initializing variable {}", identifier);
        let var_entries = self.var_entries.with_value_entry(identifier, value);
        Symbols { var_entries }
    }
    pub(crate) fn with_function_entry(self, function: Box<dyn Function>) -> Symbols {
        let var_entries = self.var_entries.with_function_entry(function);
        Symbols { var_entries }
    }
}

impl Clone for VarEntry {
    fn clone(&self) -> Self {
        match self {
            VarEntry::Uninitialized => { VarEntry::Uninitialized}
            VarEntry::Value(value) => { VarEntry::Value(value.clone()) }
        }
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
        VarEntries::Entry(Rc::new(self), identifier.clone(),
                          VarEntry::Uninitialized)
    }
    pub(crate) fn with_value_entry(self, identifier: &Identifier, value: &Value) -> VarEntries {
        VarEntries::Entry(Rc::new(self), identifier.clone(),
                          VarEntry::Value(value.clone()))
    }
    pub(crate) fn with_function_entry(self, function: Box<dyn Function>) -> VarEntries {
        let identifier = Identifier::new(String::from(function.id()));
        let value = Value::Function(Rc::new(function));
        self.with_value_entry(&identifier, &value)

    }
}

impl Clone for VarEntries {
    fn clone(&self) -> Self {
        match self {
            VarEntries::Nil => { VarEntries::Nil }
            VarEntries::Entry(var_entries, identifier, entry) => {
                VarEntries::Entry(var_entries.clone(), identifier.clone(),
                                  entry.clone())
            }
        }
    }
}

impl Clone for Symbols {
    fn clone(&self) -> Self {
        let var_entries = self.var_entries.clone();
        Symbols { var_entries }
    }
}