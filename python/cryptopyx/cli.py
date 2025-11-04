import click

import cryptopyx.encodings as encodings


@click.group()
def cli() -> None:
    """CryptoPyX: A Python library for cryptographic operations."""
    pass


@cli.group()
def base32() -> None:
    """Base32 encoding and decoding commands."""
    pass


@base32.command()
@click.argument('data', type=click.STRING)
def encode(data: str) -> None:
    """Encode data to Base32."""
    click.echo('Encoded data:')
    click.secho(encodings.base32.encode(data), fg='green')


@base32.command()
@click.argument('data', type=click.STRING)
def decode(data: str) -> None:
    """Decode data to Base32."""
    click.echo('Decoded data:')
    click.secho(encodings.base32.decode(data), fg='green')
