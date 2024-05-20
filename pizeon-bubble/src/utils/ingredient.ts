import ingredientHost from "ingredient/ingredientHost.vue";
import ingredientText from "ingredient/ingredientText.vue";
export function useIngredientType(type: string) {
  switch (type) {
    case "host":
      return ingredientHost;
    default:
      return ingredientText;
  }
}
