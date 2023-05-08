s = input()
for i in range(4):
  if "R" * (3 - i) in s:
    print(3 - i)
    break
  pass
