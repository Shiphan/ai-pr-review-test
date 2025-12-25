use std::env;

use futures::io::{AsyncBufReadExt, BufReader};
use gpui::{AsyncApp, Context, IntoElement, ParentElement, Render, WeakEntity, Window};
use gpui_net::async_net::UnixStream;

use crate::widget::{Widget, widget_wrapper};

pub struct HyprlandWorkspace {
    info: String,
}

impl Widget for HyprlandWorkspace {
    fn new(cx: &mut Context<Self>) -> Self {
        cx.spawn(info).detach();

        Self {
            info: String::new(),
        }
    }
}

impl Render for HyprlandWorkspace {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        widget_wrapper().child(self.info.trim().to_owned())
    }
}

async fn info(this: WeakEntity<HyprlandWorkspace>, cx: &mut AsyncApp) {
    let xdg_runtime_dir = match env::var("XDG_RUNTIME_DIR") {
        Ok(x) => x,
        Err(e) => {
            let _ = this.update(cx, |this, cx| {
                this.info = format!("error while getting XDG_RUNTIME_DIR: {e}");
                cx.notify()
            });
            return;
        }
    };
    let hyprland_instance_signature = match env::var("HYPRLAND_INSTANCE_SIGNATURE") {
        Ok(x) => x,
        Err(e) => {
            let _ = this.update(cx, |this, cx| {
                this.info = format!("error while getting HYPRLAND_INSTANCE_SIGNATURE: {e}");
                cx.notify()
            });
            return;
        }
    };
    let socket_path = format!("{xdg_runtime_dir}/hypr/{hyprland_instance_signature}/.socket2.sock");

    let mut stream = match UnixStream::connect(socket_path).await {
        Ok(x) => BufReader::new(x),
        Err(e) => {
            let _ = this.update(cx, |this, cx| {
                this.info = format!("error while connecting to hyprland socket: {e}");
                cx.notify()
            });
            return;
        }
    };

    loop {
        let mut buffer = vec![];
        let message = match stream.read_until(b'\n', &mut buffer).await {
            Ok(_) => match String::from_utf8(buffer) {
                Ok(line) => {
                    if line.starts_with("activelayout>>") {
                        continue;
                    }
                    line
                }
                Err(e) => format!("the bytes from socket is not valid UTF8: {e}"),
            },
            Err(e) => format!("error while reading the socket: {e}"),
        };
        let _ = this.update(cx, |this, cx| {
            this.info = message;
            cx.notify()
        });
    }
}
