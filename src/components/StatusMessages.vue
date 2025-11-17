<script setup lang="ts">
defineProps<{
  statusMessage?: string;
  errorMessage?: string;
}>();
</script>

<template>
  <div v-if="statusMessage || errorMessage" class="mt-4 space-y-3">
    <div v-if="statusMessage" class="flex items-center gap-3 bg-green-500/20 backdrop-blur-lg border border-green-300/30 rounded-2xl p-4 shadow-lg">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-white shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
      <span class="text-white font-medium">{{ statusMessage }}</span>
    </div>
    <div v-if="errorMessage" class="flex items-center gap-3 bg-red-500/20 backdrop-blur-lg border border-red-300/30 rounded-2xl p-4 shadow-lg">
      <svg xmlns="http://www.w3.org/2000/svg" class="stroke-white shrink-0 h-6 w-6" fill="none" viewBox="0 0 24 24"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z" /></svg>
      <span class="text-white font-medium">{{ errorMessage }}</span>
    </div>
  </div>
</template>
