<script setup lang="ts"></script>

<template>
  <div class="container">
    <nav>
      <RouterLink :to="{ name: 'repo', params: { type: 'Fresh' } }"
        >Go to Repo</RouterLink
      >
      <RouterLink to="/meals">Go to Meals</RouterLink>
      <RouterLink to="/kitchen">Go to Kitchen</RouterLink>
      <RouterLink :to="{ name: 'repo', params: { type: 'Fridge' } }"
        >Open the Fridge</RouterLink
      >
      <RouterLink :to="{ name: 'repo', params: { type: 'Unwelcomed' } }"
        >Check outdoors</RouterLink
      >
      <RouterLink :to="{ name: 'repo', params: { type: 'Junk' } }"
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
