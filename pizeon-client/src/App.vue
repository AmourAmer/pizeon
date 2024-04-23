<script setup lang="ts"></script>

<template>
  <div class="container">
    <nav>
      <RouterLink :to="{ name: 'menu', params: { type: 'menu' } }"
        >Go to Menu</RouterLink
      >
      <RouterLink to="/meal">Go to Meal</RouterLink>
      <RouterLink to="/kitchen">Go to Kitchen</RouterLink>
      <RouterLink :to="{ name: 'menu', params: { type: 'fridge' } }"
        >Open the Fridge</RouterLink
      >
      <RouterLink :to="{ name: 'menu', params: { type: 'unwelcomed' } }"
        >Check outdoors</RouterLink
      >
      <RouterLink :to="{ name: 'menu', params: { type: 'junk' } }"
        >Examine trash can</RouterLink
      >
    </nav>
    <router-view v-slot="{ Component }">
      <transition>
        <keep-alive>
          <component :is="Component" />
        </keep-alive>
      </transition>
    </router-view>
  </div>
</template>
