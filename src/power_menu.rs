use gpui::{
    App, ClickEvent, Context, DisplayId, ElementId, Entity, FocusHandle, FontWeight, KeyBinding,
    StatefulInteractiveElement, Window, WindowBackgroundAppearance, WindowKind, WindowOptions,
    actions, black, div,
    layer_shell::{KeyboardInteractivity, Layer, LayerShellOptions},
    opaque_grey,
    prelude::*,
    rems, rgba, white,
};

actions!([Escape]);

pub struct PowerMenu {
    selected: Option<PowerMenuOption>,
    focus_handle: FocusHandle,
}

impl PowerMenu {
    pub fn build_root_view(window: &mut Window, cx: &mut App) -> Entity<Self> {
        cx.new(|cx| {
            cx.bind_keys([
                KeyBinding::new("escape", Escape, Some("power-menu")),
                KeyBinding::new("q", Escape, Some("power-menu")),
            ]);

            // TODO: on_action callback on an element requires that element to be focused,
            // should see if there is any way to bind a key on window level
            let focus_handle = cx.focus_handle();
            focus_handle.focus(window, cx);

            Self {
                selected: None,
                focus_handle,
            }
        })
    }
    pub fn window_options(display_id: Option<DisplayId>) -> WindowOptions {
        WindowOptions {
            titlebar: None,
            display_id,
            window_background: WindowBackgroundAppearance::Transparent,
            kind: WindowKind::LayerShell(LayerShellOptions {
                namespace: "eucalyptus-twig-power-menu".to_owned(),
                layer: Layer::Overlay,
                keyboard_interactivity: KeyboardInteractivity::Exclusive,
                ..Default::default()
            }),
            ..Default::default()
        }
    }
}

fn button(
    id: impl Into<ElementId>,
    text: &'static str,
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div().id(id).on_click(on_click).child(text)
}

impl Render for PowerMenu {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .key_context("power-menu")
            .track_focus(&self.focus_handle)
            // .on_action(|_escape: &Escape, window, _cx| {
            //     window.remove_window();
            // })
            .size_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .bg(opaque_grey(0.2, 0.8))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(rems(1.0))
                    .text_size(rems(1.5))
                    .font_weight(FontWeight::EXTRA_BOLD)
                    .text_color(white())
                    .bg(rgba(0x0000044))
                    .rounded_xl()
                    .child(button(
                        "reboot",
                        "Reboot",
                        cx.listener(|this, _, _, _| this.selected = Some(PowerMenuOption::Reboot)),
                    ))
                    .child(button(
                        "shutdown",
                        "Shutdown",
                        cx.listener(|this, _, _, _| {
                            this.selected = Some(PowerMenuOption::Shutdown)
                        }),
                    ))
                    .child(format!("selected == {:?}", self.selected)),
            )
            .child(
                div()
                    .id("power-menu-close")
                    .on_click(|_click_event, window, _cx| {
                        window.remove_window();
                    })
                    .text_color(white())
                    .bg(black())
                    .child("Close"),
            )
    }
}

#[derive(Clone, Copy, Debug)]
enum PowerMenuOption {
    Reboot,
    Shutdown,
}
