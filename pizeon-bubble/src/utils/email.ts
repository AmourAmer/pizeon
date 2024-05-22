import validator from "validator";

export function splitEmailAddress(str: string) {
  return str
    .replaceAll("\n", ";")
    .split(";")
    .filter((email) => validator.isEmail(email, { allow_display_name: true }));
}
