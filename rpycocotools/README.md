# rpycocotools

[![PyPI](https://img.shields.io/pypi/v/rpycocotools?color=green&style=flat)](https://pypi.org/project/rpycocotools)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/rpycocotools?style=flat)](https://pypi.org/project/rpycocotools)
[![PyPI - Downloads](https://img.shields.io/pypi/dm/rpycocotools?style=flat-square)](https://pypistats.org/packages/rpycocotools)
[![PyPI - License](https://img.shields.io/pypi/l/rpycocotools?style=flat)](https://opensource.org/licenses/MIT)
![CI Python](https://github.com/hoel-bagard/cocotools-rs/actions/workflows/ci-python-rpycocotools.yaml/badge.svg)
![CI Rust](https://github.com/hoel-bagard/cocotools-rs/actions/workflows/ci-rust-rpycocotools.yaml/badge.svg)

The `rpycocotools` package provides tools to load, manipulate, convert and visualize COCO format datasets. The documentation is available [here](https://cocotools-rs.readthedocs.io/en/latest/index.html).

### Installation

The package is available on PyPI [here](https://pypi.org/project/rpycocotools/), and can installed with pip:
```
pip install rpycocotools
```

You can also git clone this repo and build it yourself with:
```
pip install -r requirements/requirements-build.txt
pip install .
```

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
mask = rpycocotools.mask.decode(anns[0].segmentation)
```
The mask is a numpy array and can be visualized (for example with opencv):

<p align="center">
  <img alt="bike_segmentation" src="https://user-images.githubusercontent.com/34478245/226691842-8a11cde1-905d-434e-b287-0c3c685e01d1.png">
</p>


## Benchmarks

<details>
<summary>Details</summary>

There are a few benchmarking scripts to compare to `pycocotools`.\
The results reported here are done on my own PC and presented only to get a general idea. I might run the benchmark on a more reproducible environment in the future.

### Setup
Some of the benchmarks use the `instances_train2017.json` files from the 2017 COCO dataset.\
Either place this file in the `data_sample` folder or run the commands below with the ` -m "not coco2017"` option.

```bash
pip install -r requirements/requirements-benchmarks.txt
pip install .
```

### Load
Benchmark how much time it takes load a COCO dataset.

```bash
python -m pytest benchmarks/load.py -vv
```

Results:

| Test Name                                       | Mean time in s |
|:-----------------------------------------------:|:--------------:|
| rpycocotools on COCO `instances_train2017.json` | 4.4            |
| pycocotools on COCO `instances_train2017.json`  | 16.5           |

### Area
Benchmark how much time it takes to compute the total number of mask pixels in a COCO dataset.

```bash
python -m pytest benchmarks/area.py -vv -m "not coco2017"
```

Results:
| Test Name                     | Mean time in us |
|:-----------------------------:|:---------------:|
| pycocotools on `data_sample`  | 15              |
| rpycocotools on `data_sample` | 776             |


```bash
python -m pytest benchmarks/area.py -vv -m coco2017
```

Results:
| Test Name                                       | Mean time in ms |
|:-----------------------------------------------:|:---------------:|
| rpycocotools on COCO `instances_train2017.json` | 880.6           |
| pycocotools on COCO `instances_train2017.json`  | 19,302.9        |

</details>
