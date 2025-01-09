def factorielle_recursive(n : int) -> int:
    if (n == 0 or n == 1):
        return 1
    return n * factorielle_recursive(n-1)

