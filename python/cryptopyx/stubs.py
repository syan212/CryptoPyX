import ast
import importlib.util
from collections.abc import Callable, Generator
from functools import wraps
from pathlib import Path
from types import ModuleType
from typing import ParamSpec, TypeVar


def find_pyi_files(package: str) -> Generator[tuple[str, Path], None, None]:
    """Yield (module_name, pyi_path) for all .pyi files under a package."""
    # Load package spec to find its location
    package_spec = importlib.util.find_spec(package)
    if package_spec is None or package_spec.submodule_search_locations is None:
        raise ImportError(f'Cannot find package {package}')
    base_paths = package_spec.submodule_search_locations
    # Search for .pyi files
    for base in base_paths:
        base_path = Path(base)
        for path in base_path.rglob('*.pyi'):
            rel = path.relative_to(base_path)
            # Handle two cases: __init__.pyi and regular .pyi files
            if rel.name == '__init__.pyi':
                parts = rel.parent.parts  # directory holding __init__.pyi
                module_name = package + ('' if not parts else '.' + '.'.join(parts))
            else:
                module_name = package + '.' + '.'.join(rel.with_suffix('').parts)
            yield module_name, path


def parse_pyi_docstrings(pyi_path: Path) -> dict[str, str]:
    """Return a dict of docstrings parsed from a .pyi file."""
    tree = ast.parse(pyi_path.read_text(), filename=str(pyi_path))
    # Collect docstrings
    docs = {}
    module_doc = ast.get_docstring(tree)
    docs['__module__'] = module_doc
    # Walk through the AST nodes
    for node in tree.body:
        # Plain functions
        if isinstance(node, ast.FunctionDef):
            docs[node.name] = ast.get_docstring(node)
        # Classes and methods
        # ! Still not testesd
        elif isinstance(node, ast.ClassDef):
            class_doc = ast.get_docstring(node)
            docs[node.name] = class_doc
            for sub in node.body:
                if isinstance(sub, ast.FunctionDef):
                    fullname = f'{node.name}.{sub.name}'
                    docs[fullname] = ast.get_docstring(sub)
    return docs


def apply_docs_to_module(module: ModuleType, docs: dict[str, str]) -> None:
    """Inject parsed docstrings into the module and its attributes."""
    # Module-level docstring
    if docs.get('__module__'):
        try:
            module.__doc__ = docs['__module__']
        except Exception as e:
            print(f'Warning: cannot set module docstring for {module.__name__}: {e}')
    # Other docstrings
    for name, doc in docs.items():
        if name == '__module__' or not doc:
            continue
        # Class or function at module level
        if hasattr(module, name):
            obj = getattr(module, name)
            try:
                obj.__doc__ = doc
            # Unwritable builtin types (e.g., built-in functions)
            except AttributeError:
                P = ParamSpec('P')
                R = TypeVar('R')

                def wrapper(input_func: Callable[P, R], docs: str) -> Callable[P, R]:
                    @wraps(input_func)
                    def inner(*args: P.args, **kwargs: P.kwargs) -> R:
                        return input_func(*args, **kwargs)

                    inner.__doc__ = docs
                    return inner

                wrapped = wrapper(obj, doc)
                setattr(module, name, wrapped)
        # Class methods
        # ! Still not testesd
        if '.' in name:
            cls_name, meth_name = name.split('.', 1)
            cls = getattr(module, cls_name, None)
            if cls and hasattr(cls, meth_name):
                meth = getattr(cls, meth_name)
                try:
                    meth.__doc__ = doc
                except Exception as e:
                    print(f'Warning: cannot set module docstring for {name}: {e}')
