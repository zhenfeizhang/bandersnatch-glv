benchmarking cost for curve operation of Arkwork's implementation
------
AMD 5900x; ubuntu 20.04; arkworks 0.3.0; rust 1.52.1

# Edward curves ops

|   |  Jubjub | ed_on_bls12_377|
|:---|---| --- |
| rand base mul  |  75 us  | 75 us|
| fix base mul | 75 us  | 73 us |

# G1 ops

|   |  bls12-381 | bls12_377|
|:---|---| --- |
| rand base mul  |  124 us  | 125 us|
| fix base mul | 123 us  | 121 us |

# G2 ops

|   |  bls12-381 | bls12_377|
|:---|---| --- |
| rand base mul  |  365 us  | 432 us|
| fix base mul | 372 us  | 432 us |

