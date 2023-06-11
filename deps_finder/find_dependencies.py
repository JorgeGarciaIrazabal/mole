import json
import typer
from poetry.repositories.pypi_repository import PyPiRepository
from poetry.core.packages.dependency import Dependency
import re

app = typer.Typer()

@app.command()
def find_dependencies(package_name: str, version: str):
    my_repo = PyPiRepository(disable_cache=True)
    relese_info = my_repo.get_release_info(package_name, version)

    print(json.dumps(relese_info.requires_dist, indent=2))


@app.command()
def find_package_info(package_name: str):
    my_repo = PyPiRepository(disable_cache=True)
    info = my_repo.get_release_info(package_name)
    requires_dist = []
    for dist in info.requires_dist:
        dependency, constrain = dist.split(" ; ")
        dependency_name, version = dependency.split(" ")
        version = version.replace("(", "").replace(")", "")
        extra = None
        if "extra" in constrain:
            extra = constrain.split("extra == ")[1].replace('"', "")
        requires_dist.append({
            "name": dependency_name,
            "version": version,
            "extra": extra
        })
    print(json.dumps(requires_dist, indent=2))



if __name__ == "__main__":
    app()


