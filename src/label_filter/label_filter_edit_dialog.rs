use fltk::app::{self, channel};
use fltk::enums::{CallbackTrigger, Event, Key};
use fltk::frame::Frame;
use fltk::group::{Flex, Pack, PackType};
use fltk::input::Input;
use fltk::menu::Choice;
use fltk::{button::*, window::*};

use fltk::prelude::*;

use parking_lot::Mutex;
use serious_organizer_lib::lens::Lens;
use serious_organizer_lib::models::LabelAutoFilter;
use std::sync::Arc;

use crate::label_filter::label_filter_preview_list::LabelFilterPreviewList;

// use super::entry_label_list::EntryLabelList;

#[derive(Clone, Debug)]
pub enum LabelFilterEditMessage {
    NameChanged(String),
    FilterChanged(String),
    LabelChanged(String),
    ListChanged,
    SaveClicked,
    ExitDialog,
}

pub struct LabelFilterEditDialog {
    lens: Arc<Mutex<Lens>>,
    label_filter: Arc<Mutex<LabelAutoFilter>>,
}

impl LabelFilterEditDialog {
    pub fn new(lens: Arc<Mutex<Lens>>) -> Self {
        LabelFilterEditDialog {
            lens,
            label_filter: Arc::new(Mutex::new(LabelAutoFilter {
                id: -1,
                name: String::new(),
                filter: String::new(),
                label_id: -1,
            })),
        }
    }

    pub fn show_edit(&self, label_filter: &LabelAutoFilter) {
        *self.label_filter.lock() = label_filter.clone();
        self.show()
    }

    pub fn show_new(&self) {
        self.show()
    }

    fn show(&self) {
        let (sender, reciever) = channel::<LabelFilterEditMessage>();

        let mut dialog = Window::new(300, 100, 610, 560, "New Label filter");
        dialog.make_modal(true);
        dialog.make_resizable(true);

        let mut col = Flex::default_fill().column();
        col.set_margin(10);

        let mut top_row = Pack::default().with_size(dialog.width(), 25);

        let _spacer = Frame::default().with_size(35, 25);
        let mut input_name = Input::default().with_size(130, 25).with_label("Name");

        let _spacer = Frame::default().with_size(40, 25);
        let mut input_filter = Input::default().with_size(180, 25).with_label("Filter");

        let _spacer = Frame::default().with_size(40, 25);
        let mut choice = Choice::default().with_size(120, 25).with_label("Label");

        for label in self.lens.lock().get_labels().iter() {
            choice.add_choice(&label.name);
        }

        {
            let label_filter = self.label_filter.lock();
            input_name.set_value(&label_filter.name);
            input_filter.set_value(&label_filter.filter);
            // TODO: Set label_id as well
        }

        top_row.end();
        top_row.set_spacing(10);
        top_row.set_type(PackType::Horizontal);
        col.set_size(&top_row, 25);

        let lens_c = self.lens.clone();
        let sender_c = sender.clone();
        let mut lbl_table = LabelFilterPreviewList::new(200, 205, lens_c, sender_c);

        let bot_row = Flex::default_fill().row();

        let mut but_save = Button::new(10, 10, 60, 25, "Save");
        but_save.deactivate();
        let mut but_cancel = Button::new(80, 10, 60, 25, "Cancel");

        top_row.end();
        col.set_size(&bot_row, 25);

        col.end();

        // Button save callback
        but_save.emit(sender.clone(), LabelFilterEditMessage::SaveClicked);

        // Button cancel callback
        but_cancel.emit(sender.clone(), LabelFilterEditMessage::ExitDialog);

        // Handle dialog escape
        let sender_c = sender.clone();
        dialog.handle(move |_, evt: Event| {
            if evt.contains(Event::Shortcut) && app::event_key() == Key::Escape {
                sender_c.send(LabelFilterEditMessage::ExitDialog);
                return true;
            }

            false
        });

        // Name
        let sender_c = sender.clone();
        input_name.set_trigger(CallbackTrigger::Changed);
        input_name.set_callback(move |input_c: &mut Input| {
            let name = input_c.value();
            sender_c.send(LabelFilterEditMessage::NameChanged(name));
        });

        // Filter
        let sender_c = sender.clone();
        input_filter.set_trigger(CallbackTrigger::Changed);
        input_filter.set_callback(move |input_c: &mut Input| {
            let filter = input_c.value();
            sender_c.send(LabelFilterEditMessage::FilterChanged(filter));
        });

        // Selected label
        let sender_c = sender;
        choice.set_callback(move |c| {
            if let Some(choice) = c.choice() {
                sender_c.send(LabelFilterEditMessage::LabelChanged(choice));
            }
        });

        dialog.end();
        dialog.show();
        dialog.make_current();

        let lens_c = self.lens.clone();
        let mut name_done = false;
        let mut filter_done = false;
        let mut label_done = false;

        while dialog.shown() {
            while fltk::app::wait() {
                if let Some(msg) = reciever.recv() {
                    match msg {
                        LabelFilterEditMessage::NameChanged(name) => {
                            println!("Name changed {name}");
                            name_done = !name.trim().is_empty();
                            (*self.label_filter.lock()).name = name;

                            self.set_save_status(&mut but_save, name_done, filter_done, label_done);
                        }
                        LabelFilterEditMessage::FilterChanged(filter) => {
                            println!("Filter changed {filter}");
                            filter_done = !filter.trim().is_empty();
                            (*self.label_filter.lock()).filter = filter.clone();
                            let lens_c = lens_c.lock();
                            if let Ok(entries) = lens_c.get_entries_for_regex(&filter) {
                                println!("Got entries {}", entries.len());
                                lbl_table.set_entries(entries);
                            } else {
                                lbl_table.set_entries(Vec::new());
                                println!("Invalid regex");
                            }

                            self.set_save_status(&mut but_save, name_done, filter_done, label_done);
                        }
                        LabelFilterEditMessage::LabelChanged(label) => {
                            let lens_c = lens_c.lock();
                            if let Some(label) =
                                lens_c.get_labels().iter().find(|l| l.name == label)
                            {
                                println!("Got label: {} {}", label.name, label.id);

                                (*self.label_filter.lock()).label_id = label.id;

                                label_done = true;
                            }

                            self.set_save_status(&mut but_save, name_done, filter_done, label_done);
                        }
                        LabelFilterEditMessage::ListChanged => lbl_table.update(),
                        LabelFilterEditMessage::SaveClicked => {
                            let mut lens = lens_c.lock();
                            let label_filter = self.label_filter.lock();
                            lens.add_update_label_filter(&label_filter);

                            dialog.hide();
                            break;
                        }
                        LabelFilterEditMessage::ExitDialog => {
                            dialog.hide();
                            break;
                        }
                    }
                }
            }
        }

        println!("Exit Edit filter dialog");
    }

    fn set_save_status(
        &self,
        save_button: &mut Button,
        name_done: bool,
        filter_done: bool,
        label_done: bool,
    ) {
        println!("Active? {name_done} {filter_done} {label_done}");

        if name_done && filter_done && label_done {
            save_button.activate();
        } else {
            save_button.deactivate();
        }
    }
}
