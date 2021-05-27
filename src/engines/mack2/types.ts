import { Game } from "../../move-generator";

export type Pairings = { [id: string]: Game | null };

export type Batch = { white: string; black: string }[];
