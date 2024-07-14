const IRIS_EPOCH: number = 1577836800;
const SEQUENCE_BITS: number = 8;
const WORKER_BITS: number = 5;
const ISSUER_BITS: number = 5;

export function getTimestamp(snowflake: number) {
    const timestamp = BigInt(snowflake) >> BigInt(WORKER_BITS + SEQUENCE_BITS + ISSUER_BITS);
    return new Date(Number(timestamp + BigInt(IRIS_EPOCH)) * 1000);
}

export function getTimestampFormatted(date: Date): string {
    const formatter = Intl.DateTimeFormat('en-US', { hour: '2-digit', minute: '2-digit' });
    return formatter.format(date);
}