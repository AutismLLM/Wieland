type Shape =
  | { readonly kind: "circle"; readonly radius: number }
  | { readonly kind: "square"; readonly side: number };

export function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2;
    default:
      return 0;
  }
}

export function first(values: readonly number[]): number {
  return values[0]!;
}

export function parse(text: string): number {
  const value: any = JSON.parse(text);
  return value.count;
}

export function port(raw: string): number {
  // @ts-ignore
  return parseInt(raw, 10);
}

async function save(id: string): Promise<void> {
  await Promise.resolve(id);
}

export function saveAll(ids: readonly string[]): void {
  for (const id of ids) {
    save(id);
  }
}
