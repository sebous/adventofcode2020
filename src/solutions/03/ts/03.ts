import { join } from "path";
import { cloneDeep } from "lodash";
import { readInputFile } from "../../../lib/input";

interface Coord {
  x: number;
  y: number;
}

interface State {
  grid: { [key: string]: boolean };
  treesCount: number;
  currentPos: Coord;
  currentGridWidth: number;
}

const coordToKey = ({ x, y }: Coord) => `${x},${y}`;
const keyToCoord = (k: string): Coord => {
  const [x, y] = k.split(",").map(Number);
  return { x, y };
};

let gridWidth = 0;
let gridHeight = 0;
const sourceGrid: { [key: string]: boolean } = {};

function calculateNext(slope: [number, number], state: State): State {
  let { treesCount, grid, currentPos, currentGridWidth } = state;
  const [TARGET_X, TARGET_Y] = slope;

  const nextPos: Coord = { x: currentPos.x + TARGET_X, y: currentPos.y + TARGET_Y };
  // if (nextPos.y + 1 > gridHeight) return;
  if (nextPos.x + 1 > currentGridWidth) {
    // copy grid right
    Object.entries(sourceGrid).forEach(([pos, val]) => {
      const coordPos = keyToCoord(pos);
      coordPos.x = coordPos.x + currentGridWidth;
      grid[coordToKey(coordPos)] = val;
    });
    currentGridWidth += gridWidth;
  }
  if (grid[coordToKey(nextPos)]) {
    treesCount += 1;
  }
  currentPos.x = nextPos.x;
  currentPos.y = nextPos.y;
  return { currentGridWidth, currentPos, grid, treesCount };
}

function createSourceGrid(input: string) {
  input.split("\n").forEach((line, y) => {
    if (y + 1 > gridHeight) {
      gridHeight = y + 1;
    }
    line.split("").forEach((spot, x) => {
      if (x + 1 > gridWidth) {
        gridWidth = x + 1;
      }
      sourceGrid[coordToKey({ x, y })] = spot === "#";
    });
  });
}

function countTreesInSlope(slope: [number, number]) {
  let grid = cloneDeep(sourceGrid);
  let currentGridWidth = gridWidth;
  let treesCount = 0;
  let currentPos: Coord = { x: 0, y: 0 };

  let state: State = { grid, currentGridWidth, treesCount, currentPos };

  while (state.currentPos.y + slope[1] + 1 <= gridHeight) {
    state = calculateNext(slope, state);
  }
  return state.treesCount;
}

const SLOPES: [number, number][] = [
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
];

async function run() {
  const input = await readInputFile(join(__dirname, "../data.txt"));
  createSourceGrid(input);

  const part1 = countTreesInSlope([3, 1]);
  console.log(`Part 1: ${part1}`);

  const resultsForSlopes = SLOPES.map((slope) => countTreesInSlope(slope));
  const part2 = resultsForSlopes.reduce((total, num) => total * num, 1);
  console.log(`Part 2: ${part2}`);
}

run();
