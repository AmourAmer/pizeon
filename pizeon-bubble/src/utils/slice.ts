import { watch, Ref, ref } from "vue";

const dict = {
  time: ["time"],
};

function done(
  input: Ref<string>,
  newInput: string,
  keyword: string,
  pattern: string,
  validator: (type: string) => true,
) {
  if (!newInput.startsWith(pattern + ": ")) return false;
  validator(keyword);
  // datum.value.type = keyword;
  input.value = input.value.slice(pattern.length + 1);
  return true;
}

export function useUpdateType(
  input: Ref<string>,
  validator: (type: string) => true,
) {
  const msg = ref("");
  watch(input, (newInput) => {
    let keyword: keyof typeof dict;
    for (keyword in dict) {
      for (let i of dict[keyword])
        try {
          if (done(input, newInput, keyword, i, validator)) return;
        } catch (e) {
          msg.value = JSON.stringify(e);
        }
    }
  });
  return msg;
}
