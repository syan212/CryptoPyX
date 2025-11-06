import click
import cryptopyx.encodings as encodings

from . import utils
from .cli import cli


@cli.group()
def base32() -> None:
    """Base32 encoding and decoding commands."""
    pass


@base32.command()
@click.argument('data', type=click.STRING)
def encode(data: str) -> None:
    """Encode data to Base32."""
    result = utils.calc_loading(encodings.base32.encode, data)
    click.echo('Encoded data:')
    click.secho(result, fg='green')


@base32.command()
@click.argument('data', type=click.STRING)
def decode(data: str) -> None:
    """Decode data to Base32."""
    result = utils.calc_loading(encodings.base32.decode, data)
    click.echo('Decoded data:')
    click.secho(result, fg='green')
