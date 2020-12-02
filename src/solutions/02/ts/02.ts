import path from "path";
import { readInputFile } from "../../../lib/input";

interface Instruction {
  minMax: [number, number];
  letter: string;
  password: string;
}

function countChars(str: string, char: string) {
  return str
    .split("")
    .reduce((acc, curr) => (curr === char ? (acc += 1) : acc), 0);
}

function validateLetterExistenceTimes({
  letter,
  minMax,
  password,
}: Instruction) {
  const letterCount = countChars(password, letter);
  const [min, max] = minMax;
  if (letterCount >= min && letterCount <= max) return true;
  return false;
}

function validateExactlyOneAtCorrectIndex({
  letter,
  minMax,
  password,
}: Instruction) {
  const [min, max] = minMax;
  const low = min - 1;
  const high = max - 1;

  if (password[low] === letter && password[high] !== letter) return true;
  if (password[low] !== letter && password[high] === letter) return true;
  return false;
}

async function run() {
  const input = await readInputFile(path.join(__dirname, "../data.txt"));
  const inputData: Instruction[] = input.split("\n").map((str) => {
    const splitted = str.split(" ");

    return {
      minMax: [
        Number(splitted[0].split("-")[0]),
        Number(splitted[0].split("-")[1]),
      ],
      letter: splitted[1][0],
      password: splitted[2],
    };
  });

  const validData = inputData.filter(validateLetterExistenceTimes);
  console.log(`Part 1: ${validData.length}`);

  const validPart2 = inputData.filter(validateExactlyOneAtCorrectIndex);
  console.log(`Part 2: ${validPart2.length}`);
}

run();
