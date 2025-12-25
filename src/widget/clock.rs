use std::time::Duration;

use gpui::{Context, IntoElement, ParentElement, Render, Window};
use time::{
    OffsetDateTime,
    error::InvalidFormatDescription,
    format_description::{self, OwnedFormatItem},
};

use crate::widget::{Widget, widget_wrapper};

const CLOCK_ICON: [&'static str; 12] = [
    "󱑊 ", "󱐿 ", "󱑀 ", "󱑁 ", "󱑃 ", "󱑂 ", "󱑄 ", "󱑅 ", "󱑆 ", "󱑇 ", "󱑈 ", "󱑉 ",
];

pub struct Clock {
    format_description: Result<OwnedFormatItem, InvalidFormatDescription>,
}

impl Widget for Clock {
    fn new(cx: &mut Context<Self>) -> Self {
        let format_description = format_description::parse_owned::<2>(
            "[month padding:none repr:numerical]/[day padding:none] [weekday repr:short] [hour padding:none repr:12]:[minute padding:zero] [period case:upper]",
        );

        if format_description.is_ok() {
            cx.spawn(async move |this, cx| {
                loop {
                    let _ = this.update(cx, |_, cx| cx.notify());
                    cx.background_executor()
                        .timer(Duration::from_millis(500))
                        .await;
                }
            })
            .detach();
        }

        Self { format_description }
    }
}

impl Render for Clock {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let clock = match &self.format_description {
            Ok(format_description) => current_time(format_description),
            Err(e) => format!("Error while parsing time format description: {e}"),
        };
        widget_wrapper().child(clock)
    }
}

// TODO: maybe we should use icu4x for localized formatting?
fn current_time(format_description: &OwnedFormatItem) -> String {
    let time = match OffsetDateTime::now_local() {
        Ok(x) => x,
        Err(e) => return format!("Error while getting local time: {e}"),
    };
    format!(
        "{}{}",
        CLOCK_ICON.get(time.hour() as usize % 12).unwrap_or(&""),
        match time.format(format_description) {
            Ok(x) => x,
            Err(e) => return format!("Error while formatting time `{time}`: {e}"),
        }
    )
}
