l = 256237
reverse = 0

while l > 0:
    last_digit = l % 10
    reverse =  reverse * 10 + last_digit
    l = int(l / 10)
    
print(reverse)