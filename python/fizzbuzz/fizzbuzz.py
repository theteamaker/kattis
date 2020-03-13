import sys

def is_divisible_by(number, divisor):
    if number % divisor == 0:
        return True
    return False

def fizzbuzz(number):
    results = []
    
    if is_divisible_by(number, fizz) is True:
        results.append("Fizz")
    
    if is_divisible_by(number, buzz) is True:
        results.append("Buzz")
    
    return results

for i in sys.stdin:
    raw_elements = i.split(" ")
    elements = []

    for element in raw_elements:
        elements.append(element.strip("\n"))
    
    fizz = int(elements[0])
    buzz = int(elements[1])
    max_number = int(elements[2])

    for i in range(1, max_number + 1):
        result = fizzbuzz(i)
        if len(result) == 0:
            print(i)
        
        else:
            to_print = ""
            for i in result:
                to_print += i
            print(to_print)