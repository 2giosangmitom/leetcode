/**
 * @class ListNode
 * @description Represents a node in a singly linked list. Each node contains a value and a reference to the next node in the list.
 */
class ListNode {
  #val;
  #next;

  /**
   * Creates an instance of ListNode.
   * @constructor
   * @param {number} [val=0] - The value to store in the node. Defaults to 0 if not provided.
   * @param {ListNode|null} [next=null] - The next node in the list. Defaults to null if not provided.
   */
  constructor(val = 0, next = null) {
    this.#val = val;
    this.#next = next;
  }

  /**
   * Gets the value stored in the node.
   * @returns {number} The value of the node.
   */
  get val() {
    return this.#val;
  }

  /**
   * Sets the value stored in the node.
   * @param {number} val - The new value to set for the node.
   */
  set val(val) {
    this.#val = val;
  }

  /**
   * Gets the next node in the linked list.
   * @returns {ListNode|null} The next node, or null if this is the last node in the list.
   */
  get next() {
    return this.#next;
  }

  /**
   * Sets the next node in the linked list.
   * @param {ListNode|null} next - The node to set as the next node in the list. Can be null if this is the last node.
   */
  set next(next) {
    this.#next = next;
  }

  /**
   * Creates a linked list from an array of values.
   * @param {number[]} values - The array of values to convert into a linked list.
   * @returns {ListNode|null} The head node of the newly created linked list, or null if the array is empty.
   * @example
   * const list = ListNode.fromArray([1, 2, 3]);
   * // Returns a linked list: 1 -> 2 -> 3
   */
  static fromArray(values) {
    if (!Array.isArray(values) || values.length === 0) return null;
    const head = new ListNode(values[0]);
    let tail = head;
    for (let i = 1; i < values.length; i++) {
      tail.next = new ListNode(values[i]);
      tail = tail.next;
    }
    return head;
  }

  /**
   * Converts the linked list to an array of values.
   * @returns {number[]} An array representing the values of the linked list nodes in order.
   * @example
   * const list = new ListNode(1, new ListNode(2, new ListNode(3)));
   * console.log(list.toArray()); // Outputs: [1, 2, 3]
   */
  toArray() {
    const values = [];
    values.push(this.val);
    let node = this.#next;
    while (node) {
      values.push(node.val);
      node = node.next;
    }
    return values;
  }
}

export { ListNode };
