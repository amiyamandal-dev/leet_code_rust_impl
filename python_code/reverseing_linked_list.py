class Node:
    def __init__(self, value):
        self.value = value
        self.next = None
    
    
def reverse_linked_list(head:Node):
    
    if head is None or head.next is None:
        return head
    
    rez = reverse_linked_list(head.next)
    
    head.next.next = head
    head.next = None
    return rez