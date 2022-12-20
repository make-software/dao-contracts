use core::panic;

use casper_types::{CLType, CLTyped};

pub trait ContractDefinition {
    fn contract_def() -> ContractDef;
}

pub trait EventDefinition {
    fn event_def() -> EventDef;
}

#[derive(Debug, Clone)]
pub struct ContractDef {
    pub ident: &'static str,
    pub methods: Vec<MethodDef>,
}

impl ContractDef {
    pub fn new(ident: &'static str) -> Self {
        Self {
            ident,
            methods: Vec::new(),
        }
    }

    pub fn add_method(&mut self, method: MethodDef) {
        self.methods.push(method);
    }

    pub fn add_event<T: EventDefinition>(&mut self, method_ident: &'static str) {
        self.method_mut(method_ident).map(|method| {
            method.add_event(T::event_def());
        });
    }

    pub fn mutable_methods(&self) -> Vec<MethodDef> {
        self.methods
            .clone()
            .into_iter()
            .filter(|m| m.is_mutable)
            .collect()
    }

    fn method_mut(&mut self, method_ident: &'static str) -> Option<&mut MethodDef> {
        for method in &mut self.methods {
            if method.ident == method_ident {
                return Some(method);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct MethodDef {
    pub ident: &'static str,
    pub is_mutable: bool,
    pub args: Vec<ElemDef>,
    pub return_ty: CLType,
    pub events: Vec<EventDef>,
}

impl MethodDef {
    pub fn new<T: CLTyped>(ident: &'static str, is_mutable: bool) -> Self {
        MethodDef {
            ident,
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

#[derive(Debug, Clone)]
pub struct EventDef {
    pub ident: &'static str,
    pub fields: Vec<ElemDef>,
}

#[derive(Debug, Clone)]
pub struct ElemDef {
    pub ident: &'static str,
    pub ty: CLType,
}

impl ElemDef {
    pub fn new<T: CLTyped>(ident: &'static str) -> Self {
        ElemDef {
            ident,
            ty: T::cl_type(),
        }
    }
}

impl EventDef {
    pub fn new(ident: &'static str) -> Self {
        Self {
            ident,
            fields: Vec::new(),
        }
    }

    pub fn with_field(mut self, elem: ElemDef) -> Self {
        self.fields.push(elem);
        self
    }
}
