import rpycocotools
from hypothesis import given
from hypothesis import strategies as st

u32_max = 4_294_967_295
u32_st = st.integers(min_value=0, max_value=u32_max)


def test_access_cats(coco_dataset: rpycocotools.COCO) -> None:
    cats = coco_dataset.get_cats()
    assert len(cats) == 80


def test_access_cat(coco_dataset: rpycocotools.COCO) -> None:
    cat = coco_dataset.get_cat(2)
    assert cat.supercategory == "vehicle"
    assert cat.id == 2
    assert cat.name == "bicycle"
    assert str(cat) == "Category(id=2, name='bicycle', supercategory='vehicle')"


def test_access_anns(coco_dataset: rpycocotools.COCO) -> None:
    anns = coco_dataset.get_anns()
    assert len(anns) == 45


def test_access_ann(coco_dataset: rpycocotools.COCO) -> None:
    ann = coco_dataset.get_ann(1348739)
    assert ann.id == 1348739
    assert isinstance(ann.segmentation, rpycocotools.anns.PolygonsRS)
    assert str(ann) == ("Annotation(id=1348739, image_id=174482, category_id=3, segmentation=PolygonsRS("
                        "size=[388, 640], "
                        "counts=[[81.28, 87.23, 82.91, 83.96, 84.0, 76.33, 99.48, 76.22, 105.91, 84.5, 108.09, "
                        "93.98, 98.17, 93.44, 90.33, 94.2, 85.97, 94.53, 84.0, 94.31]]), "
                        "area=390.6123, bbox=BBox(left=81.28, top=76.22, width=26.81, height=18.31), iscrowd=0)")


def test_access_imgs(coco_dataset: rpycocotools.COCO) -> None:
    imgs = coco_dataset.get_imgs()
    assert len(imgs) == 4


def test_access_img(coco_dataset: rpycocotools.COCO) -> None:
    img = coco_dataset.get_img(img_id=174482)
    assert img.id == 174482
    assert str(img) == "Image(id=174482, width='640', height='388', file_name='000000174482.jpg')"


def test_get_img_anns(coco_dataset: rpycocotools.COCO) -> None:
    anns = coco_dataset.get_img_anns(480985)
    assert len(anns) == 13
    assert all(ann.image_id == 480985 for ann in anns)


@given(u32_st, u32_st, u32_st, u32_st)
def test_bbox_create(left: int, top: int, width: int, height: int) -> None:
    bbox = rpycocotools.anns.BBox(left, top, width, height)
    assert isinstance(bbox, rpycocotools.anns.BBox)


@given(u32_st, u32_st, u32_st, u32_st)
def test_bbox_equality(left: int, top: int, width: int, height: int) -> None:
    bbox1 = rpycocotools.anns.BBox(left, top, width, height)
    bbox2 = rpycocotools.anns.BBox(left, top, width, height)
    assert bbox1 == bbox2


@given(st.tuples(st.tuples(u32_st, u32_st, u32_st, u32_st),
                 st.tuples(u32_st, u32_st, u32_st, u32_st)).filter(lambda x: x[0] != x[1]))
def test_bbox_inequality(coords: tuple[tuple[int, int, int, int], tuple[int, int, int, int]]) -> None:
    bbox1 = rpycocotools.anns.BBox(*coords[0])
    bbox2 = rpycocotools.anns.BBox(*coords[1])
    assert bbox1 != bbox2


@given(u32_st, u32_st, u32_st, u32_st)
def test_bbox_repr(left: int, top: int, width: int, height: int) -> None:
    bbox = rpycocotools.anns.BBox(left, top, width, height)
    assert str(bbox) == f"BBox(left={left}, top={top}, width={width}, height={height})"
