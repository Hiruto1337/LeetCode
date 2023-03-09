class ListNode {
    val: number
    next: ListNode | null
    constructor(val?: number, next?: ListNode | null) {
        this.val = (val === undefined ? 0 : val)
        this.next = (next === undefined ? null : next)
    }
}

function detectCycle(head: ListNode | null): ListNode | null {
    if (head === null || head.val === 100001) {
        return head;
    }

    head.val = 100001;

    return detectCycle(head.next);
};
