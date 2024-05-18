// TODO: maybe need refactor, maybe not
import { watch, Ref, ref } from "vue";
import { stringMap } from "@utils/type";

// TODO: define an enum
export const dict = {
  time: ["time", "date"],
  text: ["text"],
};

function done(
  input: Ref<string>,
  newInput: string,
  keyword: string,
  datum: Ref<stringMap>,
  pattern: string,
  rValidator: (type: string, datum: stringMap) => false | string,
) {
  if (!newInput.startsWith(pattern + ": ")) return false;
  const res = rValidator(keyword, datum);
  if (res) return res;
  datum.value.type = keyword;
  input.value = input.value.slice(pattern.length + 2);
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
        const res = done(input, newInput, keyword, datum, i, rValidator);
        if (!res) continue;
        if (res != true) msg.value = res;
        return;
      }
    }
  });
  return msg;
}
