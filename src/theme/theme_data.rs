use bevy::prelude::*;
use sickle_math::ease::Ease;

use super::{
    icons::Icons,
    theme_colors::{SchemeColors, ThemeColors},
    theme_spacing::ThemeSpacing,
    typography::ThemeTypography,
    AnimationSettings,
};

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub enum Contrast {
    #[default]
    Standard,
    Medium,
    High,
}

#[derive(Clone, Copy, Debug, Reflect)]
pub enum Scheme {
    Light(Contrast),
    Dark(Contrast),
}

impl Default for Scheme {
    fn default() -> Self {
        Self::Dark(Default::default())
    }
}

impl Scheme {
    pub fn is_light(&self) -> bool {
        matches!(self, Scheme::Light(_))
    }

    pub fn is_dark(&self) -> bool {
        matches!(self, Scheme::Dark(_))
    }
}

#[derive(Resource, Clone, Debug, Reflect)]
pub struct ThemeData {
    pub active_scheme: Scheme,
    pub colors: ThemeColors,
    pub spacing: ThemeSpacing,
    pub text: ThemeTypography,
    pub icons: Icons,
    pub interaction_animation: AnimationSettings,
}

impl Default for ThemeData {
    fn default() -> Self {
        let mut interaction_animation = AnimationSettings::new();
        interaction_animation
            .pointer_enter(0.1, Ease::OutExpo, None)
            .pointer_leave(0.1, Ease::OutExpo, None)
            .press(0.1, Ease::OutExpo, None);

        Self {
            active_scheme: Default::default(),
            colors: Default::default(),
            spacing: Default::default(),
            text: Default::default(),
            icons: Default::default(),
            interaction_animation,
        }
    }
}

impl ThemeData {
    /// Returns the scheme colors of the current active scheme / contrast
    pub fn colors(&self) -> SchemeColors {
        match self.active_scheme {
            Scheme::Light(contrast) => self.colors.schemes.light.contrast(contrast),
            Scheme::Dark(contrast) => self.colors.schemes.dark.contrast(contrast),
        }
    }
}
