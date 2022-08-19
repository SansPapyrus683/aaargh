from random import randint

arr_len = randint(1, 20)
arr = [randint(1, 500) for _ in range(arr_len)]

print(arr_len)
print(' '.join(str(i) for i in arr))
