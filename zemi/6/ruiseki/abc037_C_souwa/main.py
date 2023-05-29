n, k = map(int, input().split())
a = list(map(int, input().split()))

cumulative_sum = [0] * (n + 1)
for i in range(n):
  cumulative_sum[i + 1] = cumulative_sum[i] + a[i]

total = 0
for i in range(n - k + 1):
  total += cumulative_sum[i + k] - cumulative_sum[i]

print(total)
