class ListNode:
    def __init__(self, value=0, next=None):
        self.value = value
        self.next = next
        
        
def tortoise_and_hare(head:ListNode):
    tortoise = hare = head
    
    while hare != None and hare.next != None:
        hare = hare.next.next
        tortoise = tortoise.next
        
        if hare == tortoise:
            return True
        
    return False