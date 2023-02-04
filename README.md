# rust_coco_tools
COCO utils implemented in rust



cargo clippy --fix -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used -W clippy::expect_used -A clippy::must_use_candidate -A clippy::module_name_repetitions

--allow-dirty

### Supported formats:
The COCO dataset describes [6 formats](https://cocodataset.org/#format-data) for 6 different tasks. As of now only the `Object Detection` format is supported, with plans to add support for the `Keypoint Detection` in the near future.
