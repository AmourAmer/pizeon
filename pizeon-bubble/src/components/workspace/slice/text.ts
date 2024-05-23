export let placeholderFn = (destinations: string[]) => {
  const msg = (dest: string) =>
    "What notice do you want to send to " + dest + "?";
  switch (destinations.length) {
    case 0:
      return "Please choose a server or email address to send notice to.";
    case 1:
      return msg(destinations[0]);
    case 2:
      return msg(destinations[0] + " and " + destinations[1]);
    default:
      return msg(
        destinations.slice(0, -1).join(", ") +
          ", and " +
          destinations.slice(-1),
      );
  }
};
