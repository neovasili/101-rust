def takes_u32(x: int):
  print(f"u32: {x}")

def takes_i8(y: float):
  print(f"i8: {y}")

# Types inference
def main():
  print("------------------------\n")

  x = 10
  y = 20

  takes_u32(x)
  takes_i8(y)
  takes_u32(y)

if __name__ == "__main__":
  main()
