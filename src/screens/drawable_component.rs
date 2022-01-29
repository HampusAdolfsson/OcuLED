use crate::rendering;

pub trait DrawableComponent {
    fn draw_to(&self, canvas: &mut rendering::Canvas);
}
