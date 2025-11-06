import click


# This is the main CLI group, for importing in subcommand modules.
@click.group()
def cli() -> None:
    """CryptoPyX: A Python library for cryptographic operations."""
    pass
