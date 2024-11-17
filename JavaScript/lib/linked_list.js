class ListNode {
  #val;
  #next;

  /**
   * Creates a new ListNode.
   * @param {number} [val=0] - The value of the node.
   * @param {ListNode|null} [next=null] - The next node in the list.
   */
  constructor(val = 0, next = null) {
    this.#val = val;
    this.#next = next;
  }

  /**
   * Gets the value of the node.
   * @returns {number} The value of the node.
   */
  get val() {
    return this.#val;
  }

  /**
   * Sets the value of the node.
   * @param {number} val - The new value to set.
   */
  set val(val) {
    this.#val = val;
  }

  /**
   * Gets the next node in the list.
   * @returns {ListNode|null} The next node or null if none exists.
   */
  get next() {
    return this.#next;
  }

  /**
   * Sets the next node in the list.
   * @param {ListNode|null} next - The next node to set.
   */
  set next(next) {
    this.#next = next;
  }

  /**
   * Creates a linked list from an array of values.
   * @param {number[]} values - The array of values to convert into a linked list.
   * @returns {ListNode|null} The head of the linked list, or null if the array is empty.
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
   * @returns {number[]} The array of values.
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
