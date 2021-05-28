use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FlipDirection {
    Horizontal,
    Vertical,
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Crop {
    /// Number of pixels to crop from the top.
    pub top: usize,

    /// Number of pixels to crop from the bottom.
    pub bottom: usize,

    /// Number of pixels to crop from the left.
    pub left: usize,

    /// Number of pixels to crop from the right.
    pub right: usize,
}

impl FromStr for Crop {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split(':').collect::<Vec<_>>()[..] {
            [left, right, top, bottom] => {
                match (left.parse(), right.parse(), top.parse(), bottom.parse()) {
                    (Ok(left), Ok(right), Ok(top), Ok(bottom)) => Ok(Crop {
                        top,
                        bottom,
                        left,
                        right,
                    }),
                    _ => Err(format!("Failed to parse crop region string: {}", s)),
                }
            }
            _ => Err(format!("Bad resolution format: {}", s)),
        }
    }
}

impl Serialize for Crop {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let s = format!("{}:{}:{}:{}", self.left, self.right, self.top, self.bottom);

        serializer.serialize_str(&s)
    }
}

impl<'de> Deserialize<'de> for Crop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        Crop::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[skip_serializing_none]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransformProperties {
    /// Framerate.
    #[serde(alias = "fps")]
    pub framerate: Option<u32>,

    /// The desired resolution.
    pub resolution: Option<crate::Resolution>,

    /// Rotation angle in degrees.
    pub rotation: Option<f64>,

    /// Flip direction.
    pub flip_direction: Option<FlipDirection>,

    /// Crop region.
    #[serde(alias = "crop")]
    pub crop_region: Option<Crop>,
}

#[cfg(test)]
mod test {
    use super::{Crop, FlipDirection, TransformProperties};
    use serde_json::{from_str, to_string};

    #[test]
    fn transform() {
        let t = TransformProperties {
            framerate: Some(15),
            resolution: Some(crate::Resolution {
                width: 640,
                height: 480,
            }),
            rotation: Some(88_f64),
            flip_direction: Some(FlipDirection::Vertical),
            crop_region: Some(Crop {
                top: 51,
                bottom: 49,
                left: 9,
                right: 1,
            }),
        };

        let s = to_string(&t).unwrap();
        assert_eq!(t, from_str(&s).unwrap());
    }
}
