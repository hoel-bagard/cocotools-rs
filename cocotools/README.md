# Cocotools
The `cocotools` crate provides tools to load, manipulate, convert and visualize COCO format datasets.

## API Usage example
```
use std::path::PathBuf;
use cocotools::COCO;

let annotations_file_path = PathBuf::from("../data_samples/coco_25k/annotations.json");
let dataset = COCO::try_from(&annotations_file_path)?;
let file_name = dataset.get_img(17627)?.file_name;
```

## Program Usage

```
cargo run -- visualize  ../data_samples/coco_25k/annotations.json ../data_samples/coco_25k/images -s 000000017627
cargo run -- convert-segmentation ../data_samples/coco_25k/annotations.json rle -o annotations_rle.json
```

## Planned features
- [ ] Add support for keypoint detection format.
- [ ] Add conversion from/to PascalVOC format.
- [ ] Add conversion from/to SOLO format.
- [ ] Add validation of the data when loading it, for example check that sum(rle) == nb pixels in the image (behind a crate feature flags ?)
- [ ] Use rayon when loading/converting the data ?
