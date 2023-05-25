# Cocotools
![cocotools ci](https://github.com/hoel-bagard/cocotools-rs/actions/workflows/ci-cocotools.yaml/badge.svg)
[![Crate](https://img.shields.io/crates/v/cocotools.svg?color=green&style=flat)](https://crates.io/crates/cocotools)
[![Minimum rustc 1.64](https://img.shields.io/badge/rustc-1.64+-blue.svg)](https://rust-lang.github.io/rfcs/2495-min-rust-version.html)
[![Documentation](https://docs.rs/cocotools/badge.svg)](https://docs.rs/cocotools)

The `cocotools` crate provides tools to load, manipulate/convert and visualize COCO format datasets.

## Setup
Get the crate from [crates.io](https://crates.io/crates/cocotools).

## API Usage
You can find the documentation [here](https://docs.rs/cocotools/latest/cocotools/index.html).

### Example
```
use std::path::PathBuf;
use cocotools::COCO;

let annotations_file_path = PathBuf::from("../data_samples/coco_25k/annotations.json");
let image_folder_path = PathBuf::from("../data_samples/coco_25k/images");
let coco_dataset = COCO::new(&annotations_file_path, &image_folder_path)?;
let file_name = dataset.get_img(17627)?.file_name;
```

## Program Usage

```
cargo run -- visualize  ../data_samples/coco_25k/annotations.json ../data_samples/coco_25k/images -s 000000017627
cargo run -- convert-segmentation ../data_samples/coco_25k/annotations.json rle -o annotations_rle.json
```

## Future features
- [ ] Add support for keypoint detection format.
- [ ] Add conversion from/to PascalVOC format.
- [ ] Add conversion from/to SOLO format.
- [ ] Add validation of the data when loading it, for example check that sum(rle) == nb pixels in the image (behind a crate feature flags ?)
- [ ] Use rayon when loading/converting the data ?
