#![allow(unused)]

use egui::{
    self,
    *,

};

use crate::egui_ext::{self, FormWidget};

pub enum ProjectError {
    AlreadyExists(String),
    InvalidPermission(String),
    Unknown(String),
}

impl ProjectError {
    pub fn permission(message: String) -> Self {
        Self::InvalidPermission(message)
    }

    pub fn unknown(message: String) -> Self {
        Self::Unknown(message)
    }

    pub fn already_exists(message: String) -> Self {
        Self::AlreadyExists(message)
    }
}

pub trait TemplateForm {
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn update(&mut self, ui: &mut Ui);
    fn generate(&self) -> Result<(), ProjectError>;
    fn reset(&mut self);
}

pub struct TemplateData<'a> {
    pub forms: Vec<Box<dyn TemplateForm + 'a>>,
}

impl<'a> TemplateData<'a> {
    fn new() -> Self {
        Self {
            forms: Vec::new(),
        }
    }

    pub fn add<T: TemplateForm + 'a>(&mut self, data: T) {
        self.forms.push(Box::new(data));
    }

    pub fn load() -> Self {
        let mut data = Self::new();
        data.add(ExampleTemplate::new());
        data

    }
}

pub struct ExampleTemplate {
    form: egui_ext::Form,
}

impl ExampleTemplate {
    fn new() -> Self {
        use egui_ext::*;
        Self {
            form: Form::with(vec![
            FormItem::new("item1", "Boolean", FormData::Boolean(false)),
            FormItem::new("item2", "Integer", FormData::Integer(ScalarData::new(0))),
            FormItem::new("Unsigned", "Unsigned", FormData::Unsigned(ScalarData::new(0))),
            FormItem::new("item2", "Choices", FormData::Choice(Choices::new(vec!["One".into(), "Two".into(), "Three".into()]))),
        ]),
        }
    }
}

impl TemplateForm for ExampleTemplate {
    fn title(&self) -> String {
        "Example Template".to_owned()
    }

    fn description(&self) -> String {
        "This is the description for the Example Template.".to_owned()
    }

    fn update(&mut self, ui: &mut Ui) {
        self.form.update(ui);
    }

    fn generate(&self) -> Result<(), ProjectError> {
        // Nothing to generate.
        Ok(())
    }

    fn reset(&mut self) {
        todo!()
    }
}