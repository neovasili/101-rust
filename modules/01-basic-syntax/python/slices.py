print("------------------------\n")
# Array slices
print("Array slices")

my_ints_array = [10, 20, 30, 40, 50, 60]
print(f"my_ints_array: {my_ints_array}")

my_ints_slice = my_ints_array[2:4]
print(f"my_ints_slice: {my_ints_slice}")

print("------------------------\n")
# String vs &str
print("String vs &str")

s1 = "World"
print(f"s1: {s1}")

s2 = "Hello "
print(f"s2: {s2}")
s2 += s1
print(f"s2: {s2}")

s3 = s2[6:]
print(f"s3: {s3}")
print("NOTE: Python does not have different types of string")
