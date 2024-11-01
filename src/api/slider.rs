
#[derive(Debug, Clone)]
pub enum MessageSlider {
    ValueChanged(BuilderSlider),
}

#[derive(Debug, Clone)]
pub struct BuilderSlider {
    id: &'static str,
    min: f32,
    max: f32,
    pub val: f32,
}

impl BuilderSlider {
    pub fn new(id: &'static str, min: f32, max: f32, val: f32) -> Self {
        Self{
            id,
            min,
            max,
            val,
        }
    }
}

impl<'a> super::ElementBuilder<'a> for BuilderSlider {
    fn build(&self) -> super::Element<'a> {
        let id = self.id;
        let min = self.min;
        let max = self.max;

        let result = iced::widget::slider(self.min..=self.max, self.min, move |new_val| {
            (id, super::MessageGenericInner::Slider(MessageSlider::ValueChanged(
                BuilderSlider::new(id, min, max, new_val),
            )))
        });

        result.into()
    }
    
    fn clone_box(&self) -> Box<dyn super::ElementBuilder<'a>> {
        Box::new(self.clone())
    }
}

impl std::default::Default for BuilderSlider {
    fn default() -> Self {
        Self {
            id: Default::default(),
            min: Default::default(),
            max: Default::default(),
            val: Default::default()
        }
    }
}
