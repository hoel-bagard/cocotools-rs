# Rpycocotools

Tool to handle COCO-like data in python. This repo is very much a wip.

### Build

Build and install into local virtualenv with `maturin develop`.

### Usage example

```python
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
coco_dataset.visualize_img(174482)
```

![rpycocotools_visu_example](https://user-images.githubusercontent.com/34478245/216580391-72226762-3fca-482b-a5ed-f93ed5a21931.png)

### Run the tests
```
python -m pytest . -vv
```

## TODO
- Try to use `hypothesis` for testing.
