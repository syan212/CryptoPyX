import base64
import random


# Random base32
def random_base32(length: int, string_num: int = 100) -> list[str]:
    """Generate random base32 string. For testing only"""
    random.seed(31415)
    out = []
    bytes_per_block = (length * 5) // 8  # since 8 chars = 5 bytes

    for _ in range(string_num):
        data = bytes(random.getrandbits(8) for _ in range(bytes_per_block))
        s = base64.b32encode(data).decode('ascii')
        s = s.rstrip('=')  # remove padding
        out.append(s)

    return out
