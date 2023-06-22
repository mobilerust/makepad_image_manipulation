use makepad_widgets::*;

live_design! {
    import makepad_widgets::desktop_window::DesktopWindow;
    import makepad_widgets::frame::*;
    import makepad_draw::shader::std::*;

    import makepad_image_manipulation::image_grid::ImageGrid;

    COLOR_BG = #FDFDFD

    App = {{App}} {
        ui: <DesktopWindow>{
            window: {inner_size: vec2(540, 960)},
            show_bg: true
            layout: {
                flow: Down,
                spacing: 0.0,
                align: {
                    x: 0.0,
                    y: 0.0
                },
                padding: {top: 10.0, right: 5.0, bottom: 10.0, left: 5.0}
            },
            draw_bg: {
                fn pixel(self) -> vec4 {
                    return (COLOR_BG)
                }
            }

            <ImageGrid> {}
        }
    }
}

app_main!(App);

#[derive(Live)]

pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        crate::makepad_widgets::live_design(cx);
        crate::image_grid::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            let cx = &mut Cx2d::new(cx, event);

            self.ui.draw_widget_all(cx);
        }

        self.ui.handle_widget_event(cx, event);
    }
}
