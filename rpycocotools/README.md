


```python
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json")
for cat_id, cat in coco_dataset.cats.items():
     print(f"Category id: {cat_id: >2}, category name: {cat.name}")
```

```python
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json")
print(coco_dataset.cats[1].name)
coco_dataset.cats[1].name = "elf"
print(coco_dataset.cats[1].name)
```
