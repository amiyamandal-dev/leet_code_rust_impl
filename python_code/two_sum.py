from typing import List

def two_sum(input_list:List[int], target:int=0):
    visited_val = set()
    
    for i in input_list:
        diff = target - i
        if diff in visited_val:
            return [i, diff]
        visited_val.add(i)
    return None
        
        
