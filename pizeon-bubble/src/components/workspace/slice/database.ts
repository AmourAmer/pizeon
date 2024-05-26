import { Ref } from "vue";
import { stringMap } from "@utils/type";

// I don't if it's reasonable to split such a file
export const db = (datum: stringMap) => {
  try {
    // FIXME: fucking xlsx merged cells
    const mapper = (datum["body"] as string)
      .split("\n")
      .map((it) => it.split("\t"));
    // TODO: warn on seen empties
    if (datum.empty == "inherit") {
      for (let i = 1; i < mapper.length; i++) {
        for (let j = 1; j < mapper[i].length; j++) {
          if (!mapper[i][j]) mapper[i][j] = mapper[i - 1][j];
        }
      }
    } else if (datum.empty == "emty") {
    }
    return new Map(mapper.filter((l) => l[0]).map((l) => [l.shift(), l]));
  } catch (e) {
    return new Map();
  }
};
