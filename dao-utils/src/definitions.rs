use casper_types::{CLType, CLTyped};
use serde::Serialize;

pub trait ContractDefinition {
    fn contract_def() -> ContractDef;
}

pub trait EventDefinition {
    fn event_def() -> EventDef;
}

#[derive(Debug, Clone, Serialize)]
pub struct ContractDef {
    pub name: &'static str,
    pub entry_points: Vec<MethodDef>,
}

impl ContractDef {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            entry_points: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: MethodDef) {
        self.entry_points.push(method);
    }

    pub fn add_event<T: EventDefinition>(&mut self, method_name: &'static str) {
        if let Some(method) = self.method_mut(method_name) {
            method.add_event(T::event_def());
        }
    }

    pub fn with_event<T: EventDefinition>(mut self, method_name: &'static str) -> Self {
        self.add_event::<T>(method_name);
        self
    }

    pub fn mutable_methods(&self) -> Vec<MethodDef> {
        self.entry_points
            .clone()
            .into_iter()
            .filter(|m| m.is_mutable)
            .collect()
    }

    fn method_mut(&mut self, method_name: &'static str) -> Option<&mut MethodDef> {
        self.entry_points
            .iter_mut()
            .find(|method| method.name == method_name)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct MethodDef {
    pub name: &'static str,
    pub is_mutable: bool,
    pub args: Vec<ElemDef>,
    pub return_ty: CLType,
    pub events: Vec<EventDef>,
}

impl MethodDef {
    pub fn new<T: CLTyped>(name: &'static str, is_mutable: bool) -> Self {
        MethodDef {
            name,
            is_mutable,
            args: Vec::new(),
            return_ty: T::cl_type(),
            events: Vec::new(),
        }
    }

    pub fn add_arg(&mut self, arg: ElemDef) {
        self.args.push(arg);
    }

    pub fn add_event(&mut self, event: EventDef) {
        self.events.push(event);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EventDef {
    pub name: &'static str,
    pub fields: Vec<ElemDef>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ElemDef {
    pub name: &'static str,
    pub ty: CLType,
}

impl ElemDef {
    pub fn new<T: CLTyped>(name: &'static str) -> Self {
        ElemDef {
            name,
            ty: T::cl_type(),
        }
    }
}

impl EventDef {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, elem: ElemDef) -> Self {
        self.fields.push(elem);
        self
    }
}
