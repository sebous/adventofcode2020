import { join } from "path";
import { readInputFile } from "../../../lib/input";
import { range } from "../../../lib/loop";

interface SeatInfo {
  id: number;
  row: number;
  column: number;
}

async function run() {
  const input = await readInputFile(join(__dirname, "../data.txt"));
  const data = input.split("\n").map((row) => row.split(""));

  const seatInfos: SeatInfo[] = data.map((code) => {
    let rowLow = 0;
    let rowHigh = 127;

    range(7).forEach((i) => {
      if (code[i] === "F") {
        rowHigh = rowLow + Math.floor((rowHigh - rowLow) / 2);
      }
      if (code[i] === "B") {
        rowLow = rowLow + Math.ceil((rowHigh - rowLow) / 2);
      }
    });

    let colLow = 0;
    let colHigh = 7;
    range(7, 11).forEach((i) => {
      if (code[i] === "R") {
        colLow = colLow + Math.ceil((colHigh - colLow) / 2);
      }
      if (code[i] === "L") {
        colHigh = colLow + Math.floor((colHigh - colLow) / 2);
      }
    });
    return {
      row: rowLow,
      column: colLow,
      id: rowLow * 8 + colLow,
    };
  });

  const highestId = seatInfos.reduce((a, b) => (a.id > b.id ? a : b)).id;
  console.log(`Part 1: ${highestId}`);

  const seatIds = seatInfos.filter((seat) => seat.row !== 127 && seat.row !== 0).map((s) => s.id);
  let mySeatId = 0;
  for (let id of seatIds) {
    if (!seatIds.includes(id + 1) && seatIds.includes(id + 2)) {
      mySeatId = id + 1;
      break;
    }
    if (!seatIds.includes(id - 1) && seatIds.includes(id - 2)) {
      mySeatId = id - 1;
      break;
    }
  }
  console.log(`Part 2: ${mySeatId}`);
}

run();
