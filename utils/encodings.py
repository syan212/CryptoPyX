import random


# Random base32
def random_base32(length: int, string_num: int = 100) -> list[str]:
    """Generate random base32 string. For testing only"""
    random.seed(31415)
    out = []
    alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ234567'
    for _ in range(string_num):
        s = ''.join(random.choice(alphabet) for _ in range(length))
        out.append(s)

    return out


# Random base64
def random_base64(length: int, string_num: int = 100) -> list[str]:
    """Generate random base64 string. For testing only"""
    random.seed(31415)
    out = []
    alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'
    for _ in range(string_num):
        s = ''.join(random.choice(alphabet) for _ in range(length))
        out.append(s)

    return out
