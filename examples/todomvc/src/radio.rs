use std::fmt::{Debug, Display};

use strum::IntoEnumIterator;

use gtk::prelude::*;
use gtk::*;
use vgtk::{gtk, Callback, Component, VNode};

#[derive(Clone, Debug, Default)]
pub struct Radio<Enum: Unpin> {
    pub active: Enum,
    pub on_changed: Option<Callback<Enum>>,
}

#[derive(Clone, Debug)]
pub enum RadioMsg<Enum: Unpin> {
    Selected(Enum),
}

impl<Enum, I> Component for Radio<Enum>
where
    Enum: 'static
        + IntoEnumIterator<Iterator = I>
        + Display
        + PartialEq
        + Debug
        + Default
        + Copy
        + Send
        + Unpin,
    I: Iterator<Item = Enum>,
{
    type Message = RadioMsg<Enum>;
    type Properties = Self;

    fn create(props: Self::Properties) -> Self {
        props
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        *self = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        match msg {
            RadioMsg::Selected(selected) => {
                self.active = selected;
                if let Some(ref callback) = self.on_changed {
                    callback.send(self.active);
                }
            }
        }
        true
    }

    fn view(&self) -> VNode<Radio<Enum>> {
        gtk! {
            <Box orientation={Orientation::Horizontal} spacing=10>
                { Enum::iter().map(|label| {
                    gtk!{
                        <ToggleButton label={label.to_string()} active={label == self.active}
                                      on toggled=|_| {RadioMsg::Selected(label)}/>
                    }
                }) }
            </Box>
        }
    }
}
