# Rpycocotools

Tool to handle COCO-like data in python. This repo is very much a wip.

### Usage example

```python
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json")
rpycocotools.visualize_img(coco_dataset, "../data_samples/coco_25k/images/", 174482)
```

## TODO
- Try to use `hypothesis` for testing.
