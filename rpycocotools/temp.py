from hbtools import show_img
import cv2
import rpycocotools
coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
img = coco_dataset.get_img(174482)
anns = coco_dataset.get_img_anns(174482)
for ann in anns:
    encoded_mask = ann.segmentation
    mask = rpycocotools.mask.decode(encoded_mask)
    # show_img(255*mask)

img = cv2.imread("../data_samples/coco_25k/images/" + img.file_name)
print(mask.shape)
print(img.shape)
# show_img(img)
