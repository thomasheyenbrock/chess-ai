import { Pairings } from "./types";

export function parsePairings(str: string): Pairings {
  const lines = str.trim().split("\n");
  const pairings: Pairings = {};
  for (const line of lines) {
    const [id, ...gameParts] = line.split(" ");
    const gameString = gameParts.join(" ");
    pairings[id] = JSON.parse(gameString);
  }
  return pairings;
}

export function stringifyPairings(pairings: Pairings) {
  let stringified = "";
  for (const id in pairings) {
    stringified += `${id} ${JSON.stringify(pairings[id])}\n`;
  }
  return stringified;
}
