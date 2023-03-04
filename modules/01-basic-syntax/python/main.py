import src.utils.utils as utils
import src.modules.compound_types as compound_types
import src.modules.references as references
import src.modules.slices as slices
import src.modules.functions as functions

def main():
  utils.print_h1(title="Basic syntax")

  # Compound types
  utils.print_h2(title="Compound types")
  compound_types.array_asignment_and_access()
  compound_types.tuple_assignment_and_access()
  utils.print_br()

  # References
  utils.print_h2(title="References")
  references.references()
  utils.print_br()

  # Slices
  utils.print_h2(title="Slices")
  slices.array_slices()
  slices.string_vs_str()
  utils.print_br()

  # Functions
  utils.print_h2(title="Functions")
  functions.fizzbuzz_to(10)
  functions.test_methods_from_rectangle()
  functions.test_overloading()
  utils.print_br()

if __name__ == "__main__":
  main()
