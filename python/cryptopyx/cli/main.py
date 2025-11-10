from . import base32
from .cli import cli

# Add subgroups and commands here
cli.add_command(base32.base32)
