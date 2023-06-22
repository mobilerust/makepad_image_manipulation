use crate::makepad_widgets::*;
use makepad_widgets::frame::Frame as Image;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::Image;

    IMG_1 = dep("crate://self/resources/image_1.jpg")

    ImageBox= {{ImageBox}} {
        layout: {padding:2}

        image: <Image> {
            walk: {width: 80, height: 80},
            image: (IMG_1)
            show_bg: true
            draw_bg: {
                instance angle: 0.0

                fn rotate_2d_from_center(v: vec2, a: float) -> vec2 {
                    let ca = cos(-a);
                    let sa = sin(-a);
                    let p = v - vec2(0.5, 0.5);
                    return vec2(p.x * ca - p.y * sa, p.x * sa + p.y * ca) + vec2(0.5, 0.5);
                }

                fn get_color(self) -> vec4 {
                    let rot = rotate_2d_from_center(self.pos.xy, self.angle);
                    return sample2d(self.image, rot).xyzw;
                }

                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                    let c = self.rect_size * 0.5;
                    sdf.rotate(self.angle, c.x, c.y);
                    sdf.box(1., 1., self.rect_size.x, self.rect_size.y, 1);

                    sdf.fill(self.get_color());
                    return sdf.result
                }
            }
        }

        state: {
            rotation = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {angle: 0.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 10, end: 1.0}}
                    apply: {
                        image: { draw_bg: {angle: 6.28318}}
                    }
                }
            }
        }
    }

    ImageGrid= {{ImageGrid}} {
        image_box: <ImageBox> {}
        walk: {
            width: Fill,
            height: Fill
        }
    }
}

#[derive(Live)]
pub struct ImageGrid {
    #[live]
    walk: Walk,
    #[live]
    layout: Layout,
    #[live]
    image_box: Option<LivePtr>,
    #[rust]
    image_boxes: ComponentMap<ImageBoxId, ImageBox>,
}

impl Widget for ImageGrid {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
    }

    fn get_walk(&self) -> Walk {
        self.walk
    }

    fn redraw(&mut self, _cx: &mut Cx) {}

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl LiveHook for ImageGrid {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ImageGrid)
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, index: usize, nodes: &[LiveNode]) {
        for image_box in self.image_boxes.values_mut() {
            if let Some(index) = nodes.child_by_name(index, live_id!(image_box).as_field()) {
                image_box.apply(cx, from, index, nodes);
            }
        }
    }
}

impl ImageGrid {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, _walk: Walk) {
        let start_pos = cx.turtle().pos();
        let image_box = self.image_box;

        for y in 0..8 {
            for x in 0..3 {
                let box_id = LiveId(x * 100 + y).into();
                let image_box = self
                    .image_boxes
                    .get_or_insert(cx, box_id, |cx| ImageBox::new_from_ptr(cx, image_box));
                let pos = start_pos + dvec2(x as f64 * 130.0, y as f64 * 130.0);
                image_box.draw_abs(cx, pos);
            }
        }
        self.image_boxes.retain_visible();
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, ImageGridAction),
    ) {
        let mut actions = Vec::new();
        for (box_id, image_box) in self.image_boxes.iter_mut() {
            image_box
                .handle_event_with(cx, event, &mut |_, action| actions.push((*box_id, action)));
        }
    }
}

#[derive(Live)]
pub struct ImageBox {
    #[live]
    draw_bg: DrawQuad,
    #[live]
    image: Image,

    #[live]
    layout: Layout,
    #[state]
    state: LiveState,

    #[live]
    angle: f32,
}

impl LiveHook for ImageBox {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.animate_state(cx, id!(rotation.on));
    }
}

impl ImageBox {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, ImageBoxAction),
    ) {
        self.state_handle_event(cx, event);

        if let Hit::FingerHoverIn(_) = event.hits(cx, self.image.area()) {
            cx.set_cursor(MouseCursor::Arrow);
            self.animate_state(cx, id!(rotation.on));
        }
    }

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2) {
        let bg_size = Size::Fixed(120.0);

        _ = self
            .image
            .draw_walk_widget(cx, Walk::size(bg_size, bg_size).with_abs_pos(pos));
    }
}

#[derive(Clone, WidgetAction)]
pub enum ImageGridAction {
    None,
}

pub enum ImageBoxAction {}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ImageBoxId(pub LiveId);
