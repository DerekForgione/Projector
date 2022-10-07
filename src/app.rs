#[allow(unused)]
use crate::proj::{self, *};
use crate::egui_ext::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ProjectorMain<'a> {
    // this how you opt-out of serialization of a member
    #[serde(skip)]
    template_data: TemplateData<'a>,
}

impl<'a> Default for ProjectorMain<'a> {
    fn default() -> Self {
        Self {
            template_data: TemplateData::<'a>::load(),
        }
    }
}

impl<'a> ProjectorMain<'a> {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl<'a> eframe::App for ProjectorMain<'a> {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self { 
            template_data,
         } = self;


        egui::CentralPanel::default().show(ctx, |ui| {
            template_data.forms
                .iter_mut()
                .for_each(|form| {
                    form.update(ui);
                });
            egui::warn_if_debug_build(ui);
        });



    }
}


struct FormTest {
    form: Form
}

impl FormTest {
    fn new() -> Self {
        Self {
            form: Form::with(vec![
                FormItem::new("Boolean", "Boolean", FormData::Boolean(false)),
                FormItem::new("Integer", "Integer", FormData::Integer(ScalarData::new(0))),
                FormItem::new("Unsigned", "Unsigned", FormData::Unsigned(ScalarData::new(0))),
            ])
        }
    }
}