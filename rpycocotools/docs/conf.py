# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = "rpycocotools"
copyright = "2023, Hoel Bagard"
author = "Hoel Bagard"
release = "0.0.3"

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

autodoc_typehints = "none"
master_doc = "index"

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

extensions = [
    "hoverxref.extension",
    "sphinx_codeautolink",
    "sphinx.ext.viewcode",
    "sphinx.ext.intersphinx",
]

# See https://sphinx-hoverxref.readthedocs.io/en/latest/configuration.html
hoverxref_auto_ref = True
hoverxref_domains = ["py"]
hoverxref_role_types = {
    "attr": "tooltip",
    "class": "tooltip",
    "const": "tooltip",
    "exc": "tooltip",
    "func": "tooltip",
    "meth": "tooltip",
    "mod": "tooltip",
    "obj": "tooltip",
    "ref": "tooltip",
}

intersphinx_mapping = {
    "numpy": ("https://numpy.org/doc/stable/", None),
}

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "sphinx_rtd_theme"
html_static_path = ["_static"]
