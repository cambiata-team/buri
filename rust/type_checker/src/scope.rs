use crate::GenericTypeId;
use std::collections::HashMap;

pub struct Scope<'a, 'b> {
    pub parent: Option<&'a Scope<'b, 'b>>,
    pub identifiers: HashMap<String, GenericTypeId>,
}

impl<'a, 'b> Scope<'a, 'b> {
    pub fn get_declaration_type(&self, variable_name: &str) -> Option<GenericTypeId> {
        self.identifiers.get(variable_name).map_or_else(
            || {
                self.parent
                    .and_then(|parent| (*parent).get_declaration_type(variable_name))
            },
            |x| Some(*x),
        )
    }
}
