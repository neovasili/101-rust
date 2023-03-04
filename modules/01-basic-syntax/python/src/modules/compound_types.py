import src.utils.utils as utils

# Array assignment and access
def array_asignment_and_access() -> None:
  utils.print_h3(title="Array assignment and access")

  my_ints_array = [42] * 10
  my_ints_array[5] = 0
  print(f"my_ints_array: {my_ints_array}")

# Tuple assignment and access
def tuple_assignment_and_access() -> None:
  utils.print_h3(title="Tuple assignment and access")

  my_mixed_types_tuple = (7, True)
  print(f"1st index: {my_mixed_types_tuple[0]}")
  print(f"2nd index: {my_mixed_types_tuple[1]}")
