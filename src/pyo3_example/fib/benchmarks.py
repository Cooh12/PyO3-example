import rust
import python
from timeit import timeit


def bench_fast(num: int) -> None:
    print(f"{num} PyO3  fast: {timeit(lambda: rust.fast_find_fibonacci_num(num))} sec")
    print(f"{num} Python fast: {timeit(lambda: python.fast_find_fibonacci_num(num))} sec")


def bench_recursive(num: int) -> None:
    print(f"{num} PyO3 recursive: {timeit(lambda: rust.recursive_find_fibonacci_num(num))} sec")
    print(f"{num} Python recursive: {timeit(lambda: python.recursive_find_fibonacci_num(num))} sec")


def main() -> None:
    bench_fast(100)
    bench_fast(200)
    bench_fast(300)

    bench_recursive(10)
    bench_recursive(15)

main()
