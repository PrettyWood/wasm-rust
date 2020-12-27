function firstFactor(x: bigint): bigint {
  if (x % 2n == 0n) { return 2n; }
  let i = 3n;
  while (i * i <= x) {
    if (x % i == 0n) { return i; }
    i += 2n;
  }
  return x;
}

export function isPrime(n : bigint): boolean {
  if (n <= 1) { return false; }
  return firstFactor(n) == n
}
