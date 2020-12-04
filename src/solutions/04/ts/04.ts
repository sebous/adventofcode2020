import { join } from "path";
import { readInputFile } from "../../../lib/input";

type PassportKey = "byr" | "iyr" | "eyr" | "hgt" | "hcl" | "ecl" | "pid" | "cid";
const PASSPORT_KEYS = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
type PassportStore = { [key in PassportKey]: string }[];
type ValidationFn = (passport: { [key in PassportKey]: string }) => boolean;

function parseInput(input: string): PassportStore {
  const lines = input.split("\n");
  let currentItem = 0;
  const passwordStore: PassportStore = [];
  for (let x = 0; x < lines.length; x++) {
    if (lines[x] === "") {
      currentItem += 1;
      continue;
    }
    const keyValuePairs = lines[x]
      .split(" ")
      .map((str) => ({ key: str.split(":")[0], value: str.split(":")[1] }));
    const kvObject = keyValuePairs.reduce((o, prop) => ({ ...o, [prop.key]: prop.value }), {});
    passwordStore[currentItem] = { ...passwordStore[currentItem], ...kvObject };
  }
  return passwordStore;
}

const rules: { [key in PassportKey]: RegExp } = {
  byr: /^(19[2-9]\d)|(200[0-2])$/,
  iyr: /^20((1\d)|(20))$/,
  eyr: /^20((2\d)|(30))$/,
  hgt: /^((1[5-8]\dcm)|(19[0-3]cm)|((59)|(6\d)|(7[0-6]))in)$/,
  hcl: /^#[\da-f]{6}$/,
  ecl: /^(amb)|(blu)|(brn)|(gry)|(grn)|(hzl)|(oth)$/,
  pid: /^\d{9}$/,
  cid: /./,
};

function validateComplex(pasportStore: PassportStore): { valid: PassportStore; invalid: PassportStore } {
  const validatedStore: { valid: PassportStore; invalid: PassportStore } = { valid: [], invalid: [] };
  pasportStore.forEach((pass) => {
    const valid = Object.entries(rules).every(
      ([key, rule]) => key === "cid" || (pass[key as PassportKey] && pass[key as PassportKey].match(rule))
    );
    if (valid) validatedStore.valid.push(pass);
    else validatedStore.invalid.push(pass);
  });
  return validatedStore;
}

function validateSimple(passportStore: PassportStore): { valid: PassportStore; invalid: PassportStore } {
  const validatedStore: { valid: PassportStore; invalid: PassportStore } = { valid: [], invalid: [] };
  passportStore.forEach((pass) => {
    const passportKeys = Object.keys(pass);
    const missingKeys = PASSPORT_KEYS.filter((reqKey) => !passportKeys.includes(reqKey));
    if (missingKeys.length > 0) {
      validatedStore.invalid.push(pass);
      return;
    }
    validatedStore.valid.push(pass);
  });
  return validatedStore;
}

async function run() {
  const input = await readInputFile(join(__dirname, "../data.txt"));
  const passwordStore = parseInput(input);
  const validatedStore = validateSimple(passwordStore);
  console.log(`Part 1: ${validatedStore.valid.length}`);

  const complexValidatedStore = validateComplex(passwordStore);
  console.log(`Part 2: ${complexValidatedStore.valid.length}`);
}

run();
