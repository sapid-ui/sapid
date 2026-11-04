//! Finite sRGB HSV coordinates; no parser/runtime dependency or visited history.
use glory::style::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hsva {
    /// Degrees in 0..=360. Both endpoints represent red.
    pub hue: f64,
    pub saturation: f64,
    pub value: f64,
    pub alpha: f64,
}
impl Default for Hsva {
    fn default() -> Self {
        Self {
            hue: 0.,
            saturation: 1.,
            value: 1.,
            alpha: 1.,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ColorError {
    NonFinite,
    HueOutOfRange,
    UnitOutOfRange,
    InvalidHex,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ColorFormat {
    #[default]
    Hex,
    Hexa,
    Rgb,
    Rgba,
    Hsl,
    Hsla,
}
impl ColorFormat {
    pub const fn alpha(self) -> bool {
        matches!(self, Self::Hexa | Self::Rgba | Self::Hsla)
    }
}
impl std::fmt::Display for ColorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::NonFinite => "color coordinates must be finite",
            Self::HueOutOfRange => "hue requires 0..=360 degrees",
            Self::UnitOutOfRange => "saturation, value and alpha require 0..=1",
            Self::InvalidHex => "color requires #rgb, #rgba, #rrggbb or #rrggbbaa",
        })
    }
}
impl std::error::Error for ColorError {}
impl Hsva {
    pub fn validate(self) -> Result<(), ColorError> {
        if [self.hue, self.saturation, self.value, self.alpha]
            .into_iter()
            .any(|v| !v.is_finite())
        {
            return Err(ColorError::NonFinite);
        }
        if !(0. ..=360.).contains(&self.hue) {
            return Err(ColorError::HueOutOfRange);
        }
        if [self.saturation, self.value, self.alpha]
            .into_iter()
            .any(|v| !(0. ..=1.).contains(&v))
        {
            return Err(ColorError::UnitOutOfRange);
        }
        Ok(())
    }
    /// Quantizes only the rendered RGBA8 preview. The original HSV source stays f64.
    pub fn to_rgba(self) -> Result<Color, ColorError> {
        self.validate()?;
        let h = self.hue.rem_euclid(360.) / 60.;
        let c = self.value * self.saturation;
        let x = c * (1. - ((h % 2.) - 1.).abs());
        let (r, g, b) = if h < 1. {
            (c, x, 0.)
        } else if h < 2. {
            (x, c, 0.)
        } else if h < 3. {
            (0., c, x)
        } else if h < 4. {
            (0., x, c)
        } else if h < 5. {
            (x, 0., c)
        } else {
            (c, 0., x)
        };
        let m = self.value - c;
        let byte = |v: f64| (v * 255.).round().clamp(0., 255.) as u8;
        Ok(Color(
            byte(r + m),
            byte(g + m),
            byte(b + m),
            byte(self.alpha),
        ))
    }
    /// Achromatic input has hue zero. A picker retaining an achromatic hue should
    /// keep its HSV model rather than repeatedly decode the rendered preview.
    pub fn from_rgba(Color(r, g, b, a): Color) -> Self {
        let (r, g, b) = (
            f64::from(r) / 255.,
            f64::from(g) / 255.,
            f64::from(b) / 255.,
        );
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;
        let hue = if delta == 0. {
            0.
        } else if max == r {
            60. * ((g - b) / delta).rem_euclid(6.)
        } else if max == g {
            60. * ((b - r) / delta + 2.)
        } else {
            60. * ((r - g) / delta + 4.)
        };
        Self {
            hue,
            saturation: if max == 0. { 0. } else { delta / max },
            value: max,
            alpha: f64::from(a) / 255.,
        }
    }
}
pub fn parse_hex_color(value: &str) -> Result<Color, ColorError> {
    // Reject before scanning or allocating, including oversized program drafts.
    if !matches!(value.len(), 4 | 5 | 7 | 9) || value.as_bytes()[0] != b'#' {
        return Err(ColorError::InvalidHex);
    }
    fn digit(byte: u8) -> Result<u8, ColorError> {
        match byte {
            b'0'..=b'9' => Ok(byte - b'0'),
            b'a'..=b'f' => Ok(byte - b'a' + 10),
            b'A'..=b'F' => Ok(byte - b'A' + 10),
            _ => Err(ColorError::InvalidHex),
        }
    }
    let bytes = &value.as_bytes()[1..];
    let mut rgba = [0, 0, 0, 255];
    if bytes.len() <= 4 {
        for (index, byte) in bytes.iter().copied().enumerate() {
            rgba[index] = digit(byte)? * 17
        }
    } else {
        for (index, pair) in bytes.as_chunks::<2>().0.iter().enumerate() {
            rgba[index] = digit(pair[0])? * 16 + digit(pair[1])?
        }
    }
    Ok(Color(rgba[0], rgba[1], rgba[2], rgba[3]))
}
pub fn format_hex_color(Color(r, g, b, a): Color, include_alpha: bool) -> String {
    if include_alpha {
        format!("#{r:02x}{g:02x}{b:02x}{a:02x}")
    } else {
        format!("#{r:02x}{g:02x}{b:02x}")
    }
}

fn arguments<'a>(value: &'a str, name: &str) -> Option<Vec<&'a str>> {
    let body = value
        .strip_prefix(name)?
        .strip_prefix('(')?
        .strip_suffix(')')?;
    Some(body.split(',').map(str::trim).collect())
}
fn byte(value: &str) -> Result<u8, ColorError> {
    value.parse().map_err(|_| ColorError::InvalidHex)
}
fn unit(value: &str) -> Result<f64, ColorError> {
    let value: f64 = value.parse().map_err(|_| ColorError::InvalidHex)?;
    if (0. ..=1.).contains(&value) {
        Ok(value)
    } else {
        Err(ColorError::InvalidHex)
    }
}
fn percent(value: &str) -> Result<f64, ColorError> {
    let value = value
        .strip_suffix('%')
        .ok_or(ColorError::InvalidHex)?
        .parse::<f64>()
        .map_err(|_| ColorError::InvalidHex)?
        / 100.;
    if (0. ..=1.).contains(&value) {
        Ok(value)
    } else {
        Err(ColorError::InvalidHex)
    }
}
pub fn parse_color(value: &str, format: ColorFormat) -> Result<Color, ColorError> {
    match format {
        ColorFormat::Hex | ColorFormat::Hexa => parse_hex_color(value).and_then(|color| {
            let has_alpha = matches!(value.len(), 5 | 9);
            if has_alpha == format.alpha() {
                Ok(color)
            } else {
                Err(ColorError::InvalidHex)
            }
        }),
        ColorFormat::Rgb | ColorFormat::Rgba => {
            let args = arguments(value, if format.alpha() { "rgba" } else { "rgb" })
                .ok_or(ColorError::InvalidHex)?;
            if args.len() != if format.alpha() { 4 } else { 3 } {
                return Err(ColorError::InvalidHex);
            }
            Ok(Color(
                byte(args[0])?,
                byte(args[1])?,
                byte(args[2])?,
                if format.alpha() {
                    (unit(args[3])? * 255.).round() as u8
                } else {
                    255
                },
            ))
        }
        ColorFormat::Hsl | ColorFormat::Hsla => {
            let args = arguments(value, if format.alpha() { "hsla" } else { "hsl" })
                .ok_or(ColorError::InvalidHex)?;
            if args.len() != if format.alpha() { 4 } else { 3 } {
                return Err(ColorError::InvalidHex);
            }
            let hue: f64 = args[0].parse().map_err(|_| ColorError::InvalidHex)?;
            if !hue.is_finite() {
                return Err(ColorError::InvalidHex);
            }
            let saturation = percent(args[1])?;
            let lightness = percent(args[2])?;
            let c = (1. - (2. * lightness - 1.).abs()) * saturation;
            let hp = hue.rem_euclid(360.) / 60.;
            let x = c * (1. - (hp % 2. - 1.).abs());
            let (r, g, b) = if hp < 1. {
                (c, x, 0.)
            } else if hp < 2. {
                (x, c, 0.)
            } else if hp < 3. {
                (0., c, x)
            } else if hp < 4. {
                (0., x, c)
            } else if hp < 5. {
                (x, 0., c)
            } else {
                (c, 0., x)
            };
            let m = lightness - c / 2.;
            let q = |v: f64| ((v + m) * 255.).round() as u8;
            Ok(Color(
                q(r),
                q(g),
                q(b),
                if format.alpha() {
                    (unit(args[3])? * 255.).round() as u8
                } else {
                    255
                },
            ))
        }
    }
}
pub fn format_color(color: Color, format: ColorFormat) -> String {
    let Color(r, g, b, a) = color;
    match format {
        ColorFormat::Hex => format_hex_color(color, false),
        ColorFormat::Hexa => format_hex_color(color, true),
        ColorFormat::Rgb => format!("rgb({r}, {g}, {b})"),
        ColorFormat::Rgba => format!("rgba({r}, {g}, {b}, {:.2})", f64::from(a) / 255.),
        ColorFormat::Hsl | ColorFormat::Hsla => {
            let hsv = Hsva::from_rgba(color);
            let lightness = hsv.value * (1. - hsv.saturation / 2.);
            let saturation = if lightness == 0. || lightness == 1. {
                0.
            } else {
                (hsv.value - lightness) / lightness.min(1. - lightness)
            };
            if format.alpha() {
                format!(
                    "hsla({:.0}, {:.0}%, {:.0}%, {:.2})",
                    hsv.hue,
                    saturation * 100.,
                    lightness * 100.,
                    f64::from(a) / 255.
                )
            } else {
                format!(
                    "hsl({:.0}, {:.0}%, {:.0}%)",
                    hsv.hue,
                    saturation * 100.,
                    lightness * 100.
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn primary_endpoints_achromatic_values_and_alpha_are_exact() {
        for (hue, color) in [
            (0., Color(255, 0, 0, 255)),
            (60., Color(255, 255, 0, 255)),
            (120., Color(0, 255, 0, 255)),
            (180., Color(0, 255, 255, 255)),
            (240., Color(0, 0, 255, 255)),
            (300., Color(255, 0, 255, 255)),
            (360., Color(255, 0, 0, 255)),
        ] {
            assert_eq!(
                Hsva {
                    hue,
                    ..Hsva::default()
                }
                .to_rgba()
                .unwrap(),
                color
            )
        }
        assert_eq!(
            Hsva {
                saturation: 0.,
                value: 0.5,
                alpha: 0.5,
                ..Hsva::default()
            }
            .to_rgba()
            .unwrap(),
            Color(128, 128, 128, 128)
        );
        assert_eq!(
            Hsva {
                value: 0.,
                alpha: 0.,
                ..Hsva::default()
            }
            .to_rgba()
            .unwrap(),
            Color(0, 0, 0, 0)
        );
    }
    #[test]
    fn rgba_grid_round_trips_without_losing_channels_including_transparent_rgb() {
        let levels = [0, 1, 17, 127, 128, 254, 255];
        for r in levels {
            for g in levels {
                for b in levels {
                    for a in levels {
                        let color = Color(r, g, b, a);
                        let hsv = Hsva::from_rgba(color);
                        hsv.validate().unwrap();
                        assert_eq!(hsv.to_rgba().unwrap(), color)
                    }
                }
            }
        }
    }
    #[test]
    fn source_precision_and_achromatic_hue_are_explicit() {
        let value = Hsva {
            hue: 123.456789012345,
            saturation: 0.123456789,
            value: 0.87654321,
            alpha: 0.543210987,
        };
        let _ = value.to_rgba().unwrap();
        assert_eq!(value.hue, 123.456789012345);
        assert_eq!(Hsva::from_rgba(Color(70, 70, 70, 0)).hue, 0.);
        assert_eq!(
            Hsva::from_rgba(Color(255, 0, 128, 255)).hue,
            329.88235294117646
        );
    }
    #[test]
    fn nonfinite_and_out_of_domain_coordinates_are_rejected_independently() {
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            for index in 0..4 {
                let mut value = Hsva::default();
                match index {
                    0 => value.hue = bad,
                    1 => value.saturation = bad,
                    2 => value.value = bad,
                    _ => value.alpha = bad,
                };
                assert_eq!(value.to_rgba(), Err(ColorError::NonFinite))
            }
        }
        for hue in [-1., 360.0001] {
            assert_eq!(
                Hsva {
                    hue,
                    ..Hsva::default()
                }
                .to_rgba(),
                Err(ColorError::HueOutOfRange)
            )
        }
        for bad in [-0.001, 1.001] {
            for index in 0..3 {
                let mut value = Hsva::default();
                match index {
                    0 => value.saturation = bad,
                    1 => value.value = bad,
                    _ => value.alpha = bad,
                };
                assert_eq!(value.to_rgba(), Err(ColorError::UnitOutOfRange))
            }
        }
    }
    #[test]
    fn hex_forms_round_trip_and_keep_alpha() {
        for (input, color) in [
            ("#f0a", Color(255, 0, 170, 255)),
            ("#f0a8", Color(255, 0, 170, 136)),
            ("#A0b1C2", Color(160, 177, 194, 255)),
            ("#abcdef01", Color(171, 205, 239, 1)),
        ] {
            assert_eq!(parse_hex_color(input), Ok(color));
            assert_eq!(parse_hex_color(&format_hex_color(color, true)), Ok(color));
        }
        assert_eq!(format_hex_color(Color(1, 2, 3, 4), false), "#010203");
    }
    #[test]
    fn malformed_hex_and_oversized_input_are_refused_without_normalizing() {
        for input in [
            "",
            "red",
            " #fff",
            "#fff ",
            "#12",
            "#12345",
            "#fffffG",
            "#🦀a",
            "#ff\0",
            "#00000000 #fff",
        ] {
            assert_eq!(
                parse_hex_color(input),
                Err(ColorError::InvalidHex),
                "{input:?}"
            )
        }
        assert_eq!(
            parse_hex_color(&"f".repeat(1_000_000)),
            Err(ColorError::InvalidHex)
        );
    }
    #[test]
    fn public_formats_parse_and_emit_canonical_colors() {
        for (format, input, color) in [
            (ColorFormat::Hex, "#228be6", Color(34, 139, 230, 255)),
            (ColorFormat::Hexa, "#228be680", Color(34, 139, 230, 128)),
            (
                ColorFormat::Rgb,
                "rgb(34, 139, 230)",
                Color(34, 139, 230, 255),
            ),
            (
                ColorFormat::Rgba,
                "rgba(34, 139, 230, 0.5)",
                Color(34, 139, 230, 128),
            ),
            (
                ColorFormat::Hsl,
                "hsl(208, 80%, 52%)",
                Color(35, 139, 231, 255),
            ),
            (
                ColorFormat::Hsla,
                "hsla(208, 80%, 52%, 0.5)",
                Color(35, 139, 231, 128),
            ),
        ] {
            assert_eq!(parse_color(input, format), Ok(color));
            assert!(parse_color(&format_color(color, format), format).is_ok());
        }
        assert!(parse_color("#228be680", ColorFormat::Hex).is_err());
        assert!(parse_color("rgba(1, 2, 3, 2)", ColorFormat::Rgba).is_err());
    }
}
