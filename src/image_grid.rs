use crate::image_box::{Animation, ImageBox, ImageBoxId, IMAGE_WIDTH};
use crate::makepad_widgets::*;

live_design! {
    import crate::image_box::*;

    IMG_1 = dep("crate://self/resources/image_1.jpg")
    IMG_2 = dep("crate://self/resources/image_2.jpg")
    IMG_3 = dep("crate://self/resources/image_3.jpg")

    ImageGrid= {{ImageGrid}} {
        fading_image_box: <ImageBox> {
            image: <CustomImage> {
                image: (IMG_1)
            }
        }
        scaling_image_box: <ImageBox> {
            image: <CustomImage> {
                image: (IMG_2)
            }
        }
        rotating_image_box: <ImageBox> {
            image: <CustomImage> {
                image: (IMG_3)
            }
        }
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
    fading_image_box: Option<LivePtr>,
    #[live]
    scaling_image_box: Option<LivePtr>,
    #[live]
    rotating_image_box: Option<LivePtr>,
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
        register_widget!(cx, ImageGrid);
        crate::image_box::live_design(cx);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let fading_image = ImageBox::new_from_ptr(cx, self.fading_image_box);
        let scaling_image = ImageBox::new_from_ptr(cx, self.scaling_image_box);
        let rotating_image = ImageBox::new_from_ptr(cx, self.rotating_image_box);

        self.image_boxes.insert(LiveId::from_str("fading_image").unwrap().into(), fading_image);
        self.image_boxes.insert(LiveId::from_str("scaling_image").unwrap().into(), scaling_image);
        self.image_boxes.insert(LiveId::from_str("rotating_image").unwrap().into(), rotating_image);
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
        for y in 0..32 {
            for x in 0..17 {
                let box_id = y * 18 + x;
                let image_offset = ((IMAGE_WIDTH * IMAGE_WIDTH * 2.0).sqrt() - IMAGE_WIDTH) / 2.0;
                let pos = start_pos
                    + dvec2(
                        x as f64 * IMAGE_WIDTH - image_offset,
                        y as f64 * IMAGE_WIDTH - image_offset,
                    );
                
                match box_id % 3 {
                    0 => {
                        println!("drawing fading image {:?}", pos);
                        self.image_boxes[LiveId::from_str("fading_image").unwrap().into()]
                            .draw_abs(cx, pos);
                    },
                    1 => {
                        println!("drawing scaling image {:?}", pos);
                        self.image_boxes[LiveId::from_str("scaling_image").unwrap().into()]
                            .draw_abs(cx, pos);
                    },
                    _ => {
                        println!("drawing rotating image {:?}", pos);
                        self.image_boxes[LiveId::from_str("rotating_image").unwrap().into()]
                            .draw_abs(cx, pos);
                    },
                }
            }
        }
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

#[derive(Clone, WidgetAction)]
pub enum ImageGridAction {
    None,
}
