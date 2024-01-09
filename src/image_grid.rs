use crate::image_box::{Animation, ImageBox, ImageBoxId, IMAGE_WIDTH};
use makepad_widgets::*;

const ROWS: i32 = 40;
const COLS: i32 = 20;

live_design! {
    import crate::image_box::*;
    import makepad_widgets::base::*;

    // IMG_1 = dep("crate://self/resources/image_1_5x5.png")
    // IMG_2 = dep("crate://self/resources/image_2_5x5.png")
    // IMG_3 = dep("crate://self/resources/image_3_5x5.png")
    IMG_1 = dep("crate://self/resources/image_1_200x200.jpg")
    IMG_2 = dep("crate://self/resources/image_2_200x200.jpg")
    IMG_3 = dep("crate://self/resources/image_3_200x200.jpg")

    ImageGrid= {{ImageGrid}} {
        fading_image_box: <ImageBox> {
            image: <RotatedImage> {
                source: (IMG_1)
            }
        }
        scaling_image_box: <ImageBox> {
            image: <RotatedImage> {
                source: (IMG_2)
            }
        }
        rotating_image_box: <ImageBox> {
            image: <RotatedImage> {
                source: (IMG_3)
            }
        }
        width: Fill,
        height: Fill
    }
}

#[derive(Live, WidgetWrap)]
pub struct ImageGrid {
    #[rust]
    #[redraw]
    area: Area,
    #[walk]
    walk: Walk,
    #[layout]
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for image_box in self.image_boxes.values_mut() {
            image_box.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, _walk: Walk) -> DrawStep {
        let start_pos = cx.turtle().pos();
        for (box_id, image_box) in self.image_boxes.iter_mut() {
            let box_idu64 = box_id.0.get_value();
            let image_offset = ((IMAGE_WIDTH * IMAGE_WIDTH * 2.0).sqrt() - IMAGE_WIDTH) / 2.0;
            let mut pos = start_pos
                + dvec2(
                    (box_idu64 / 100) as f64 * IMAGE_WIDTH - image_offset,
                    (box_idu64 % 100) as f64 * IMAGE_WIDTH - image_offset,
                );

            image_box.draw_all(cx, &mut Scope::with_data(&mut pos));
        }

        DrawStep::done()
    }
}

impl LiveRegister for ImageGrid {
    fn live_register(cx: &mut Cx) {
        register_widget!(cx, ImageGrid);
        crate::image_box::live_design(cx);
    }
}

impl LiveHook for ImageGrid {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        for y in 0..ROWS {
            for x in 0..COLS {
                let box_id = LiveId(x as u64 * 100 + y as u64).into();

                let mut new_box;
                let pattern_index = ((x as i64 - y as i64).rem_euclid(3) + 3) % 3;
                let animation = Animation::from_index(pattern_index as usize);

                match animation {
                    Animation::Fade => {
                        new_box = ImageBox::new_from_ptr(cx, self.fading_image_box);
                    }
                    Animation::Scale => {
                        new_box = ImageBox::new_from_ptr(cx, self.scaling_image_box);
                    }
                    Animation::Rotate => {
                        new_box = ImageBox::new_from_ptr(cx, self.rotating_image_box);
                    }
                }

                new_box.animation = animation;
                self.image_boxes.insert(box_id, new_box);
            }
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        for image_box in self.image_boxes.values_mut() {
            if let Some(index) = nodes.child_by_name(index, live_id!(image_box).as_field()) {
                image_box.apply(cx, apply, index, nodes);
            }
        }
    }
}
