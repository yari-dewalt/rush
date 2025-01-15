export function createSettings() {
  let settings = $state({
    interval: 1000,
  });

  return {
    get interval() { return settings.interval },
    set interval(value) { settings.interval = value },
  };
}
