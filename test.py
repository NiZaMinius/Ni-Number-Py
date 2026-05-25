import time

import ni_number_py

_string_cache = {}


def ni_digits_cached(n: int) -> str:
    if n not in _string_cache:
        _string_cache[n] = ni_number_py.ni_digits(n)
    return _string_cache[n]


print("\n--- Тяжёлый расчёт vs Кэш ---")
for digits_count in [100, 1_000, 5_000]:
    _string_cache.clear()
    ni_number_py.clear()

    start = time.perf_counter()
    result = ni_digits_cached(digits_count)
    elapsed_cold = time.perf_counter() - start

    start = time.perf_counter()
    result = ni_digits_cached(digits_count)
    elapsed_hot = time.perf_counter() - start

    print(
        f"ni_digits({digits_count:>6}): "
        f"Холодный = {elapsed_cold * 1000:>8.3f} ms | "
        f"Из кэша = {elapsed_hot * 1000:>6.3f} ms"
    )

with open(f"ni_{digits_count}_digits.txt", "w") as f:
    f.write(result)
print(f"  Сохранено в ni_{digits_count}_digits.txt")
