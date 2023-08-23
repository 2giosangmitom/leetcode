function convertToTitle(columnNumber: number): string {
  const result: number[] = [];
  while (columnNumber > 0) {
    columnNumber -= 1;
    result.push("A".charCodeAt(0) + (columnNumber % 26));
    columnNumber = Math.floor(columnNumber / 26);
  }
  return String.fromCharCode(...result.reverse());
}

export default convertToTitle;
