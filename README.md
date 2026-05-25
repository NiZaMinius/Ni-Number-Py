# ni-number-py
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/ni-number-py)](https://pypi.org/project/ni-number-py)
[![PyPI](https://img.shields.io/pypi/v/ni-number-py)](https://pypi.org/project/ni-number-py)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)


Python bindings for [ni-number](https://crates.io/crates/ni-number) — high-precision computation of the **Ni constant** (η_ν), the quantum energy scattering constant.

```
η_ν = 1.88937666040491913115597775087642096081019761538215...
``` 

Powered by Rust under the hood — no Rust installation required.

---

## What is the Ni Constant?

The Ni constant is defined by the series:

$$\eta_{\nu} = \sum_{n=1}^{\infty} \frac{\pi^n}{n! \cdot 2^{n^2}}$$

It represents the total amplitude of gauge-field attenuation at sub-Planckian distances, where interaction carriers leak into compactified extra dimensions (Calabi–Yau manifolds).

**Author of the hypothesis:** NiZaMinius

---

## Installation

```bash
pip install ni-number-py
```

No system dependencies required. The package ships precompiled wheels for Windows, macOS, and Linux.

---

## Usage

```python
import ni_number_py

# Fast f64 constant — precomputed, instant (~15 digits)
print(ni_number_py.NI_F64)
# 1.8893766604049191

# Fast f32 constant (~7 digits)
print(ni_number_py.NI_F32)
# 1.8893767

# First 50 digits — static string, instant
print(ni_number_py.ni_50_digits())
# 1.88937666040491913115597775087642096081019761538215

# Arbitrary precision — returns a string with n digits after the decimal point
print(ni_number_py.ni_digits(100))
# 1.8893766604049191311559777508764209608101976153821588...

# Clear the internal cache (frees memory after high-precision computation)

ni_number_py.clear()
```

```python
from decimal import Decimal
import ni_number_py

# Get 100 digits as string
ni_str = ni_number_py.ni_digits(100)

# Convert to Python's high-precision Decimal for math operations
ni_high_precision = Decimal(ni_str)
```

---

## API

| Function / Constant | Returns | Description |
|---|---|---|
| `NI_F64` | `float` | Precomputed constant at double precision (~15 digits) |
| `NI_F32` | `float` | Precomputed constant at single precision (~7 digits) |
| `ni_50_digits()` | `str` | First 50 decimal digits, static string |
| `ni_digits(n)` | `str` | Decimal string with `n` digits after the point, cached |
| `clear()` | `None` | Free cached values from memory |

---

## Performance

The series converges in fewer than 10 iterations for any practical precision level.
Results are cached after the first call — subsequent calls at the same precision are instant.

Benchmarks measured on release build:

| Digits | First Computation (Cold Start) | Subsequent Calls (Cached / Hot) |
| :--- | :--- | :--- |
| **100** | ~25 ms | ~0.004 ms |
| **1 000** | ~1.5 s | ~0.003 ms |
| **5 000** | ~85 s | ~0.003 ms |

*Note: For computations beyond 1,000 digits with faster cold start times, use the compiled `ni-number-py-fast` version (powered by the GMP/Rug backend).*

---

## Related

- [ni-number](https://crates.io/crates/ni-number) — the core Rust library
- [Rust source](https://github.com/NiZaMinius/Ni-Number)

---

## License

MIT — see [LICENSE](LICENSE).
