export type Vector = number[];
export type Matrix = number[][];

export function createRandomVector(
  rows: number,
  lowerBound: number = -2,
  upperBound: number = 2
): Vector {
  const vector: Vector = [];
  for (let i = 0; i < rows; i++) {
    vector.push(Math.random() * (upperBound - lowerBound) + lowerBound);
  }
  return vector;
}

export function createRandomMatrix(
  rows: number,
  columns: number,
  lowerBound: number = -2,
  upperBound: number = 2
): Matrix {
  const matrix: Matrix = [];
  for (let i = 0; i < rows; i++) {
    const row: number[] = [];
    for (let j = 0; j < columns; j++) {
      row.push(Math.random() * (upperBound - lowerBound) + lowerBound);
    }
    matrix.push(row);
  }
  return matrix;
}

function dotProduct(vector1: Vector, vector2: Vector) {
  let sum = 0;
  for (let i = 0; i < vector1.length; i++) {
    sum += vector1[i] * vector2[i];
  }
  return sum;
}

function sigmoid(x: number) {
  return 1 / (1 + Math.E ** -x);
}

export function step(activations: Vector, weights: Matrix, biases: Vector) {
  const result: Vector = [];
  for (let i = 0; i < weights.length; i++) {
    result.push(sigmoid(dotProduct(weights[i], activations) + biases[i]));
  }
  return result;
}
