// TODO: maybe need refactor, maybe not
import { watch, Ref } from "vue";
import { stringMap } from "@utils/type";
import SliceDatabase from "slice/SliceDatabase.vue";
import SliceHost from "slice/SliceHost.vue";
import SliceTextarea from "slice/SliceTextarea.vue";
import { toRefs } from "@vueuse/core";
import { db } from "src/components/workspace/slice/database";

// TODO: define an enum
export const dict = {
  title: ["title", "T"],
  db: ["db", "database"],
  host: ["host", "hostress"],
  time: ["time", "date"],
  place: ["place", "where"],
  link: ["link", "url"],
  tickbox: ["tickbox", "checkbox", "tick", "check"],
  text: ["text", "t"],
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
  Validator: (type: string, datum: Ref<stringMap>) => boolean,
) {
  // TODO: case insensitive
  if (!newInput.startsWith(pattern + ": ")) return false;
  if (!Validator(type, datum)) return false;
  // FIXME: re-focus, this problem is partially solved by vFocus, but not reliable itself. Also, should remember which to focus
  datum.value.type = type;
  datum.value[field] = newInput.slice(pattern.length + 2);

  return true;
}

export function useUpdateType(
  datum: Ref<stringMap>,
  fileds: string[],
  Validator: (type: string, datum: Ref<stringMap>) => boolean,
) {
  // Intended to watch input(map[key]) only, instead of with Validator.
  // To avoid multiple potential competing type change at a time
  if (!Array.isArray(fileds)) return;
  for (let field of fileds) {
    if (!datum.value[field]) datum.value[field] = "";
    watch(toRefs(datum)[field] as Ref<string>, (newInput) => {
      // Hahaha, toRefs, this is what I've been seeking for the whole morning. And thx vueuse!
      let type: keyof typeof dict;
      for (type in dict) {
        for (let i of dict[type]) {
          if (!done(newInput, i, type, datum, field, Validator)) continue;
          return;
        }
      }
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
      final.db = Object.fromEntries(db(datum));
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
