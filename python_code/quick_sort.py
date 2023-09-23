from typing import List

def quick_sort(array_val:List[int]):
    if len(array_val) <= 1:
        return array_val
    
    pivot_element = array_val[-1]
    greater_val = list()
    equal_val = list()
    smaller_val = list()
    
    for i in array_val:
        if i < pivot_element:
            smaller_val.append(i)
        elif i > pivot_element:
            greater_val.append(i)
        else:
            equal_val.append(i)
    return quick_sort(smaller_val) + equal_val + quick_sort(greater_val)

z = quick_sort([1,5,3,8,5,2,3,4,7,56,2,45,6,34,3,545,67,535,323,523,23,87686,3423,12313,65568568,341231231231,3131313,11,2,3,4,5,3,3,325,45])
print(z)