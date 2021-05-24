import { Result } from "../../move-generator";

export type Pairings = { [id: string]: Result | null };

export type Batch = { white: string; black: string }[];
