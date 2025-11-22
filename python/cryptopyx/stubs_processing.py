import ast
import builtins
from collections.abc import Callable, Generator
import importlib.util
import inspect
from inspect import Parameter, Signature
from pathlib import Path
from types import ModuleType
import typing

# For `resolve_annotations` function
SAFE_GLOBALS = {
    **typing.__dict__,  # typing objects
    **builtins.__dict__,  # builtins
}


def resolve_annotations(annotation: str, other_globals: dict | None = None) -> object:
    """Convert an annotation string(e.g. `'str'`)
    to a python object(e.g. the actual python class `str`).
    Can also specify extra safe annotations via `other_globals`.
    """
    # Checks
    if not annotation or annotation == 'None':
        return type(None)
    if not isinstance(annotation, str):
        return annotation
    # Allow module-specific names if provided
    safe_globals = {**SAFE_GLOBALS, **other_globals} if other_globals else SAFE_GLOBALS
    # Evaluate
    try:
        return eval(annotation, safe_globals, {})  # noqa: S307
    except NameError:
        # Fallback
        return annotation


def sig_to_ann(sig: Signature) -> dict[str, object]:
    """Converts a `Signature` object into a dict suitable for `__annotations__`"""
    ann = {}
    for name, param in sig.parameters.items():
        if param.annotation is not inspect._empty:
            ann[name] = param.annotation
    if sig.return_annotation is not inspect._empty:
        ann['return'] = sig.return_annotation

    return ann


def find_pyi_files(package: str) -> Generator[tuple[str, Path], None, None]:
    """Yield (`module_name`, `pyi_path`) for all .pyi files under a package."""
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


def build_signature(node: ast.FunctionDef) -> Signature:
    """Build `Signature` object from `ast.FunctionDef`"""
    params: list[Parameter] = []
    # Handle positional-only
    params.extend(
        Parameter(
            a.arg,
            Parameter.POSITIONAL_ONLY,
            annotation=resolve_annotations(ast.unparse(a.annotation))
            if a.annotation
            else Parameter.empty,
        )
        for a in node.args.posonlyargs
    )
    # Handle positional or keyword
    params.extend(
        Parameter(
            a.arg,
            Parameter.POSITIONAL_OR_KEYWORD,
            annotation=resolve_annotations(ast.unparse(a.annotation))
            if a.annotation
            else Parameter.empty,
        )
        for a in node.args.args
    )
    # Handle defaults for positional or keywords parameters
    total_pos_params = len(node.args.posonlyargs) + len(node.args.args)
    num_defaults = len(node.args.defaults)
    if num_defaults:
        for param, default in zip(
            params[total_pos_params - num_defaults :], node.args.defaults, strict=True
        ):
            if isinstance(default, ast.Constant):
                default_value = default.value
            else:
                default_value = Parameter.empty
            params[params.index(param)] = param.replace(default=default_value)
    # Handle *args
    if node.args.vararg:
        a = node.args.vararg
        params.append(
            Parameter(
                a.arg,
                Parameter.VAR_POSITIONAL,
                annotation=resolve_annotations(ast.unparse(a.annotation))
                if a.annotation
                else Parameter.empty,
            )
        )
    # Handle keyword only and defaults
    for arg, default in zip(node.args.kwonlyargs, node.args.kw_defaults, strict=True):
        if isinstance(default, ast.Constant):
            default_value = default.value
        else:
            print(
                f"""Warning: non-literal default ignored in {node.name}: 
                {default}"""
            )
            default_value = Parameter.empty

        params.append(
            Parameter(
                name=arg.arg,
                kind=Parameter.KEYWORD_ONLY,
                annotation=resolve_annotations(ast.unparse(arg.annotation))
                if arg.annotation
                else Parameter.empty,
                default=default_value,
            )
        )
    # Handle **kwargs
    if node.args.kwarg:
        params.append(
            Parameter(
                name=node.args.kwarg.arg,
                kind=Parameter.VAR_KEYWORD,
                annotation=(
                    resolve_annotations(ast.unparse(node.args.kwarg.annotation))
                    if node.args.kwarg.annotation
                    else Parameter.empty
                ),
            )
        )
    # Handle return annotation
    returns = (
        resolve_annotations(ast.unparse(node.returns))
        if node.returns
        else Signature.empty
    )

    return Signature(params, return_annotation=returns)


def parse_signatures(pyi_path: Path) -> dict[str, Signature]:
    """Return a dict of function/method signatures parsed from a .pyi file."""
    tree = ast.parse(pyi_path.read_text(encoding='utf-8'), filename=str(pyi_path))
    signatures: dict[str, Signature] = {}
    for node in tree.body:
        # Functions
        if isinstance(node, ast.FunctionDef):
            signatures[node.name] = build_signature(node)
            continue
        # Classes & methods
        if not isinstance(node, ast.ClassDef):
            continue
        for sub in node.body:
            if isinstance(sub, ast.FunctionDef):
                signatures[f'{node.name}.{sub.name}'] = build_signature(sub)
    return signatures


def parse_docstrings(pyi_path: Path) -> dict[str, str]:
    """Return a dict of docstrings parsed from a .pyi file."""
    tree = ast.parse(pyi_path.read_text(encoding='utf-8'), filename=str(pyi_path))
    # Collect docstrings
    docs = {}
    module_doc = ast.get_docstring(tree)
    docs['__module__'] = module_doc
    # Walk through the AST nodes
    for node in tree.body:
        # Plain functions
        if isinstance(node, ast.FunctionDef):
            docs[node.name] = ast.get_docstring(node)
            continue
        # Classes and methods
        if not isinstance(node, ast.ClassDef):
            continue
        class_doc = ast.get_docstring(node)
        docs[node.name] = class_doc
        for sub in node.body:
            if isinstance(sub, ast.FunctionDef):
                fullname = f'{node.name}.{sub.name}'
                docs[fullname] = ast.get_docstring(sub)
    return docs


# Apply docs and strings to single object
def apply_docs_and_sigs_to_obj(
    obj: Callable, docs: str | None, sig: Signature | None
) -> None:
    """Apply docs and signatures to single object(writeable)."""
    if docs:
        obj.__doc__ = docs
    if sig:
        obj.__signature__ = sig  # type: ignore
        obj.__annotations__ = sig_to_ann(sig)


# Wrap function and add docstrings and signature
def wrapper(
    input_func: Callable, docs: str | None, input_sig: Signature | None
) -> Callable:
    def inner(*args: object, **kwargs: object) -> object:
        return input_func(*args, **kwargs)

    inner.__name__, inner.__module__, inner.__qualname__ = (
        input_func.__name__,
        input_func.__module__,
        input_func.__qualname__,
    )
    inner.__wrapped__ = input_func  # type: ignore
    if hasattr(input_func, '__dict__'):
        inner.__dict__.update(input_func.__dict__)
    # Apply signature and docs if available
    apply_docs_and_sigs_to_obj(inner, docs, input_sig)
    return inner


def apply_docs_and_signatures(
    module: ModuleType,
    docs: dict[str, str],
    signatures: dict[str, Signature],
) -> None:
    """Inject parsed docstrings and signature into the module and its attributes."""
    # Module-level docstring
    if docs.get('__module__'):
        try:
            module.__doc__ = docs['__module__']
        except (AttributeError, TypeError) as e:
            print(f'Warning: cannot set module docstring for {module.__name__}: {e}')
    # Non-module objects
    for name in frozenset(docs) | frozenset(signatures):
        if name == '__module__':
            continue
        sig = signatures.get(name)
        doc = docs.get(name)
        # Class or function at module level
        if hasattr(module, name):
            obj = getattr(module, name)
            try:
                # Apply docs and signature if available
                apply_docs_and_sigs_to_obj(obj, doc, sig)
            # Unwritable builtin types (e.g., built-in functions)
            except (AttributeError, TypeError):
                wrapped = wrapper(obj, doc, sig)
                setattr(module, name, wrapped)
        # Class method
        # ! Still not tested
        if '.' in name:
            cls_name, meth_name = name.split('.', 1)
            cls = getattr(module, cls_name, None)
            if not (cls and hasattr(cls, meth_name)):
                continue
            meth = getattr(cls, meth_name)
            try:
                apply_docs_and_sigs_to_obj(meth, doc, sig)
            except (AttributeError, TypeError) as e:
                print(f'Warning: cannot set module docstring for {name}: {e}')
