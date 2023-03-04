import src.utils.utils as utils

# References
def references() -> None:
  utils.print_h3(title="References")

  x = 10
  ref_x = x
  ref_x = 20
  print(f"x: {x}")
  print(f"ref_x: {ref_x}")
  print("NOTE: references or pointers do not exists in Python")
