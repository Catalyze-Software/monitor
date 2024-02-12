export function convertTimestamp(timestamp: bigint): string {
  const date = new Date(Number(timestamp / 1000000n))
  return date.toLocaleDateString()
}

export function convertTimestampToDateTime(timestamp: bigint): string {
  const date = new Date(Number(timestamp / 1000000n))
  return date.toLocaleString()
}
