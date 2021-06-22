benchmarking cost for curve operation of Arkwork's implementation
------

# Edward curves ops

|   |  Jubjub | ed_on_bls12_377|
|:---|---| --- |
| rand base mul  |  75 ns  | 75 ns|
| fix base mul | 75 ns  | 73 ns |

# G1 ops

|   |  bls12-381 | bls12_377|
|:---|---| --- |
| rand base mul  |  124 ns  | 125 ns|
| fix base mul | 123 ns  | 121 ns |

# G2 ops

|   |  bls12-381 | bls12_377|
|:---|---| --- |
| rand base mul  |  365 ns  | 432 ns|
| fix base mul | 372 ns  | 432 ns |

