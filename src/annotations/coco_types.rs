pub struct Dataset {
    pub images: Vec<Image>,
    pub annotatons: Vec<Annotation>,
    pub categories: Vec<Category>,
}

pub struct Image {
    id: u32,
    width: u32,
    height: u32,
    file_name: String,
}

pub struct Annotation {
    id: u32,
    image_id: u32,
    category_id: u32,
    /// Segmentation can be a polygon, RLE or encoded RLE.
    /// Exemple of polygon: "segmentation": [[510.66,423.01,511.72,420.03,...,510.45,423.01]]
    /// Exemple of RLE: "segmentation": {"size": [40, 40], "counts": [245, 5, 35, 5, 35, 5, 35, 5, 35, 5, 1190]}
    /// Exemple of encoded RLE: "segmentation": {"size": [480, 640], "counts": "aUh2b0X...BgRU4"}
    segmentation: Segmentation,
    area: f64,
    /// The COCO bounding box format is [top left x position, top left y position, width, height].
    /// bbox exemple:  "bbox": [473.07,395.93,38.65,28.67]
    bbox: BBox,
    /// Either 1 or 0
    iscrowd: u32,
}

pub enum Segmentation {
    Polygon(Vec<Vec<f64>>),
    RLE(RLE),
    EncodedRLE(EncodedRLE),
}

pub struct RLE {
    size: Vec<u32>,
    counts: Vec<u32>,
}

pub struct EncodedRLE {
    size: Vec<u32>,
    counts: String,
}

pub struct BBox {
    left: u32,
    top: u32,
    width: u32,
    height: u32,
}

pub struct Category {
    id: u32,
    name: String,
    supercategory: String,
}
