def fast_find_fibonacci_num(num: int) -> int:
    if num <= 1:
        return num
    a, b = 0, 1
    for i in range(2, num + 1):
        a, b = b, a + b
    return b


def recursive_find_fibonacci_num(num: int) -> int:
    if num <= 1:
        return num
    return recursive_find_fibonacci_num(num - 1) + recursive_find_fibonacci_num(num - 2)
