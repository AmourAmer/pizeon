// TODO: maybe need refactor, maybe not
import { watch, Ref, ref } from "vue";
import { stringMap } from "@utils/type";

// TODO: define an enum
export const dict = {
  time: ["time", "date"],
  text: ["text"],
  heading: ["heading", "h"],
};

function done(
  newInput: string,
  keyword: string,
  datum: Ref<stringMap>,
  pattern: string,
  rValidator: (type: string, datum: stringMap) => false | string,
) {
  if (!newInput.startsWith(pattern + ": ")) return false;
  const res = rValidator(keyword, datum.value);
  if (res) return res;
  datum.value.type = keyword;
  // FIXME: watch misses this last update, this pushed me to do init check on each slice
  // input.value = input.value.slice(pattern.length + 2);
  return true;
}

export function useUpdateType(
  input: Ref<string>,
  datum: Ref<stringMap>,
  rValidator: (type: string, datum: stringMap) => false | string,
) {
  const msg = ref("");
  // Intended to watch input only, instead of with rValidator.
  // To avoid multiple potential competing type change at a time
  watch(input, (newInput) => {
    let keyword: keyof typeof dict;
    for (keyword in dict) {
      for (let i of dict[keyword]) {
        const res = done(newInput, keyword, datum, i, rValidator);
        if (!res) continue;
        if (res != true) msg.value = res;
        return;
      }
    }
  });
  return msg;
}

// TODO: polish this according to demands, add hooks or something
export function useUpdateDatum(datum: Ref<stringMap>, map: stringMap) {
  for (let key in map) {
    let value = map[key];
    // Should be more elegant, failed. solved: a reactive effect is mutating its own dependencies
    watch(value, (newValue) => {
      datum.value[key] = newValue;
    });
  }
}

// FIXME: this fn is created because input misses last update
export function useInitCheck(datum: Ref<stringMap>, map: stringMap) {
  for (let key in map) {
    for (let keyword of dict[datum.value.type as keyof typeof dict]) {
      if (map[key].value?.startsWith(keyword + ": ")) {
        map[key].value = map[key].value.slice(keyword.length + 2);
        break;
      }
    }
  }
}
