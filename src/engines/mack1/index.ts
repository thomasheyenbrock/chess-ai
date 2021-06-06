import { PickMove } from "../type";

const pickMove: PickMove = function (game) {
  const possibleMoves = Object.values(game.possibleMoves);
  return possibleMoves[Math.floor(Math.random() * possibleMoves.length)];
};

export default pickMove;
