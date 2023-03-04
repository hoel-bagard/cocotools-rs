For arguments (advanced): In public interfaces, you usually don't want to use Path or PathBuf directly, but rather a generic P: AsRef<Path> or P: Into<PathBuf>. That way the caller can pass in Path, PathBuf, &str or String.


## Usage

```
cargo run -- visualize  ../data_samples/coco_25k/annotations.json ../data_samples/coco_25k/images -s 000000017627
```

