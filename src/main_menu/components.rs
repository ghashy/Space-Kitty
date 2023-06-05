use bevy::prelude::*;

#[derive(Component)]
pub struct MainMenu {}

#[derive(Component)]
pub struct PlanetsLayer;

pub trait ButtonType {
    fn get_default_handle(&self) -> Handle<Image>;
    fn get_hover_handle(&self) -> Handle<Image>;
    fn get_click_handle(&self) -> Handle<Image>;
}

#[derive(Component)]
pub struct PlayButton {
    pub default_handle: Handle<Image>,
    pub hover_handle: Handle<Image>,
    pub click_handle: Handle<Image>,
}

#[derive(Component)]
pub struct QuitButton {
    pub default_handle: Handle<Image>,
    pub hover_handle: Handle<Image>,
    pub click_handle: Handle<Image>,
}

macro_rules! impl_button_type {
    ($component:ty) => {
        impl ButtonType for &$component {
            fn get_default_handle(&self) -> Handle<Image> {
                self.default_handle.clone_weak()
            }
            fn get_hover_handle(&self) -> Handle<Image> {
                self.hover_handle.clone_weak()
            }
            fn get_click_handle(&self) -> Handle<Image> {
                self.click_handle.clone_weak()
            }
        }
    };
}

impl_button_type!(PlayButton);
impl_button_type!(QuitButton);
