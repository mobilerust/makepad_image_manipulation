use crate::makepad_widgets::*;
use makepad_widgets::frame::Frame;
pub const IMAGE_WIDTH: f64 = 27.0;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::frame::Image;

    CustomImage = <Image> {
        show_bg: true
        draw_bg: {
            instance angle: 0.0
            instance fade_factor: 1.0
            instance scale_factor: 1.0

            fn rotation_padding(w: float, h: float) -> float {
                let d = max(w, h);
                return ((sqrt(d * d * 2.0) / d) - 1.0) / 2.0;
            }

            fn rotate_2d_from_center(v: vec2, a: float) -> vec2 {
                let ca = cos(-a);
                let sa = sin(-a);
                let p = v - vec2(0.5, 0.5);
                return vec2(p.x * ca - p.y * sa, p.x * sa + p.y * ca) + vec2(0.5, 0.5);
            }

            fn get_color(self, rot_padding: float) -> vec4 {
                // Current position is a traslated one, so let's get the original position
                let current_pos = self.pos.xy - vec2(rot_padding, rot_padding);
                let original_pos = rotate_2d_from_center(current_pos, self.angle);

                // Scale the current position by the scale factor
                let scaled_pos = (original_pos - vec2(0.5, 0.5)) / self.scale_factor + vec2(0.5, 0.5);

                // Take pixel color from the original image
                let color = sample2d(self.image, scaled_pos).xyzw;

                let faded_color = color * vec4(1.0, 1.0, 1.0, self.fade_factor);
                return faded_color;
            }

            fn pixel(self) -> vec4 {
                let rot_padding = rotation_padding(self.rect_size.x, self.rect_size.y);

                let sdf = Sdf2d::viewport(self.pos * self.rect_size);

                let translation_offset = self.rect_size * rot_padding;
                sdf.translate(translation_offset.x, translation_offset.y);

                let center = self.rect_size * 0.5;
                sdf.rotate(self.angle, center.x, center.y);

                let scaled_size = self.rect_size * self.scale_factor;
                let offset = (self.rect_size - scaled_size) * 0.5;
                sdf.box(offset.x, offset.y, scaled_size.x, scaled_size.y, 1);

                sdf.fill(self.get_color(rot_padding));
                return sdf.result
            }

            fn vertex(self) -> vec4 {
                let rot_padding = rotation_padding(self.rect_size.x, self.rect_size.y);

                // I don't know if different draw_clip values are properly supported
                let clipped: vec2 = clamp(
                    self.geom_pos * self.rect_size * (1.0 + rot_padding * 2) + self.rect_pos,
                    self.draw_clip.xy,
                    self.draw_clip.zw * (1.0 + rot_padding * 2)
                );

                self.pos = (clipped - self.rect_pos) / self.rect_size;
                return self.camera_projection * (self.camera_view * (
                    self.view_transform * vec4(clipped.x, clipped.y, self.draw_depth + self.draw_zbias, 1.)
                ));
            }
        }
    }

    ImageBox= {{ImageBox}} {
        image: <CustomImage> {}

        state: {
            fade = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {fade_factor: 1.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {fade_factor: 0.0} }
                    }
                }
            }

            scale = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {scale_factor: 1.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {scale_factor: 0.0} }
                    }
                }
            }

            rotate = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {angle: 0.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {angle: 6.28318}}
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct ImageBox {
    #[live]
    draw_bg: DrawQuad,
    #[live]
    image: Frame,
    #[live]
    layout: Layout,
    #[state]
    state: LiveState,

    #[rust]
    pub animation: Animation,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ImageBoxId(pub LiveId);

impl LiveHook for ImageBox {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        _apply_from: ApplyFrom,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
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
    }

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2) {
        if self.state.need_init() {
            match self.animation {
                Animation::Fade => self.animate_state(cx, id!(fade.on)),
                Animation::Scale => self.animate_state(cx, id!(scale.on)),
                Animation::Rotate => self.animate_state(cx, id!(rotate.on)),
            }
        }

        let bg_size = Size::Fixed(IMAGE_WIDTH);
        _ = self
            .image
            .draw_walk_widget(cx, Walk::size(bg_size, bg_size).with_abs_pos(pos));
    }
}

pub enum ImageBoxAction {}

#[derive(Default)]
pub enum Animation {
    #[default]
    Rotate,
    Scale,
    Fade,
}

impl Animation {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Animation::Fade,
            1 => Animation::Scale,
            2 => Animation::Rotate,
            _ => Animation::Rotate,
        }
    }
}
