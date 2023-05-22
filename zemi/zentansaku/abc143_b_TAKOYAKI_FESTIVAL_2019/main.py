import itertools

N = int(input())
D = list(map(int, input().split()))
print(sum([x * y for x, y in itertools.combinations(D, 2)]))
