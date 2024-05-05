use bevy::prelude::*;

use super::theme_data::Contrast;

#[derive(Clone, Copy, Debug)]
pub enum Surface {
    Background,
    Surface,
    SurfaceVariant,
    SurfaceDim,
    SurfaceBright,
    InverseSurface,
}

#[derive(Clone, Copy, Debug)]
pub enum Accent {
    Primary,
    PrimaryFixed,
    PrimaryFixedDim,
    InversePrimary,
    Secondary,
    SecondaryFixed,
    SecondaryFixedDim,
    Tertiary,
    TertiaryFixed,
    TertiaryFixedDim,
    Error,
    Outline,
    OutlineVariant,
    Shadow,
    Scrim,
}

#[derive(Clone, Copy, Debug)]
pub enum Container {
    Primary,
    Secondary,
    Tertiary,
    Error,
    SurfaceLowest,
    SurfaceLow,
    SurfaceMid,
    SurfaceHigh,
    SurfaceHighest,
}

#[derive(Clone, Copy, Debug)]
pub enum On {
    Primary,
    PrimaryContainer,
    PrimaryFixed,
    PrimaryFixedVariant,
    Secondary,
    SecondaryContainer,
    SecondaryFixed,
    SecondaryFixedVariant,
    Tertiary,
    TertiaryContainer,
    TertiaryFixed,
    TertiaryFixedVariant,
    Error,
    ErrorContainer,
    Background,
    Surface,
    SurfaceVariant,
    InverseSurface,
}

#[derive(Clone, Debug, Default, Reflect)]
pub struct ExtendedColor {
    pub name: String,
    pub color: Color,
    pub description: String,
    pub harmonized: bool,
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct CoreColors {
    pub primary: Color,
    pub secondary: Option<Color>,
    pub tertiary: Option<Color>,
    pub error: Option<Color>,
    pub neutral: Option<Color>,
    pub neutral_variant: Option<Color>,
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct SchemeColors {
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,
    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,
    pub tertiary: Color,
    pub on_tertiary: Color,
    pub tertiary_container: Color,
    pub on_tertiary_container: Color,
    pub error: Color,
    pub on_error: Color,
    pub error_container: Color,
    pub on_error_container: Color,
    pub background: Color,
    pub on_background: Color,
    pub surface: Color,
    pub on_surface: Color,
    pub surface_variant: Color,
    pub on_surface_variant: Color,
    pub outline: Color,
    pub outline_variant: Color,
    pub shadow: Color,
    pub scrim: Color,
    pub inverse_surface: Color,
    pub inverse_on_surface: Color,
    pub inverse_primary: Color,
    pub primary_fixed: Color,
    pub on_primary_fixed: Color,
    pub primary_fixed_dim: Color,
    pub on_primary_fixed_variant: Color,
    pub secondary_fixed: Color,
    pub on_secondary_fixed: Color,
    pub secondary_fixed_dim: Color,
    pub on_secondary_fixed_variant: Color,
    pub tertiary_fixed: Color,
    pub on_tertiary_fixed: Color,
    pub tertiary_fixed_dim: Color,
    pub on_tertiary_fixed_variant: Color,
    pub surface_dim: Color,
    pub surface_bright: Color,
    pub surface_container_lowest: Color,
    pub surface_container_low: Color,
    pub surface_container: Color,
    pub surface_container_high: Color,
    pub surface_container_highest: Color,
}

impl SchemeColors {
    pub fn surface(&self, surface: Surface) -> Color {
        match surface {
            Surface::Background => self.background,
            Surface::Surface => self.surface,
            Surface::InverseSurface => self.inverse_surface,
            Surface::SurfaceVariant => self.surface_variant,
            Surface::SurfaceDim => self.surface_dim,
            Surface::SurfaceBright => self.surface_bright,
        }
    }

    pub fn accent(&self, accent: Accent) -> Color {
        match accent {
            Accent::Primary => self.primary,
            Accent::PrimaryFixed => self.primary_fixed,
            Accent::PrimaryFixedDim => self.primary_fixed_dim,
            Accent::InversePrimary => self.inverse_primary,
            Accent::Secondary => self.secondary,
            Accent::SecondaryFixed => self.secondary_fixed,
            Accent::SecondaryFixedDim => self.secondary_fixed_dim,
            Accent::Tertiary => self.tertiary,
            Accent::TertiaryFixed => self.tertiary_fixed,
            Accent::TertiaryFixedDim => self.tertiary_fixed_dim,
            Accent::Error => self.error,
            Accent::Outline => self.outline,
            Accent::OutlineVariant => self.outline_variant,
            Accent::Shadow => self.shadow,
            Accent::Scrim => self.scrim,
        }
    }

    pub fn container(&self, container: Container) -> Color {
        match container {
            Container::Primary => self.primary_container,
            Container::Secondary => self.secondary_container,
            Container::Tertiary => self.tertiary_container,
            Container::Error => self.error_container,
            Container::SurfaceLowest => self.surface_container_lowest,
            Container::SurfaceLow => self.surface_container_low,
            Container::SurfaceMid => self.surface_container,
            Container::SurfaceHigh => self.surface_container_high,
            Container::SurfaceHighest => self.surface_container_highest,
        }
    }

    pub fn on(&self, on: On) -> Color {
        match on {
            On::Primary => self.on_primary,
            On::PrimaryContainer => self.on_primary_container,
            On::PrimaryFixed => self.on_primary_fixed,
            On::PrimaryFixedVariant => self.on_primary_fixed_variant,
            On::Secondary => self.on_secondary,
            On::SecondaryContainer => self.on_secondary_container,
            On::SecondaryFixed => self.on_secondary_fixed,
            On::SecondaryFixedVariant => self.on_secondary_fixed_variant,
            On::Tertiary => self.on_tertiary,
            On::TertiaryContainer => self.on_tertiary_container,
            On::TertiaryFixed => self.on_tertiary_fixed,
            On::TertiaryFixedVariant => self.on_tertiary_fixed_variant,
            On::Error => self.on_error,
            On::ErrorContainer => self.on_error_container,
            On::Background => self.on_background,
            On::Surface => self.on_surface,
            On::SurfaceVariant => self.on_surface_variant,
            On::InverseSurface => self.inverse_on_surface,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct ColorScheme {
    pub colors: SchemeColors,
    pub medium_contrast: SchemeColors,
    pub high_contrast: SchemeColors,
}

impl ColorScheme {
    pub fn contrast(&self, contrast: Contrast) -> SchemeColors {
        match contrast {
            Contrast::Standard => self.colors,
            Contrast::Medium => self.medium_contrast,
            Contrast::High => self.high_contrast,
        }
    }
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct ColorSchemes {
    pub light: ColorScheme,
    pub dark: ColorScheme,
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct ColorPalette {
    pub p_0: Color,
    pub p_5: Color,
    pub p_10: Color,
    pub p_15: Color,
    pub p_20: Color,
    pub p_25: Color,
    pub p_30: Color,
    pub p_35: Color,
    pub p_40: Color,
    pub p_50: Color,
    pub p_60: Color,
    pub p_70: Color,
    pub p_80: Color,
    pub p_90: Color,
    pub p_95: Color,
    pub p_98: Color,
    pub p_99: Color,
    pub p_100: Color,
}

#[derive(Clone, Copy, Debug, Default, Reflect)]
pub struct ColorPalettes {
    pub primary: ColorPalette,
    pub secondary: ColorPalette,
    pub tertiary: ColorPalette,
    pub neutral: ColorPalette,
    pub neutral_variant: ColorPalette,
}

// TODO: write asset loader for theme colors and load it from a material-theme.json
/// Loosly Follows Material3 theme format
#[derive(Clone, Debug, Reflect)]
pub struct ThemeColors {
    pub description: String,
    // TODO: Generate colors from seed & core colors?
    pub seed: Color,
    pub core_colors: CoreColors,
    pub extended_colors: Vec<ExtendedColor>,
    pub schemes: ColorSchemes,
    pub palettes: ColorPalettes,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            description: "Sickle UI Theme".into(),
            seed: Color::hex("EC0A1E").unwrap(),
            core_colors: CoreColors {
                primary: Color::hex("EC0A1E").unwrap(),
                ..default()
            },
            extended_colors: Default::default(),
            schemes: ColorSchemes {
                light: ColorScheme {
                    colors: SchemeColors {
                        primary: Color::hex("904A44").unwrap(),
                        on_primary: Color::hex("FFFFFF").unwrap(),
                        primary_container: Color::hex("FFDAD6").unwrap(),
                        on_primary_container: Color::hex("3B0907").unwrap(),
                        secondary: Color::hex("775653").unwrap(),
                        on_secondary: Color::hex("FFFFFF").unwrap(),
                        secondary_container: Color::hex("FFDAD6").unwrap(),
                        on_secondary_container: Color::hex("2C1513").unwrap(),
                        tertiary: Color::hex("715B2E").unwrap(),
                        on_tertiary: Color::hex("FFFFFF").unwrap(),
                        tertiary_container: Color::hex("FEDFA6").unwrap(),
                        on_tertiary_container: Color::hex("261900").unwrap(),
                        error: Color::hex("BA1A1A").unwrap(),
                        on_error: Color::hex("FFFFFF").unwrap(),
                        error_container: Color::hex("FFDAD6").unwrap(),
                        on_error_container: Color::hex("410002").unwrap(),
                        background: Color::hex("FFF8F7").unwrap(),
                        on_background: Color::hex("231918").unwrap(),
                        surface: Color::hex("FFF8F7").unwrap(),
                        on_surface: Color::hex("231918").unwrap(),
                        surface_variant: Color::hex("F5DDDA").unwrap(),
                        on_surface_variant: Color::hex("534341").unwrap(),
                        outline: Color::hex("857371").unwrap(),
                        outline_variant: Color::hex("D8C2BF").unwrap(),
                        shadow: Color::hex("000000").unwrap(),
                        scrim: Color::hex("000000").unwrap(),
                        inverse_surface: Color::hex("392E2D").unwrap(),
                        inverse_on_surface: Color::hex("FFEDEA").unwrap(),
                        inverse_primary: Color::hex("FFB4AB").unwrap(),
                        primary_fixed: Color::hex("FFDAD6").unwrap(),
                        on_primary_fixed: Color::hex("3B0907").unwrap(),
                        primary_fixed_dim: Color::hex("FFB4AB").unwrap(),
                        on_primary_fixed_variant: Color::hex("73332E").unwrap(),
                        secondary_fixed: Color::hex("FFDAD6").unwrap(),
                        on_secondary_fixed: Color::hex("2C1513").unwrap(),
                        secondary_fixed_dim: Color::hex("E7BDB8").unwrap(),
                        on_secondary_fixed_variant: Color::hex("5D3F3C").unwrap(),
                        tertiary_fixed: Color::hex("FEDFA6").unwrap(),
                        on_tertiary_fixed: Color::hex("261900").unwrap(),
                        tertiary_fixed_dim: Color::hex("E0C38C").unwrap(),
                        on_tertiary_fixed_variant: Color::hex("584419").unwrap(),
                        surface_dim: Color::hex("E8D6D4").unwrap(),
                        surface_bright: Color::hex("FFF8F7").unwrap(),
                        surface_container_lowest: Color::hex("FFFFFF").unwrap(),
                        surface_container_low: Color::hex("FFF0EE").unwrap(),
                        surface_container: Color::hex("FCEAE7").unwrap(),
                        surface_container_high: Color::hex("F6E4E2").unwrap(),
                        surface_container_highest: Color::hex("F1DEDC").unwrap(),
                    },
                    medium_contrast: SchemeColors {
                        primary: Color::hex("6E302A").unwrap(),
                        on_primary: Color::hex("FFFFFF").unwrap(),
                        primary_container: Color::hex("AA6058").unwrap(),
                        on_primary_container: Color::hex("FFFFFF").unwrap(),
                        secondary: Color::hex("593B38").unwrap(),
                        on_secondary: Color::hex("FFFFFF").unwrap(),
                        secondary_container: Color::hex("8F6C68").unwrap(),
                        on_secondary_container: Color::hex("FFFFFF").unwrap(),
                        tertiary: Color::hex("534015").unwrap(),
                        on_tertiary: Color::hex("FFFFFF").unwrap(),
                        tertiary_container: Color::hex("897142").unwrap(),
                        on_tertiary_container: Color::hex("FFFFFF").unwrap(),
                        error: Color::hex("8C0009").unwrap(),
                        on_error: Color::hex("FFFFFF").unwrap(),
                        error_container: Color::hex("DA342E").unwrap(),
                        on_error_container: Color::hex("FFFFFF").unwrap(),
                        background: Color::hex("FFF8F7").unwrap(),
                        on_background: Color::hex("231918").unwrap(),
                        surface: Color::hex("FFF8F7").unwrap(),
                        on_surface: Color::hex("231918").unwrap(),
                        surface_variant: Color::hex("F5DDDA").unwrap(),
                        on_surface_variant: Color::hex("4F3F3D").unwrap(),
                        outline: Color::hex("6C5B59").unwrap(),
                        outline_variant: Color::hex("897674").unwrap(),
                        shadow: Color::hex("000000").unwrap(),
                        scrim: Color::hex("000000").unwrap(),
                        inverse_surface: Color::hex("392E2D").unwrap(),
                        inverse_on_surface: Color::hex("FFEDEA").unwrap(),
                        inverse_primary: Color::hex("FFB4AB").unwrap(),
                        primary_fixed: Color::hex("AA6058").unwrap(),
                        on_primary_fixed: Color::hex("FFFFFF").unwrap(),
                        primary_fixed_dim: Color::hex("8D4841").unwrap(),
                        on_primary_fixed_variant: Color::hex("FFFFFF").unwrap(),
                        secondary_fixed: Color::hex("8F6C68").unwrap(),
                        on_secondary_fixed: Color::hex("FFFFFF").unwrap(),
                        secondary_fixed_dim: Color::hex("745450").unwrap(),
                        on_secondary_fixed_variant: Color::hex("FFFFFF").unwrap(),
                        tertiary_fixed: Color::hex("897142").unwrap(),
                        on_tertiary_fixed: Color::hex("FFFFFF").unwrap(),
                        tertiary_fixed_dim: Color::hex("6F592C").unwrap(),
                        on_tertiary_fixed_variant: Color::hex("FFFFFF").unwrap(),
                        surface_dim: Color::hex("E8D6D4").unwrap(),
                        surface_bright: Color::hex("FFF8F7").unwrap(),
                        surface_container_lowest: Color::hex("FFFFFF").unwrap(),
                        surface_container_low: Color::hex("FFF0EE").unwrap(),
                        surface_container: Color::hex("FCEAE7").unwrap(),
                        surface_container_high: Color::hex("F6E4E2").unwrap(),
                        surface_container_highest: Color::hex("F1DEDC").unwrap(),
                    },
                    high_contrast: SchemeColors {
                        primary: Color::hex("44100D").unwrap(),
                        on_primary: Color::hex("FFFFFF").unwrap(),
                        primary_container: Color::hex("6E302A").unwrap(),
                        on_primary_container: Color::hex("FFFFFF").unwrap(),
                        secondary: Color::hex("341C19").unwrap(),
                        on_secondary: Color::hex("FFFFFF").unwrap(),
                        secondary_container: Color::hex("593B38").unwrap(),
                        on_secondary_container: Color::hex("FFFFFF").unwrap(),
                        tertiary: Color::hex("2E2000").unwrap(),
                        on_tertiary: Color::hex("FFFFFF").unwrap(),
                        tertiary_container: Color::hex("534015").unwrap(),
                        on_tertiary_container: Color::hex("FFFFFF").unwrap(),
                        error: Color::hex("4E0002").unwrap(),
                        on_error: Color::hex("FFFFFF").unwrap(),
                        error_container: Color::hex("8C0009").unwrap(),
                        on_error_container: Color::hex("FFFFFF").unwrap(),
                        background: Color::hex("FFF8F7").unwrap(),
                        on_background: Color::hex("231918").unwrap(),
                        surface: Color::hex("FFF8F7").unwrap(),
                        on_surface: Color::hex("000000").unwrap(),
                        surface_variant: Color::hex("F5DDDA").unwrap(),
                        on_surface_variant: Color::hex("2E211F").unwrap(),
                        outline: Color::hex("4F3F3D").unwrap(),
                        outline_variant: Color::hex("4F3F3D").unwrap(),
                        shadow: Color::hex("000000").unwrap(),
                        scrim: Color::hex("000000").unwrap(),
                        inverse_surface: Color::hex("392E2D").unwrap(),
                        inverse_on_surface: Color::hex("FFFFFF").unwrap(),
                        inverse_primary: Color::hex("FFE7E4").unwrap(),
                        primary_fixed: Color::hex("6E302A").unwrap(),
                        on_primary_fixed: Color::hex("FFFFFF").unwrap(),
                        primary_fixed_dim: Color::hex("521A16").unwrap(),
                        on_primary_fixed_variant: Color::hex("FFFFFF").unwrap(),
                        secondary_fixed: Color::hex("593B38").unwrap(),
                        on_secondary_fixed: Color::hex("FFFFFF").unwrap(),
                        secondary_fixed_dim: Color::hex("402623").unwrap(),
                        on_secondary_fixed_variant: Color::hex("FFFFFF").unwrap(),
                        tertiary_fixed: Color::hex("534015").unwrap(),
                        on_tertiary_fixed: Color::hex("FFFFFF").unwrap(),
                        tertiary_fixed_dim: Color::hex("3B2A02").unwrap(),
                        on_tertiary_fixed_variant: Color::hex("FFFFFF").unwrap(),
                        surface_dim: Color::hex("E8D6D4").unwrap(),
                        surface_bright: Color::hex("FFF8F7").unwrap(),
                        surface_container_lowest: Color::hex("FFFFFF").unwrap(),
                        surface_container_low: Color::hex("FFF0EE").unwrap(),
                        surface_container: Color::hex("FCEAE7").unwrap(),
                        surface_container_high: Color::hex("F6E4E2").unwrap(),
                        surface_container_highest: Color::hex("F1DEDC").unwrap(),
                    },
                },
                dark: ColorScheme {
                    colors: SchemeColors {
                        primary: Color::hex("FFB4AB").unwrap(),
                        on_primary: Color::hex("561E1A").unwrap(),
                        primary_container: Color::hex("73332E").unwrap(),
                        on_primary_container: Color::hex("FFDAD6").unwrap(),
                        secondary: Color::hex("E7BDB8").unwrap(),
                        on_secondary: Color::hex("442927").unwrap(),
                        secondary_container: Color::hex("5D3F3C").unwrap(),
                        on_secondary_container: Color::hex("FFDAD6").unwrap(),
                        tertiary: Color::hex("E0C38C").unwrap(),
                        on_tertiary: Color::hex("3F2E04").unwrap(),
                        tertiary_container: Color::hex("584419").unwrap(),
                        on_tertiary_container: Color::hex("FEDFA6").unwrap(),
                        error: Color::hex("FFB4AB").unwrap(),
                        on_error: Color::hex("690005").unwrap(),
                        error_container: Color::hex("93000A").unwrap(),
                        on_error_container: Color::hex("FFDAD6").unwrap(),
                        background: Color::hex("1A1110").unwrap(),
                        on_background: Color::hex("F1DEDC").unwrap(),
                        surface: Color::hex("1A1110").unwrap(),
                        on_surface: Color::hex("F1DEDC").unwrap(),
                        surface_variant: Color::hex("534341").unwrap(),
                        on_surface_variant: Color::hex("D8C2BF").unwrap(),
                        outline: Color::hex("A08C8A").unwrap(),
                        outline_variant: Color::hex("534341").unwrap(),
                        shadow: Color::hex("000000").unwrap(),
                        scrim: Color::hex("000000").unwrap(),
                        inverse_surface: Color::hex("F1DEDC").unwrap(),
                        inverse_on_surface: Color::hex("392E2D").unwrap(),
                        inverse_primary: Color::hex("904A44").unwrap(),
                        primary_fixed: Color::hex("FFDAD6").unwrap(),
                        on_primary_fixed: Color::hex("3B0907").unwrap(),
                        primary_fixed_dim: Color::hex("FFB4AB").unwrap(),
                        on_primary_fixed_variant: Color::hex("73332E").unwrap(),
                        secondary_fixed: Color::hex("FFDAD6").unwrap(),
                        on_secondary_fixed: Color::hex("2C1513").unwrap(),
                        secondary_fixed_dim: Color::hex("E7BDB8").unwrap(),
                        on_secondary_fixed_variant: Color::hex("5D3F3C").unwrap(),
                        tertiary_fixed: Color::hex("FEDFA6").unwrap(),
                        on_tertiary_fixed: Color::hex("261900").unwrap(),
                        tertiary_fixed_dim: Color::hex("E0C38C").unwrap(),
                        on_tertiary_fixed_variant: Color::hex("584419").unwrap(),
                        surface_dim: Color::hex("15130C").unwrap(),
                        surface_bright: Color::hex("3B3930").unwrap(),
                        surface_container_lowest: Color::hex("140C0B").unwrap(),
                        surface_container_low: Color::hex("231918").unwrap(),
                        surface_container: Color::hex("271D1C").unwrap(),
                        surface_container_high: Color::hex("322826").unwrap(),
                        surface_container_highest: Color::hex("3D3231").unwrap(),
                    },
                    medium_contrast: SchemeColors {
                        primary: Color::hex("FFBAB2").unwrap(),
                        on_primary: Color::hex("330404").unwrap(),
                        primary_container: Color::hex("CC7B73").unwrap(),
                        on_primary_container: Color::hex("000000").unwrap(),
                        secondary: Color::hex("EBC1BC").unwrap(),
                        on_secondary: Color::hex("26100E").unwrap(),
                        secondary_container: Color::hex("AD8884").unwrap(),
                        on_secondary_container: Color::hex("000000").unwrap(),
                        tertiary: Color::hex("E5C790").unwrap(),
                        on_tertiary: Color::hex("1F1400").unwrap(),
                        tertiary_container: Color::hex("A78D5B").unwrap(),
                        on_tertiary_container: Color::hex("000000").unwrap(),
                        error: Color::hex("FFBAB1").unwrap(),
                        on_error: Color::hex("370001").unwrap(),
                        error_container: Color::hex("FF5449").unwrap(),
                        on_error_container: Color::hex("000000").unwrap(),
                        background: Color::hex("1A1110").unwrap(),
                        on_background: Color::hex("F1DEDC").unwrap(),
                        surface: Color::hex("1A1110").unwrap(),
                        on_surface: Color::hex("FFF9F9").unwrap(),
                        surface_variant: Color::hex("534341").unwrap(),
                        on_surface_variant: Color::hex("DCC6C3").unwrap(),
                        outline: Color::hex("B39E9C").unwrap(),
                        outline_variant: Color::hex("927F7D").unwrap(),
                        shadow: Color::hex("000000").unwrap(),
                        scrim: Color::hex("000000").unwrap(),
                        inverse_surface: Color::hex("F1DEDC").unwrap(),
                        inverse_on_surface: Color::hex("322826").unwrap(),
                        inverse_primary: Color::hex("74352F").unwrap(),
                        primary_fixed: Color::hex("FFDAD6").unwrap(),
                        on_primary_fixed: Color::hex("2C0102").unwrap(),
                        primary_fixed_dim: Color::hex("FFB4AB").unwrap(),
                        on_primary_fixed_variant: Color::hex("5E231F").unwrap(),
                        secondary_fixed: Color::hex("FFDAD6").unwrap(),
                        on_secondary_fixed: Color::hex("200B09").unwrap(),
                        secondary_fixed_dim: Color::hex("E7BDB8").unwrap(),
                        on_secondary_fixed_variant: Color::hex("4B2F2C").unwrap(),
                        tertiary_fixed: Color::hex("FEDFA6").unwrap(),
                        on_tertiary_fixed: Color::hex("191000").unwrap(),
                        tertiary_fixed_dim: Color::hex("E0C38C").unwrap(),
                        on_tertiary_fixed_variant: Color::hex("463309").unwrap(),
                        surface_dim: Color::hex("1A1110").unwrap(),
                        surface_bright: Color::hex("423735").unwrap(),
                        surface_container_lowest: Color::hex("140C0B").unwrap(),
                        surface_container_low: Color::hex("231918").unwrap(),
                        surface_container: Color::hex("271D1C").unwrap(),
                        surface_container_high: Color::hex("322826").unwrap(),
                        surface_container_highest: Color::hex("3D3231").unwrap(),
                    },
                    high_contrast: SchemeColors {
                        primary: Color::hex("FFF9F9").unwrap(),
                        on_primary: Color::hex("000000").unwrap(),
                        primary_container: Color::hex("FFBAB2").unwrap(),
                        on_primary_container: Color::hex("000000").unwrap(),
                        secondary: Color::hex("FFF9F9").unwrap(),
                        on_secondary: Color::hex("000000").unwrap(),
                        secondary_container: Color::hex("EBC1BC").unwrap(),
                        on_secondary_container: Color::hex("000000").unwrap(),
                        tertiary: Color::hex("FFFAF7").unwrap(),
                        on_tertiary: Color::hex("000000").unwrap(),
                        tertiary_container: Color::hex("E5C790").unwrap(),
                        on_tertiary_container: Color::hex("000000").unwrap(),
                        error: Color::hex("FFF9F9").unwrap(),
                        on_error: Color::hex("000000").unwrap(),
                        error_container: Color::hex("FFBAB1").unwrap(),
                        on_error_container: Color::hex("000000").unwrap(),
                        background: Color::hex("1A1110").unwrap(),
                        on_background: Color::hex("F1DEDC").unwrap(),
                        surface: Color::hex("1A1110").unwrap(),
                        on_surface: Color::hex("FFFFFF").unwrap(),
                        surface_variant: Color::hex("534341").unwrap(),
                        on_surface_variant: Color::hex("FFF9F9").unwrap(),
                        outline: Color::hex("DCC6C3").unwrap(),
                        outline_variant: Color::hex("DCC6C3").unwrap(),
                        shadow: Color::hex("000000").unwrap(),
                        scrim: Color::hex("000000").unwrap(),
                        inverse_surface: Color::hex("F1DEDC").unwrap(),
                        inverse_on_surface: Color::hex("000000").unwrap(),
                        inverse_primary: Color::hex("4E1714").unwrap(),
                        primary_fixed: Color::hex("FFE0DC").unwrap(),
                        on_primary_fixed: Color::hex("000000").unwrap(),
                        primary_fixed_dim: Color::hex("FFBAB2").unwrap(),
                        on_primary_fixed_variant: Color::hex("330404").unwrap(),
                        secondary_fixed: Color::hex("FFE0DC").unwrap(),
                        on_secondary_fixed: Color::hex("000000").unwrap(),
                        secondary_fixed_dim: Color::hex("EBC1BC").unwrap(),
                        on_secondary_fixed_variant: Color::hex("26100E").unwrap(),
                        tertiary_fixed: Color::hex("FFE3B2").unwrap(),
                        on_tertiary_fixed: Color::hex("000000").unwrap(),
                        tertiary_fixed_dim: Color::hex("E5C790").unwrap(),
                        on_tertiary_fixed_variant: Color::hex("1F1400").unwrap(),
                        surface_dim: Color::hex("1A1110").unwrap(),
                        surface_bright: Color::hex("423735").unwrap(),
                        surface_container_lowest: Color::hex("140C0B").unwrap(),
                        surface_container_low: Color::hex("231918").unwrap(),
                        surface_container: Color::hex("271D1C").unwrap(),
                        surface_container_high: Color::hex("322826").unwrap(),
                        surface_container_highest: Color::hex("3D3231").unwrap(),
                    },
                },
            },
            palettes: ColorPalettes {
                primary: ColorPalette {
                    p_0: Color::hex("000000").unwrap(),
                    p_5: Color::hex("2D0001").unwrap(),
                    p_10: Color::hex("410002").unwrap(),
                    p_15: Color::hex("540004").unwrap(),
                    p_20: Color::hex("690006").unwrap(),
                    p_25: Color::hex("7D0009").unwrap(),
                    p_30: Color::hex("93000D").unwrap(),
                    p_35: Color::hex("A90010").unwrap(),
                    p_40: Color::hex("C00014").unwrap(),
                    p_50: Color::hex("ED0C1F").unwrap(),
                    p_60: Color::hex("FF544B").unwrap(),
                    p_70: Color::hex("FF897E").unwrap(),
                    p_80: Color::hex("FFB4AB").unwrap(),
                    p_90: Color::hex("FFDAD6").unwrap(),
                    p_95: Color::hex("FFEDEA").unwrap(),
                    p_98: Color::hex("FFF8F7").unwrap(),
                    p_99: Color::hex("FFFBFF").unwrap(),
                    p_100: Color::hex("FFFFFF").unwrap(),
                },
                secondary: ColorPalette {
                    p_0: Color::hex("000000").unwrap(),
                    p_5: Color::hex("2B0202").unwrap(),
                    p_10: Color::hex("3A0A08").unwrap(),
                    p_15: Color::hex("481411").unwrap(),
                    p_20: Color::hex("551F1A").unwrap(),
                    p_25: Color::hex("632924").unwrap(),
                    p_30: Color::hex("72342F").unwrap(),
                    p_35: Color::hex("80403A").unwrap(),
                    p_40: Color::hex("8E4B45").unwrap(),
                    p_50: Color::hex("AC635C").unwrap(),
                    p_60: Color::hex("CA7C74").unwrap(),
                    p_70: Color::hex("E8958D").unwrap(),
                    p_80: Color::hex("FFB4AB").unwrap(),
                    p_90: Color::hex("FFDAD6").unwrap(),
                    p_95: Color::hex("FFEDEA").unwrap(),
                    p_98: Color::hex("FFF8F7").unwrap(),
                    p_99: Color::hex("FFFBFF").unwrap(),
                    p_100: Color::hex("FFFFFF").unwrap(),
                },
                tertiary: ColorPalette {
                    p_0: Color::hex("000000").unwrap(),
                    p_5: Color::hex("190F00").unwrap(),
                    p_10: Color::hex("261900").unwrap(),
                    p_15: Color::hex("332300").unwrap(),
                    p_20: Color::hex("402D00").unwrap(),
                    p_25: Color::hex("4E3800").unwrap(),
                    p_30: Color::hex("5C4200").unwrap(),
                    p_35: Color::hex("6B4D00").unwrap(),
                    p_40: Color::hex("7A5900").unwrap(),
                    p_50: Color::hex("997000").unwrap(),
                    p_60: Color::hex("B98900").unwrap(),
                    p_70: Color::hex("D9A21B").unwrap(),
                    p_80: Color::hex("F7BD39").unwrap(),
                    p_90: Color::hex("FFDEA2").unwrap(),
                    p_95: Color::hex("FFEFD5").unwrap(),
                    p_98: Color::hex("FFF8F2").unwrap(),
                    p_99: Color::hex("FFFBFF").unwrap(),
                    p_100: Color::hex("FFFFFF").unwrap(),
                },
                neutral: ColorPalette {
                    p_0: Color::hex("000000").unwrap(),
                    p_5: Color::hex("15100F").unwrap(),
                    p_10: Color::hex("201A19").unwrap(),
                    p_15: Color::hex("2B2423").unwrap(),
                    p_20: Color::hex("362F2E").unwrap(),
                    p_25: Color::hex("413A39").unwrap(),
                    p_30: Color::hex("4D4544").unwrap(),
                    p_35: Color::hex("59504F").unwrap(),
                    p_40: Color::hex("655C5B").unwrap(),
                    p_50: Color::hex("7F7574").unwrap(),
                    p_60: Color::hex("998E8D").unwrap(),
                    p_70: Color::hex("B4A9A7").unwrap(),
                    p_80: Color::hex("D0C4C2").unwrap(),
                    p_90: Color::hex("EDE0DE").unwrap(),
                    p_95: Color::hex("FBEEEC").unwrap(),
                    p_98: Color::hex("FFF8F7").unwrap(),
                    p_99: Color::hex("FFFBFF").unwrap(),
                    p_100: Color::hex("FFFFFF").unwrap(),
                },
                neutral_variant: ColorPalette {
                    p_0: Color::hex("000000").unwrap(),
                    p_5: Color::hex("190E0D").unwrap(),
                    p_10: Color::hex("251917").unwrap(),
                    p_15: Color::hex("302321").unwrap(),
                    p_20: Color::hex("3B2D2B").unwrap(),
                    p_25: Color::hex("473836").unwrap(),
                    p_30: Color::hex("534341").unwrap(),
                    p_35: Color::hex("5F4F4D").unwrap(),
                    p_40: Color::hex("6B5A58").unwrap(),
                    p_50: Color::hex("857371").unwrap(),
                    p_60: Color::hex("A08C8A").unwrap(),
                    p_70: Color::hex("BBA6A4").unwrap(),
                    p_80: Color::hex("D8C2BF").unwrap(),
                    p_90: Color::hex("F5DDDA").unwrap(),
                    p_95: Color::hex("FFEDEA").unwrap(),
                    p_98: Color::hex("FFF8F7").unwrap(),
                    p_99: Color::hex("FFFBFF").unwrap(),
                    p_100: Color::hex("FFFFFF").unwrap(),
                },
            },
        }
    }
}
