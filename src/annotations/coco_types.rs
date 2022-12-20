use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Dataset {
    pub images: Vec<Image>,
    pub annotations: Vec<Annotation>,
    pub categories: Vec<Category>,
}

#[derive(Deserialize, Debug)]
pub struct Image {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
}

#[derive(Deserialize, Debug)]
pub struct Annotation {
    pub id: u32,
    pub image_id: u32,
    pub category_id: u32,
    /// Segmentation can be a polygon, RLE or encoded RLE.
    /// Exemple of polygon: "segmentation": [[510.66,423.01,511.72,420.03,...,510.45,423.01]]
    /// Exemple of RLE: "segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, 35, 5, 35, 5, 35, 5, 1190]}
    /// Exemple of encoded RLE: "segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}
    pub segmentation: Segmentation,
    pub area: f64,
    /// The COCO bounding box format is [top left x position, top left y position, width, height].
    /// bbox exemple:  "bbox": [473.07,395.93,38.65,28.67]
    pub bbox: Bbox,
    /// Either 1 or 0
    pub iscrowd: u32,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Segmentation {
    // Move { x: i32, y: i32 },
    Polygon(Vec<Vec<f64>>),
    RLE(RLE),
    EncodedRLE(EncodedRLE),
}

#[derive(Deserialize, Debug)]
pub struct RLE {
    pub size: Vec<u32>,
    pub counts: Vec<u32>,
}

#[derive(Deserialize, Debug)]
pub struct EncodedRLE {
    pub size: Vec<u32>,
    pub counts: String,
}

#[derive(Deserialize, Debug)]
pub struct Bbox {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Deserialize, Debug)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub supercategory: String,
}
