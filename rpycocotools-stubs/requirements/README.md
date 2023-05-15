### Requirements generation

The requirements can be re-generated with the following commands:
```
pip-compile --output-file=requirements/requirements.txt --resolver=backtracking pyproject.toml --generate-hashes
pip-compile --extra=dev --output-file=requirements/requirements-dev.txt --resolver=backtracking pyproject.toml --generate-hashes
pip-compile --extra=build --output-file=requirements/requirements-build.txt --resolver=backtracking pyproject.toml --generate-hashes
```
