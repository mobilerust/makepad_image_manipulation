use makepad_widgets::*;

live_design! {
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_widgets::view::*;
    import makepad_draw::shader::std::*;

    import makepad_image_manipulation::image_grid::ImageGrid;

    COLOR_BG = #FDFDFD

    App = {{App}} {
        ui: <Window> {
            window: {inner_size: vec2(540, 960)},
            show_bg: true
            flow: Down,
            spacing: 0.0,
            align: {
                x: 0.0,
                y: 0.0
            },
            padding: {top: 10.0, right: 5.0, bottom: 10.0, left: 5.0}
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

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
        crate::image_grid::live_design(cx);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
