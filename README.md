# rust_coco_tools
COCO utils implemented in rust

### Supported formats:
The COCO dataset describes [6 formats](https://cocodataset.org/#format-data) for 6 different tasks, as of now only the `Object Detection` format is supported.

## License
Licensed under either of:
- [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0)
- [MIT license](http://opensource.org/licenses/MIT)
at your option.

## TODO:
- [ ] Github action to release python version (see [here](https://github.com/pola-rs/polars/blob/master/.github/workflows/create-python-release.yml))
- [ ] See if rayon can be used.

- repo: https://github.com/tox-dev/pyproject-fmt
  rev: "0.9.2"
  hooks:
    - id: pyproject-fmt
