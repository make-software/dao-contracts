//! Data structures describing contracts and events metadata.
use casper_event_standard::Schemas;
use casper_types::{CLType, CLTyped};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Smart contract metadata. Should be implemented by every contract.
pub trait ContractDefinition {
    fn contract_def() -> ContractDef;
}

/// Represents a contract definition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ContractDef {
    pub name: String,
    pub entry_points: Vec<MethodDef>,
    pub events: Vec<EventDef>,
}

impl ContractDef {
    pub fn new(name: String) -> Self {
        Self {
            name,
            entry_points: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: MethodDef) {
        self.entry_points.push(method);
    }

    pub fn add_events(&mut self, schemas: Schemas) {
        for (name, schema) in schemas.0 {
            let mut event_def = EventDef::new(name);
            for (name, ty) in schema.to_vec() {
                event_def.add(ElemDef::new_with_ty(name, ty.downcast()));
            }
            self.events.push(event_def);
        }
    }

    pub fn with_events(mut self, schemas: Schemas) -> Self {
        self.add_events(schemas);
        self
    }
}

/// Represents contract entry point definition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MethodDef {
    pub name: String,
    pub is_mutable: bool,
    pub args: Vec<ElemDef>,
    pub return_ty: CLType,
}

impl MethodDef {
    pub fn new<T: CLTyped>(name: String, is_mutable: bool) -> Self {
        MethodDef {
            name,
            is_mutable,
            args: Vec::new(),
            return_ty: T::cl_type(),
        }
    }

    pub fn add_arg(&mut self, arg: ElemDef) {
        self.args.push(arg);
    }
}

/// Represents an event definition.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventDef {
    pub name: String,
    pub fields: Vec<ElemDef>,
}

impl EventDef {
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn add(&mut self, elem: ElemDef) {
        self.fields.push(elem);
    }

    pub fn with_field(mut self, elem: ElemDef) -> Self {
        self.add(elem);
        self
    }
}

/// Represents a single event field.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ElemDef {
    pub name: String,
    pub ty: CLType,
}

impl ElemDef {
    pub fn new<T: CLTyped>(name: String) -> Self {
        Self::new_with_ty(name, T::cl_type())
    }

    pub fn new_with_ty(name: String, ty: CLType) -> Self {
        ElemDef { name, ty }
    }
}
