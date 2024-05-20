// TODO: maybe need refactor, maybe not
import { watch, Ref } from "vue";
import { stringMap } from "@utils/type";

// TODO: define an enum
export const dict = {
  title: ["title"],
  host: ["host", "hostress"],
  time: ["time", "date"],
  place: ["place", "where"],
  text: ["text"],
};

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

// TODO: replace parts in old slices
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
