export const chunks = (array: Uint8Array, chunkSize: number): Uint8Array[] =>
  Array.from({ length: Math.ceil(array.length / chunkSize) }, (_, i) =>
    array.slice(i * chunkSize, i * chunkSize + chunkSize)
  );
