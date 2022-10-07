#![allow(unused)]

use std::{path::PathBuf, ops::{RangeInclusive, RangeBounds}, string};

use eframe::emath::Numeric;

use {
    std::{
        path::{
            Path,
    
        },
        time::Instant,
        hash::Hash,
        ops::Range,
        
    },
    egui::{
        widgets::{
            *,
        },
        *,
    },
};

pub trait FormWidget {
    fn update(&mut self, ui: &mut Ui) -> Response;
    fn reset(&mut self) {}
}

#[inline(always)]
pub fn time_id() -> Id {
    Id::new(Instant::now())
}

#[derive(Clone)]
pub struct Form {
    pub items: Vec<FormItem>,
}

impl Form {
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
        }
    }

    pub fn with(items: impl Into<Vec<FormItem>>) -> Self {
        Self {
            items: items.into(),
        }
    }
}

#[derive(Clone)]
pub struct FormItem {
    pub text: String,
    pub data: FormData,
    pub(crate) name: String,
    pub(crate) id: Id,
}

impl FormItem {

    #[inline(always)]
    pub fn new(name: impl Into<String>, text: impl Into<String>, data: impl Into<FormData>) -> Self {
        Self {
            text: text.into(),
            data: data.into(),
            name: name.into(),
            id: time_id(),
        }
    }

    #[inline(always)]
    pub fn id(mut self, source: impl Hash) -> Self {
        self.id = Id::new(source);
        self
    }

    #[inline(always)]
    pub fn get_id(&self) -> Id {
        self.id
    }

    #[inline(always)]
    pub fn name(&self) -> &str {
        &self.name
    }

}

/// Used to hold a value of type T.
/// We can then apply a trait that allows us to render it.
/// This allows us to create a renderable for each enum variant.
#[derive(Clone)]
pub struct DataContainer<T> {
    pub data: T
}

#[derive(Clone)]
pub enum FormData {
    Empty,
    Boolean(bool),
    Integer(ScalarData<isize>),
    Unsigned(ScalarData<usize>),
    Text(TextData),
    Double(ScalarData<f64>),
    File(FileData),
    OptionalStruct(Form),
    Choice(Choices),
    Options(Options),
    Struct(Form),
    Optional(Box<FormData>),
}

impl FormWidget for FormData {
    fn update(&mut self, ui: &mut Ui) -> Response {
        use FormData::*;
        match self {
            Empty => ui.label(RichText::new("Empty").color(Color32::RED)),
            Boolean(value) => value.update(ui),
            Integer(value) => value.update(ui),
            Unsigned(value) => value.update(ui),
            Text(value) => value.update(ui),
            Double(value) => value.update(ui),
            File(value) => value.update(ui),
            OptionalStruct(value) => value.update(ui),
            Choice(value) => value.update(ui),
            Options(value) => value.update(ui),
            Struct(value) => value.update(ui),
            Optional(value) => value.update(ui),
        }
    }
}

impl FormWidget for Form {
    fn update(&mut self, ui: &mut Ui) -> Response {
        ui.group(|ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.columns(2, |cols| {
                    self.items
                        .iter_mut()
                        .for_each(|item| {
                            cols[0].label(item.text.as_str());
                            cols[1].push_id(item.id, |ui| {
                                item.data.update(ui);
                            });
                        });
                });
            });
        }).response
    }
}

impl FormWidget for bool {
    fn update(&mut self, ui: &mut Ui) -> Response {
        ui.toggle_value(self, match self {
            true => "Turn Off",
            false => "Turn On",
        })
    }

    fn reset(&mut self) {
        *self = false;
    }

}

#[derive(Clone)]
pub struct Options {
    pub options: Vec<(String, bool)>,
}

impl Options {
    pub fn new(options: Vec<(String, bool)>) -> Self {
        Self {
            options
        }
    }
}

impl FormWidget for Options {
    fn update(&mut self, ui: &mut Ui) -> Response {
        if self.options.is_empty() {
            ui.label(RichText::new("No options!").color(Color32::RED))
        } else {
            // TODO: Manage Response for change event.
            ui.group(|ui| {
                self.options
                    .iter_mut()
                    .for_each(|item| {
                        ui.checkbox(&mut item.1, item.0.as_str());
                    });
            }).response
        }
    }

    fn reset(&mut self) {
        self.options
            .iter_mut()
            .for_each(|item| {
                item.1 = false;
            });
    }

}

#[derive(Clone)]
pub struct Choices {
    pub selected: usize,
    pub choices: Vec<String>,
}

impl Choices {
    #[inline(always)]
    pub fn new(choices: Vec<String>) -> Self {
        Self {
            choices,
            selected: 0,
        }
    }

    #[inline(always)]
    pub fn selected(&self) -> Option<(usize, &str)> {
        if self.choices.is_empty() {
            return None;
        }
        Some(
            (
                self.selected,
                &self.choices[self.selected]
            )
        )
    }

}

impl FormWidget for Choices {
    fn update(&mut self, ui: &mut Ui) -> Response {
        let seltext = if self.choices.is_empty() {
            &"<NULL>"
        } else {
            self.choices[self.selected].as_str()
        };
        egui::ComboBox::new("choices_combo", "")
            .selected_text(seltext)
            .show_ui(ui, |ui| {
                self.choices
                    .iter()
                    .enumerate()
                    .for_each(|(i, item)| {
                        ui.selectable_value(&mut self.selected, i, item);
                    });
            }).response
    }

    fn reset(&mut self) {
        self.selected = 0;
    }

}

#[derive(Clone)]
pub struct ScalarData<T> {
    pub value: T,
    pub range: RangeInclusive<T>,
}

impl<T> ScalarData<T>
where T: Numeric {
    pub fn new(value: T) -> Self {
        Self {
            value,
            range: T::MIN..=T::MAX,
        }
    }

    pub fn ranged(range: RangeInclusive<T>) -> Self {
        Self {
            value: *range.start(),
            range: range,
        }
    }
}

impl<T> FormWidget for ScalarData<T>
where T: Numeric + Clone {
    fn update(&mut self, ui: &mut Ui) -> Response {
        egui::Slider::new(&mut self.value, self.range.clone()).ui(ui)
    }

    fn reset(&mut self) {
        self.value = *self.range.start();
    }
}

#[derive(Clone)]
pub struct TextData {
    text: String,
    multiline: bool,
    length_range: RangeInclusive<usize>,
}

impl FormWidget for TextData {
    fn update(&mut self, ui: &mut Ui) -> Response {
        if self.multiline {
            ui.text_edit_multiline(&mut self.text)
        } else {
            ui.text_edit_singleline(&mut self.text)
        }
    }

    fn reset(&mut self) {
        self.text.clear();
    }
    
}

#[derive(Clone)]
pub struct FileData {
    path: PathBuf,
}

impl FormWidget for FileData {
    fn update(&mut self, ui: &mut Ui) -> Response {
        ui.label(RichText::new("TODO").color(Color32::RED))
    }

    fn reset(&mut self) {
        self.path = PathBuf::default();
    }
}

#[cfg(test)]
mod tests {
    use egui::*;
    use eframe::CreationContext;

    use super::*;


    #[test]
    fn form_test() {
        
    }

}