import { PickMove } from "./type";

const pickMove: PickMove = function (game) {
  return game.possibleMoves[
    Math.floor(Math.random() * game.possibleMoves.length)
  ];
};

export default pickMove;
