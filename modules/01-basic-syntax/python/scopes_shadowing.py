# Scopes and shadowing

print("------------------------\n")

a = 10
print(f"before: {a}")

with open("scopes_shadowing.py", "r") as this_file:
  a = "hello"
  print(f"inner scope: {a}")

  a = True
  print(f"shadowed in inner scope: {a}")

print(f"after: {a}")
