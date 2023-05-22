s = input()
for i in range(4):
  print(i)
  print("R" * (3 - i))
  print("R" * (3 - i) in s)
  if "R" * (3 - i) in s:
    print(3 - i)
    break
  pass
