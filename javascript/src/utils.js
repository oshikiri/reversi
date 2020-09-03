export const sleep = (milliSeconds) =>
  new Promise((resolve) => setTimeout(resolve, milliSeconds));
