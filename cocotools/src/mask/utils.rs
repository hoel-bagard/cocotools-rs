use crate::coco::object_detection::{Bbox, CocoRle, Polygons, PolygonsRS, Rle};
use crate::mask::conversions::mask_from_poly;
use std::cmp;

pub trait Area {
    fn area(&self) -> u32;
}

impl Area for Rle {
    fn area(&self) -> u32 {
        self.counts[1..].iter().step_by(2).sum()
    }
}

impl Area for CocoRle {
    fn area(&self) -> u32 {
        let rle = Rle::from(self);
        rle.counts[1..].iter().step_by(2).sum()
    }
}

#[allow(clippy::unwrap_used)]
impl Area for PolygonsRS {
    fn area(&self) -> u32 {
        let rle = Rle::try_from(self).unwrap();
        rle.counts[1..].iter().step_by(2).sum()
    }
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::unwrap_used
)]
impl Area for Polygons {
    fn area(&self) -> u32 {
        if self.len() <= 2 {
            return 0;
        }

        // TODO: https://en.wikipedia.org/wiki/Shoelace_formula
        let width = *self
            .iter()
            .map(|x| x.iter().step_by(2).max_by(|a, b| a.total_cmp(b)).unwrap())
            .max_by(|a, b| a.total_cmp(b))
            .unwrap() as u32;
        let height = *self
            .iter()
            .map(|x| {
                x.iter()
                    .take(1)
                    .step_by(2)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap()
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap() as u32;
        let mask = mask_from_poly(self, width, height).unwrap();
        let rle = Rle::from(&mask);
        rle.counts[1..].iter().step_by(2).sum()
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<&Rle> for Bbox {
    fn from(rle: &Rle) -> Self {
        if rle.counts.len() <= 1 {
            return Self {
                left: 0.0,
                top: 0.0,
                width: 0.0,
                height: 0.0,
            };
        }
        let height = rle.size[0];
        let width = rle.size[1];
        let mut pos: u32 = 0;
        let mut current: u32;
        let mut left: u32 = 0;
        let mut right: u32 = 0;
        let mut top = width;
        let mut bot: u32 = 0;
        for (i, count) in rle.counts[..rle.counts.len() - 1].iter().enumerate() {
            pos += count;
            current = pos - ((i % 2) as u32); // Do not count "current" pixel when adding mask pixels.

            // In the lines below, left/right and top/bot might seem inverted, that's because the RLE corresponds to a fortran array.
            if i == 0 {
                left = current / height;
            } else if i == rle.counts.len() - 2 {
                right = current / height;
            }

            let y = current % height;
            top = cmp::min(top, y);
            bot = cmp::max(bot, y);
        }

        Self {
            left: f64::from(left),
            top: f64::from(top),
            width: f64::from(right - left),
            height: f64::from(bot - top),
        }
    }
}

impl From<&CocoRle> for Bbox {
    fn from(coco_rle: &CocoRle) -> Self {
        let rle = Rle::from(coco_rle);
        Self::from(&rle)
    }
}

impl From<&PolygonsRS> for Bbox {
    fn from(poly: &PolygonsRS) -> Self {
        let left: f64 = *poly
            .counts
            .iter()
            .map(|x| {
                x.iter()
                    .step_by(2)
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        let right = *poly
            .counts
            .iter()
            .map(|x| {
                x.iter()
                    .step_by(2)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        let top = *poly
            .counts
            .iter()
            .map(|x| {
                x[1..]
                    .iter()
                    .step_by(2)
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        let bot = *poly
            .counts
            .iter()
            .map(|x| {
                x[1..]
                    .iter()
                    .step_by(2)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        Self {
            left,
            top,
            width: right - left,
            height: bot - top,
        }
    }
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
impl From<&Polygons> for Bbox {
    fn from(poly: &Polygons) -> Self {
        let left: f64 = *poly
            .iter()
            .map(|x| {
                x.iter()
                    .step_by(2)
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        let right = *poly
            .iter()
            .map(|x| {
                x.iter()
                    .step_by(2)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        let top = *poly
            .iter()
            .map(|x| {
                x[1..]
                    .iter()
                    .step_by(2)
                    .min_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .min_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        let bot = *poly
            .iter()
            .map(|x| {
                x[1..]
                    .iter()
                    .step_by(2)
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            })
            .max_by(|a, b| a.total_cmp(b))
            .unwrap_or(&0.0);

        Self {
            left,
            top,
            width: right - left,
            height: bot - top,
        }
    }
}
