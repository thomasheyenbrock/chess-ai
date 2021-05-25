import * as path from "path";

export const NUMBER_OF_NETWORKS = 100;
export const CONCURRENT_GAMES = 100;
export const SURVIVORS = 5;
export const REPRODUCTION = 15;

export const PAIRINGS_FILENAME = path.join(
  __dirname,
  `generation${process.argv[2]}`,
  "pairings.json"
);
