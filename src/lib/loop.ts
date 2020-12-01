/**
 *
 * @param a start
 * @param b end (not included)
 * @param step positive integer
 */
export function range(a: number, b?: number, step?: number): number[] {
  const isDefined = (x?: number) => typeof x === "number";

  if (!isDefined(b)) {
    return [...Array(a).keys()];
  }

  if (isDefined(b) && b) {
    const goingDown = b < a;
    let arr = [];

    if (!goingDown) {
      for (let x = a; x < b; x += step || 1) {
        arr.push(x);
      }
    }
    if (goingDown) {
      for (let x = a; x > b; x -= step || 1) {
        arr.push(x);
      }
    }
    return arr;
  }
  return [];
}
