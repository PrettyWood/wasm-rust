import { isPrime as isPrimeRust } from "wasm-rust";
import { isPrime as isPrimeTS } from "./lib/prime";

function checkIsPrime(e: Event, numberToCheck: string): void {
  e.preventDefault();

  let n: BigInt;
  try {
    n = BigInt(numberToCheck);
  } catch {
    throw new Error("Not a valid number");
  }

  const t0Rust = performance.now();
  const isPrimeRustNumber = isPrimeRust(n);
  const t1Rust = performance.now();

  const t0TS = performance.now();
  const isPrimeTSNumber = isPrimeTS(n as bigint);
  const t1TS = performance.now();

  alert(`
    Rust: ${isPrimeRustNumber ? "YES!" : "NO!"} in ${t1Rust - t0Rust}ms
    ----------------------------------
    TS: ${isPrimeTSNumber ? "YES!" : "NO!"} in ${t1TS - t0TS}ms
  `);

  (document.getElementById("numtocheck") as HTMLInputElement).value = "";
}

(window as any).checkIsPrime = checkIsPrime;
