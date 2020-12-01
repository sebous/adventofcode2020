(async () => {
  const date = new Date().getDate();

  for (let x = 1; x <= date; x++) {
    const datePadded = x < 10 ? `0${x}` : x;
    console.log(`Day ${datePadded}:`);
    await import(`./solutions/${datePadded}/ts/${datePadded}`);
  }
})();
