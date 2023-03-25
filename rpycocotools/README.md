# Rpycocotools

[![PyPI](https://img.shields.io/pypi/v/rpycocotools?style=flat)](https://pypi.org/project/rpycocotools)
[![PyPI - Implementation](https://img.shields.io/pypi/implementation/rpycocotools?style=flat)](https://pypi.org/project/rpycocotools)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/rpycocotools?style=flat)](https://pypi.org/project/rpycocotools)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/rpycocotools?style=flat-square)](https://pypistats.org/packages/rpycocotools)
[![PyPI - License](https://img.shields.io/pypi/l/rpycocotools?style=flat)](https://opensource.org/licenses/MIT)

Tool to handle COCO-like data in python. This repo is very much a wip.


### Build

Build and install into local virtualenv with `maturin develop`.

### Usage example

Visualize image with a given `id`:
```python
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
coco_dataset.visualize_img(174482)
```

<p align="center">
  <img alt="rpycocotools_visu_example" src="https://user-images.githubusercontent.com/34478245/216580391-72226762-3fca-482b-a5ed-f93ed5a21931.png">
</p>

```python
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
anns = coco_dataset.get_img_anns(174482)
mask = rpycocotools.mask.decode_poly_rs(anns[0].segmentation)
mask = 255 * mask
```
The mask is a numpy array and can be visualized (for example with opencv):

<p align="center">
  <img alt="bike_segmentation" src="https://user-images.githubusercontent.com/34478245/226691842-8a11cde1-905d-434e-b287-0c3c685e01d1.png">
</p>

### Run the tests
```
python -m pytest . -vv
```
