import ingredientHost from "ingredient/ingredientHost.vue";
export function useIngredientType(type: string) {
  switch (type) {
    default:
      return ingredientHost;
  }
}
