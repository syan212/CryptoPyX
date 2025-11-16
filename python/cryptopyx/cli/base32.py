import click
from cryptopyx import encodings

from . import utils
from .cli import cli


@cli.group
def base32() -> None:
    """Base32 encoding and decoding commands."""
    pass


@base32.command
@click.argument('data', type=click.STRING)
@click.option(
    '--string',
    '-s',
    is_flag=True,
    default=False,
    help='Indicates that the input is a string rather than a file path.',
)
@click.option(
    '--output',
    '-o',
    type=click.Path(writable=True),
    help='Output file to save the encoded data. Defaults to stdout if not provided.',
)
def encode(data: str, string: bool, output: str | None) -> None:
    """Encode string data to Base32."""
    # Read input data, either from a string or a file
    if not string:
        with open(data, encoding='utf-8') as f:
            data = f.read()
    # Calculate encoded result with loading animation
    result = utils.calc_loading(encodings.base32.encode, data)
    # Output to file or stdout
    if output:
        with open(output, 'w', encoding='utf-8') as f:
            f.write(result)
        click.secho(f'Encoded data saved to {output}', fg='green')
    else:
        click.echo('Encoded data:')
        click.secho(result, fg='green')


@base32.command
@click.argument('data', type=click.STRING)
@click.option(
    '--string',
    '-s',
    is_flag=True,
    default=False,
    help='Indicates that the input is a string rather than a file path.',
)
@click.option(
    '--output',
    '-o',
    type=click.Path(writable=True),
    help='Output file to save the decoded data. Defaults to stdout if not provided.',
)
def decode(data: str, string: bool, output: str | None) -> None:
    """Decode string data to Base32."""
    # Read input data, either from a string or a file
    if not string:
        with open(data, encoding='utf-8') as f:
            data = f.read()
    # Calculate decoded result with loading animation
    result = utils.calc_loading(encodings.base32.decode, data)
    # Output to file or stdout
    if output:
        with open(output, 'w', encoding='utf-8') as f:
            f.write(result)
        click.secho(f'Decoded data saved to {output}', fg='green')
    else:
        click.echo('Decoded data:')
        click.secho(result, fg='green')
