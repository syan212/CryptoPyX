import click


# This is the main CLI group, for importing in subcommand modules.
# Do not import the cli object from this file, use main.py instead.
@click.group
@click.version_option(
    prog_name='CryptoPyX', package_name='cryptopyx', message='%(prog)s v%(version)s'
)
def cli() -> None:
    """CryptoPyX: A Python library for cryptographic operations."""
    pass
