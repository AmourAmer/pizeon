// TODO: maybe need refactor, maybe not
import { watch, Ref } from "vue";
import { stringMap } from "@utils/type";
import sliceTitle from "slice/sliceTitle.vue";
import sliceDatabase from "slice/sliceDatabase.vue";
import sliceHost from "slice/sliceHost.vue";
import sliceTime from "slice/sliceTime.vue";
import slicePlace from "slice/slicePlace.vue";
import sliceLink from "slice/sliceLink.vue";
import sliceTickbox from "slice/sliceTickbox.vue";
import sliceTextarea from "slice/sliceTextarea.vue";

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

const sliceType = {
  title: sliceTitle,
  db: sliceDatabase,
  host: sliceHost,
  time: sliceTime,
  place: slicePlace,
  link: sliceLink,
  tickbox: sliceTickbox,
  text: sliceTextarea,
};

export function useSliceType(types = Object.keys(sliceType)) {
  return (type: string) => {
    {
      let idx = types.indexOf(type);
      if (idx == -1) idx = types.length - 1;
      if (types[idx] in sliceType)
        return sliceType[types[idx] as keyof typeof sliceType];
      return sliceTextarea;
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
