import { stringMap } from "@utils/type";

export function parseDB(datum: stringMap) {
  return (datum["body"] as string).split("\n").map((it) => it.split("\t"));
}

export function process_db(datum: stringMap) {
  // FIXME: fucking xlsx merged cells
  let mapper = parseDB(datum);
  // TODO: warn on seen empties
  if (datum.empty == "follow") {
    for (let i = 1; i < mapper.length; i++) {
      for (let j = 1; j < mapper[i].length; j++) {
        if (!mapper[i][j]) mapper[i][j] = mapper[i - 1][j];
      }
    }
  } else if (datum.empty == "empty") {
  }
  // TODO: warn on such cases
  let fields: string[];
  if (datum.first_line_as_fieldsname) {
    fields = mapper[0] || [];
    mapper = mapper.slice(1);
  } else {
    fields =
      datum.fieldsname ||
      Array.from({ length: mapper[0].length }, (_, i) => "v" + i);
  }
  const warning = [];
  let key: number | undefined = datum.key;
  if (!fields.length) warning.push("Should have at least one field!");
  if (fields.filter((f) => !f?.length).length)
    warning.push("Fields name shouldn't be empty!");
  if (key == undefined) {
    warning.push("Should have set a key field!");
    key = 0;
  }
  // TODO: check uniqness
  return { fields, mapper, key, warning };
}

export const db = (datum: stringMap) => {
  try {
    const { fields, mapper, key } = process_db(datum);
    if (key == undefined) throw "TODO: in database.ts export const db";
    return new Map(
      mapper
        .filter((l) => l[key])
        .map((l) => {
          const obj: stringMap = {};
          fields.forEach((f, i) => {
            if (i == key) return;
            obj[f] = l[i];
          });
          return [l[key], obj];
        }),
    );
  } catch {
    return new Map();
  }
};
