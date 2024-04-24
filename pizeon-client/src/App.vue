<script setup lang="ts"></script>

<template>
  <div class="container">
    <nav>
      <RouterLink :to="{ name: 'menu', params: { type: 'Fresh' } }"
        >Go to Menu</RouterLink
      >
      <RouterLink to="/meals">Go to Meals</RouterLink>
      <RouterLink to="/kitchen">Go to Kitchen</RouterLink>
      <RouterLink :to="{ name: 'menu', params: { type: 'Fridge' } }"
        >Open the Fridge</RouterLink
      >
      <RouterLink :to="{ name: 'menu', params: { type: 'Unwelcomed' } }"
        >Check outdoors</RouterLink
      >
      <RouterLink :to="{ name: 'menu', params: { type: 'Junk' } }"
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
