import sys
from typing import List
from itertools import combinations


left, right = 0, 10**9+1

N = int(input().strip())
A = [list(map(int, input().strip().split())) for _ in range(N)]


def check(x: int) -> bool:
  S = set()
  for a in A:
    bit = sum(1 << i for i in range(5) if a[i] >= x)
    S.add(bit)
  for a in S:
    for b in S:
      for c in S:
        if a | b | c == 31:  # ビットが全て立っているかどうか
          return True
  return False


while right - left > 1:
  mid = (left + right) // 2
  if check(mid):
    left = mid
  else:
    right = mid

print(left)
