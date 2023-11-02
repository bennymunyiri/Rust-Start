# def fibonacci_recursive(n):
#     if n <= 0:
#         return "Invalid input"
#     elif n == 1:
#         return 0
#     elif n == 2:
#         return 1
#     else:
#         return fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)

# # Example usage
# n = 10  # Replace 10 with any positive integer to find the nth Fibonacci number
# result = fibonacci_recursive(n)
# print(f"The {n}th Fibonacci number is: {result}")

def fibonacci_iterative(n):
    if n <= 0:
        return "Invalid input"
    elif n == 1:
        return 0
    elif n == 2:
        return 1
    
    fib = [0, 1]
    for i in range(2, n):
        fib.append(fib[i - 1] + fib[i - 2])
    
    return fib[n - 1]

# Example usage
n = 10  # Replace 10 with any positive integer to find the nth Fibonacci number
result = fibonacci_iterative(n)
print(f"The {n}th Fibonacci number is: {result}")