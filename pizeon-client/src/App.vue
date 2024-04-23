<script setup lang="ts"></script>

<template>
  <div class="container">
    <nav>
      <RouterLink to="/menu">Go to Menu</RouterLink>
      <RouterLink to="/meal">Go to Meal</RouterLink>
      <RouterLink to="/kitchen">Go to Kitchen</RouterLink>
      <RouterLink to="/fridge">Open the Fridge</RouterLink>
      <RouterLink to="/unwelcomed">Check outdoors</RouterLink>
      <RouterLink to="/junk">Examine trash can</RouterLink>
    </nav>
    <RouterView />
  </div>
</template>
