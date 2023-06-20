use crate::makepad_widgets::*;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::*;
    import makepad_widgets::label::Label;

    // Heads-up this is just using the first image for now
    // need to make the images same size in order to usem (and figure out how to identify them)
    IMG_1 = dep("crate://self/resources/image_1.jpg")
    IMG_2 = dep("crate://self/resources/image_2.jpg")
    IMG_3 = dep("crate://self/resources/image_3.jpg")

    DrawBg = {{DrawBg}} {
        fn pixel(self) -> vec4 {
            // for debugging:
            // return #3;
            return #FDFDFD;
        }
    }

    DrawImage = {{DrawImage}} {
        <Image> {
            image: (IMG_1),
            image_scale: 0.08,
            walk: {
                margin: 0.0,
                width:  Fill,
                height: Fit,
            },
            layout: {padding: 0}

            draw_bg: {
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                    // TODO: can't access DrawImage.self here (for self.angle)
                    // perhaps we need to figure out a way to run the frame code directly in DrawImage without calling Image
                    // Image is an extension of Frame so we could try using that code.

                    let angle = 90;
                    let c = self.rect_size * 0.5;

                    sdf.rotate(angle, c.x, c.y);

                    sdf.box(1., 1., self.rect_size.x, self.rect_size.y, 1);

                    sdf.fill_keep(self.get_color());

                    return sdf.result;
                }
            }
        }
    }

    ImageBox= {{ImageBox}} {
        layout: {padding:2}
    }

    ImageGrid= {{ImageGrid}} {
        image_box: <ImageBox> {}
        walk: {
            width: Fill,
            height: Fill
        }
    }
}

#[derive(Live, LiveHook)]
#[repr(C)]
pub struct DrawBg {
    #[deref]
    draw_super: DrawQuad,
    #[live]
    fast_path: f32,
}

#[derive(Live, LiveHook)]
#[repr(C)]
pub struct DrawImage {
    #[deref]
    frame: Frame,
    #[live]
    angle: f32,
}

#[derive(Live, LiveHook)]
pub struct ImageBox {
    #[live]
    draw_bg: DrawBg,
    #[live]
    draw_image: DrawImage,

    #[live]
    layout: Layout,
    #[state]
    state: LiveState,

    #[live]
    angle: f32,

    #[live]
    label_align: Align,
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
        let _ = self.draw_walk(cx, walk);
        WidgetDraw::done()
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

impl ImageBox {
    pub fn handle_event_with(
        &mut self,
        _cx: &mut Cx,
        _event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, ImageBoxAction),
    ) {
    }

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2, angle: f32) {
        self.draw_image.angle = angle;

        self.draw_bg
            .begin(cx, Walk::fit().with_abs_pos(pos), self.layout);

        _ = self.draw_image.draw_walk_widget(cx, Walk::fit());

        self.draw_bg.end(cx);
    }
}

#[derive(Clone, WidgetAction)]
pub enum ImageGridAction {
    None,
}

pub enum ImageBoxAction {}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ImageBoxId(pub LiveId);

impl ImageGrid {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, _walk: Walk) {
        let start_pos = cx.turtle().pos();
        let image_box = self.image_box;

        // TODO: just grabbed this from the numbers example
        // might not be relevant for benchmarking but we could do a proper grid system
        for y in 0..2 {
            for x in 0..16 {
                let box_id = LiveId(x * 100 + y).into();
                let image_box = self
                    .image_boxes
                    .get_or_insert(cx, box_id, |cx| ImageBox::new_from_ptr(cx, image_box));
                let pos = start_pos + dvec2(x as f64 * 40.0, y as f64 * 25.0);
                let angle = 100.0;
                image_box.draw_abs(cx, pos, angle);
            }
        }
        self.image_boxes.retain_visible();
    }

    pub fn handle_event_with(
        &mut self,
        _cx: &mut Cx,
        _event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, ImageGridAction),
    ) {
        // let mut actions = Vec::new();
        // for (box_id, image_box) in self.image_boxes.iter_mut() {
        //     image_box
        //         .handle_event_with(cx, event, &mut |_, action| actions.push((*box_id, action)));
        // }

        // for (_node_id, action) in actions {
        //     match action {}
        // }
    }
}
