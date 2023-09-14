function climbStairs(n: number): number {
  if (n === 1 || n === 2) return n;
  let a = 1;
  let b = 2;
  for (let i = 3; i < n; i++) {
    let temp = b;
    b += a;
    a = temp;
  }
  return a + b;
}

export default climbStairs;
