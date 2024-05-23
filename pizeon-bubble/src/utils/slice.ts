// TODO: maybe need refactor, maybe not
import { watch, Ref } from "vue";
import { stringMap } from "@utils/type";
import SliceDatabase from "slice/SliceDatabase.vue";
import SliceHost from "slice/SliceHost.vue";
import SliceTextarea from "slice/SliceTextarea.vue";

// TODO: define an enum
export const dict = {
  title: ["title"],
  db: ["db", "database"],
  host: ["host", "hostress"],
  time: ["time", "date"],
  place: ["place", "where"],
  link: ["link", "url"],
  tickbox: ["tickbox", "checkbox", "tick", "check"],
  text: ["text"],
};

const sliceType = new Map([
  ["title", SliceTextarea],
  ["db", SliceDatabase],
  ["host", SliceHost],
  ["time", SliceTextarea],
  ["place", SliceTextarea],
  ["link", SliceTextarea],
  ["tickbox", SliceTextarea],
  ["text", SliceTextarea],
]);

export function useSliceType(types = [...sliceType.keys()]) {
  return (type: string) => {
    {
      let idx = types.indexOf(type);
      if (idx == -1) idx = types.length - 1;
      if (sliceType.has(types[idx])) return sliceType.get(types[idx]);
      return SliceTextarea;
    }
  };
}

// TODO: config file
export function useNextSliceType() {
  return (type: string) => {
    switch (type) {
      case "tickbox":
        return "tickbox";
      default:
        return "text";
    }
  };
}

function done(
  newInput: string,
  pattern: string,
  type: string,
  datum: Ref<stringMap>,
  field: string,
  rValidator: (type: string, datum: Ref<stringMap>) => boolean,
) {
  // TODO: case insensitive
  if (!newInput.startsWith(pattern + ": ")) return false;
  if (rValidator(type, datum)) return false;
  // FIXME: re-focus
  datum.value.type = type;
  // BUG: if type keeps, map[field] changes but doesn't update patch
  datum.value[field] = newInput.slice(pattern.length + 2);

  return true;
}

export function useUpdateType(
  datum: Ref<stringMap>,
  map: stringMap,
  rValidator: (type: string, datum: Ref<stringMap>) => boolean,
) {
  // Intended to watch input(map[key]) only, instead of with rValidator.
  // To avoid multiple potential competing type change at a time
  for (let field in map) {
    watch(map[field] as Ref<string>, (newInput) => {
      let type: keyof typeof dict;
      for (type in dict) {
        for (let i of dict[type]) {
          if (!done(newInput, i, type, datum, field, rValidator)) continue;
          return;
        }
      }
    });
  }
}

export function useBindDatum(datum: Ref<stringMap>, map: stringMap) {
  for (let field in map) {
    if (datum.value[field]) {
      map[field].value = datum.value[field];
    }

    watch(map[field] as Ref<string>, (newInput) => {
      datum.value[field] = newInput;
    });
  }
}

export function useFinalize(datum: stringMap) {
  const final: stringMap = { type: datum.type };
  switch (datum.type) {
    case "title":
    case "time":
    case "place":
    case "link":
    case "text":
      if (!datum.body) return null;
      final.body = datum.body;
      break;
    case "db":
      if (!datum.body) return null;
      final.db = new Map(); // FIXME:
      break;
    case "host":
      if (!datum.name) return null;
      final.name = datum.name;
      final.title = datum.title;
      final.description = datum.description;
      break;
    case "tickbox":
      if (!datum.body) return null;
      final.body = datum.body;
      final.id = datum.id;
      break;
  }
  return final;
}
