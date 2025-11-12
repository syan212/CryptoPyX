from click.testing import CliRunner
from cryptopyx.cli.main import cli


def test_base32_encode_cli():
    runner = CliRunner()
    # Test encoding a string
    result = runner.invoke(
        cli,
        ['base32', 'encode', 'hello world', '--string'],
    )
    assert result.exit_code == 0
    assert 'NBSWY3DPEB3W64TMMQ======' in result.output

    # Test encoding from a file
    with runner.isolated_filesystem():
        with open('input.txt', 'w', encoding='utf-8') as f:
            f.write('hello world')
        result = runner.invoke(
            cli,
            ['base32', 'encode', 'input.txt'],
        )
        assert result.exit_code == 0
        assert 'NBSWY3DPEB3W64TMMQ======' in result.output


def test_base32_decode_cli():
    runner = CliRunner()
    # Test decoding a string
    result = runner.invoke(
        cli,
        ['base32', 'decode', 'NBSWY3DPEB3W64TMMQ======', '--string'],
    )
    assert result.exit_code == 0
    assert 'hello world' in result.output

    # Test decoding from a file
    with runner.isolated_filesystem():
        with open('input.txt', 'w', encoding='utf-8') as f:
            f.write('NBSWY3DPEB3W64TMMQ======')
        result = runner.invoke(
            cli,
            ['base32', 'decode', 'input.txt'],
        )
        assert result.exit_code == 0
        assert 'hello world' in result.output
