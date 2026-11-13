type Shape =
  | { readonly kind: "circle"; readonly radius: number }
  | { readonly kind: "square"; readonly side: number };

export function area(shape: Shape): number {
  switch (shape.kind) {
    case "circle":
      return Math.PI * shape.radius ** 2;
    case "square":
      return shape.side ** 2;
  }
}

export function firstOr(values: readonly number[], fallback: number): number {
  return values[0] ?? fallback;
}

export function parseCount(text: string): number {
  const value: unknown = JSON.parse(text);
  if (typeof value !== "number") {
    throw new Error("count must be a number");
  }
  return value;
}

async function save(id: string): Promise<void> {
  await Promise.resolve(id);
}

export async function saveAll(ids: readonly string[]): Promise<void> {
  for (const id of ids) {
    await save(id);
  }
}
