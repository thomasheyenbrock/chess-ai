import { Pairings } from "./types";

export function parsePairings(str: string): {
  pairings: Pairings;
  move: number;
} {
  const lines = str.trim().split("\n");
  const pairings: Pairings = {};
  for (const line of lines.slice(1)) {
    const [id, ...gameParts] = line.split(" ");
    const gameString = gameParts.join(" ");
    pairings[id] = JSON.parse(gameString);
  }
  return { pairings, move: parseInt(lines[0]) };
}

export function stringifyPairings({
  pairings,
  move,
}: {
  pairings: Pairings;
  move?: number;
}) {
  let stringified = move ? `${move}\n` : "";
  for (const id in pairings) {
    stringified += `${id} ${JSON.stringify(pairings[id])}\n`;
  }
  return stringified;
}
