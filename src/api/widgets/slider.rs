use crate::api::{Element, ElementBuilder, MessageGenericInner};

#[derive(Debug, Clone)]
pub enum MessageSlider {
    ValueChanged(BuilderSlider),
}

#[derive(Default, Debug, Clone)]
pub struct BuilderSlider {
    id: &'static str,
    min: f32,
    max: f32,
    pub val: f32,
}

impl BuilderSlider {
    pub fn new(id: &'static str, min: f32, max: f32, val: f32) -> Self {
        Self { id, min, max, val }
    }
}

impl<'a> ElementBuilder<'a> for BuilderSlider {
    fn build(&self) -> Element<'a> {
        let id = self.id;
        let min = self.min;
        let max = self.max;

        let result = iced::widget::slider(self.min..=self.max, self.min, move |new_val| {
            (
                id.to_string(),
                MessageGenericInner::Slider(MessageSlider::ValueChanged(BuilderSlider::new(
                    id, min, max, new_val,
                ))),
            )
        });

        result.into()
    }

    fn clone_box(&self) -> Box<dyn ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}
