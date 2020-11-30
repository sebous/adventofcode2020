import { readFile } from "fs/promises";

export async function readInputFile(path: string) {
  try {
    const data = await readFile(path, { encoding: "utf8" });
    return data;
  } catch (err) {
    console.log(err);
  }
}
