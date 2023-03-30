Usage example
=============

.. code-block:: python
    :caption: Visualize image with a given `id`:

    import rpycocotools
    coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
    coco_dataset.visualize_img(174482)

.. code-block:: python
    :caption: Load a mask and decode it

    import rpycocotools
    coco_dataset = rpycocotools.COCO("../data_samples/coco_25k/annotations.json", "../data_samples/coco_25k/images")
    anns = coco_dataset.get_img_anns(174482)
    encoded_mask = anns[0].segmentation
    mask = rpycocotools.mask.decode_poly_rs(encoded_mask)
