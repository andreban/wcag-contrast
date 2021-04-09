/*
 * Copyright 2019 André Cipriani Bandarra
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

//! This crate implements the WCAG specification for contrast ratio and relative luminance.
//! Read more about WCAG at [https://www.w3.org/TR/WCAG20/](https://www.w3.org/TR/WCAG20/).

use std::str::FromStr;

///
/// A representation for a color with the red, green and blue channels
///
#[derive(Debug, PartialOrd, PartialEq)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    ///
    /// Creates a new [Color].
    /// ```rust
    /// use wcagcontrast::Color;
    /// use std::str::FromStr;
    ///
    /// let color = Color::new(255, 255, 255);
    /// assert_eq!(color.rgb(), (255, 255, 255));
    /// ```
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color {r, g, b}
    }

    ///
    /// Generates an ([u8], [u8], [u8]) tuple from the [Color] with the red, green, and blue channels.
    /// ```rust
    /// use wcagcontrast::Color;
    /// use std::str::FromStr;
    ///
    /// let color = Color::from_str("#FFFFFF").unwrap();
    /// assert_eq!(color.rgb(), (255, 255, 255));
    /// ```
    pub fn rgb(&self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    ///
    /// Calculates the relative luminance, as described on
    /// [https://www.w3.org/TR/WCAG20/#relativeluminancedef](https://www.w3.org/TR/WCAG20/#relativeluminancedef)
    ///
    /// ```rust
    /// use wcagcontrast::Color;
    ///
    /// let black = Color::new(0, 0, 0);
    /// let white = Color::new(255, 255, 255);
    /// assert_eq!(white.relative_luminance(), 1.0);
    /// assert_eq!(black.relative_luminance(), 0.0);
    /// ```
    ///
    pub fn relative_luminance(&self) -> f64 {
        let red_luminance = Color::component_relative_luminance(self.r);
        let green_luminance = Color::component_relative_luminance(self.g);
        let blue_luminance = Color::component_relative_luminance(self.b);

        0.2126 * red_luminance + 0.7152 * green_luminance + 0.0722 * blue_luminance
    }

    ///
    /// Calculates the contrast ratio, as described on
    /// [https://www.w3.org/TR/WCAG20/#contrast-ratiodef](https://www.w3.org/TR/WCAG20/#contrast-ratiodef)
    ///
    /// ```rust
    /// use wcagcontrast::Color;
    ///
    /// let black = Color::new(0, 0, 0);
    /// let white = Color::new(255, 255, 255);
    /// assert_eq!(black.contrast_ratio(&white), 21.0);
    /// assert_eq!(black.contrast_ratio(&black), 1.0);
    /// assert_eq!(white.contrast_ratio(&white), 1.0);
    /// ```
    ///
    pub fn contrast_ratio(&self, other: &Color) -> f64 {
        let self_luminance = self.relative_luminance();
        let other_luminance = other.relative_luminance();

        let (lighter, darker) = if self_luminance > other_luminance {
            (self_luminance, other_luminance)
        } else {
            (other_luminance, self_luminance)
        };

        (lighter + 0.05) / (darker + 0.05)
    }

    ///
    /// Calculates the luminance of a single color component.
    ///
    fn component_relative_luminance(color_component: u8) -> f64 {
        let c = color_component as f64 / 255.0;

        if c <= 0.03928 {
            c / 12.92
        } else {
            f64::powf((c + 0.055) / 1.055, 2.4)
        }
    }
}

impl FromStr for Color {
    type Err = std::num::ParseIntError;

    /// ```rust
    /// use wcagcontrast::Color;
    /// use std::str::FromStr;
    ///
    /// let color = Color::from_str("#FFFFFF").unwrap();
    /// assert_eq!(color.rgb(), (255, 255, 255));
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = u8::from_str_radix(&s[1 .. 3], 16)?;
        let g = u8::from_str_radix(&s[3 .. 5], 16)?;
        let b = u8::from_str_radix(&s[5 .. 7], 16)?;
        Ok(Color::new(r, g, b))
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(tuple: (u8, u8, u8)) -> Self {
        Color::new(tuple.0, tuple.1, tuple.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn color_from_hex() {
        assert_eq!(
            Color::from_str("#000000").unwrap(),
            Color::new(0, 0, 0)
        );

        assert_eq!(
            Color::from_str("#FFFFFF").unwrap(),
            Color::new(255, 255, 255)
        );

        assert_eq!(
            Color::from_str("#ffffff").unwrap(),
            Color::new(255, 255, 255)
        );
    }

    #[test]
    fn calculates_correct_ratio() {
        assert_eq!(
            Color::new(0, 0, 0)
                .contrast_ratio(&Color::new(255, 255, 255)),
            21.0
        );

        assert_eq!(
            Color::new(255, 255, 255)
                .contrast_ratio(&Color::new(0, 0, 0)),
            21.0
        );

        assert_eq!(
            Color::new(255, 255, 255)
                .contrast_ratio(&Color::new(255, 255, 255)),
            1.0
        );
    }

    #[test]
    fn calculates_relative_luminance() {
        assert_eq!(Color::new(255, 255, 255).relative_luminance(), 1.0);
        assert_eq!(Color::new(0, 0, 0).relative_luminance(), 0.0);
    }
}
