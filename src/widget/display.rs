use gpui::{Context, IntoElement, ParentElement, Render, Window};

use crate::widget::{Widget, widget_wrapper};

pub struct Display;

impl Widget for Display {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for Display {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let display = match window.display(cx) {
            Some(display) => format!("display = {:?}", display.id()),
            None => "display not found".to_owned(),
        };
        widget_wrapper().child(display)
    }
}
