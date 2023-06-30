

def main():
  while True:
    c = 0
    n = int(input())
    if n == 0:
      break
    v = list(map(int, input().split()))
    peek = v[0]
    for i in range(1, n-1):
      if v[i-1] < v[i] and v[i] > v[i+1]:
        c += 1
    print(c)


main()
