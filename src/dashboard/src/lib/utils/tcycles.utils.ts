// Convert to number with 3 decimals
export function cyclesToT(cycles: bigint): number {
  return Number(cycles / 1_000_000_000n) / 1000
}
