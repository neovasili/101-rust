import os
import src.utils.utils as utils
from typing import Any

# FizzBuzz functions
def _is_divisible_by(number: int, divisible_by: int) -> bool:
  if divisible_by == 0:
    return False

  return number % divisible_by == 0

def _fizzbuzz(number: int) -> None:
  divisible_by_3 = _is_divisible_by(number=number, divisible_by=3)
  divisible_by_5 = _is_divisible_by(number=number, divisible_by=5)

  if divisible_by_3 and divisible_by_5: print("fizzbuzz")
  if divisible_by_3 and not divisible_by_5: print("fizz")
  if not divisible_by_3 and divisible_by_5: print("buzz")
  if not divisible_by_3 and not divisible_by_5: print(number)

def fizzbuzz_to(number: int) -> None:
  utils.print_h3(title="FizzBuzz")

  for index in range(1, number + 1):
    _fizzbuzz(index)

# Methods
class Rectangle:
  width: int
  height: int

  def __init__(self, width: int, height: int) -> None:
    self.width = width
    self.height = height

  def area(self) -> int:
    return self.width * self.height

  def inc_width(self, delta: int) -> None:
    self.width += delta

def test_methods_from_rectangle() -> None:
  utils.print_h3(title="Methods")

  rectangle = Rectangle(width=10, height=5)
  print(f"old area: {rectangle.area()}")
  rectangle.inc_width(delta=5)
  print(f"new area: {rectangle.area()}")

def __pick_one(fisrt_element: Any, second_element: Any) -> Any:
  return fisrt_element if os.getpid() % 2 == 0 else second_element

def test_overloading() -> None:
  utils.print_h3(title="Overloading")

  print(f'coin toss: {__pick_one(fisrt_element="heads", second_element="tails")}')
  print(f'cash prize: {__pick_one(fisrt_element=500, second_element=1000)}')
