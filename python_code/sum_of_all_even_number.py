from typing import List
def sum_of_all_even_number(input_list:List[int]):
    all_sum = 0
    for i in input_list:
        if i%2 == 0:
            all_sum += i
    return all_sum
