import click
import cryptopyx.encodings as encodings

from . import utils
from .cli import cli


@cli.group
def base32() -> None:
    """Base32 encoding and decoding commands."""
    pass


@base32.command
@click.argument('file', type=click.STRING)
@click.option(
    '--string',
    '-s',
    is_flag=True,
    help='Indicates that the input is a string rather than a file path.',
)
@click.option(
    '--output',
    '-o',
    type=click.Path(writable=True),
    help='Output file to save the encoded data. Defaults to stdout if not provided.',
)
def encode(file: str, string: bool, output: str | None) -> None:
    """Encode string data to Base32."""
    # Read input data, either from a string or a file
    if string:
        data = file
    else:
        with open(file, encoding='utf-8') as f:
            data = f.read()
    # For type checking purposes
    result: str = str(utils.calc_loading(encodings.base32.encode, data))
    # Output to file or stdout
    if output:
        with open(output, 'w', encoding='utf-8') as f:
            f.write(result)
        click.secho(f'Encoded data saved to {output}', fg='green')
    else:
        click.echo('Encoded data:')
        click.secho(result, fg='green')


@base32.command
@click.argument('file', type=click.STRING)
@click.option(
    '--string',
    '-s',
    is_flag=True,
    help='Indicates that the input is a string rather than a file path.',
)
@click.option(
    '--output',
    '-o',
    type=click.Path(writable=True),
    help='Output file to save the decoded data. Defaults to stdout if not provided.',
)
def decode(file: str, string: bool, output: str | None) -> None:
    """Decode string data to Base32."""
    # Read input data, either from a string or a file
    if string:
        data = file
    else:
        with open(file, encoding='utf-8') as f:
            data = f.read()
    # For type checking purposes
    result: str = str(utils.calc_loading(encodings.base32.decode, data))
    # Output to file or stdout
    if output:
        with open(output, 'w', encoding='utf-8') as f:
            f.write(result)
        click.secho(f'Decoded data saved to {output}', fg='green')
    else:
        click.echo('Decoded data:')
        click.secho(result, fg='green')
