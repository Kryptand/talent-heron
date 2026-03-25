<script setup lang="ts">
defineProps<{
  statusMessage?: string;
  errorMessage?: string;
  infoMessage?: string;
}>();
</script>

<template>
  <div v-if="statusMessage || errorMessage || infoMessage" class="space-y-2 mb-5">
    <div v-if="statusMessage" class="flex items-center gap-2.5 bg-emerald-900/15 border border-emerald-900/30 rounded-lg px-4 py-2.5">
      <svg class="w-4 h-4 text-emerald-500 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
      </svg>
      <span class="text-sm text-emerald-400">{{ statusMessage }}</span>
    </div>
    <div v-if="errorMessage" class="flex items-center gap-2.5 bg-red-900/15 border border-red-900/30 rounded-lg px-4 py-2.5">
      <svg class="w-4 h-4 text-red-400 shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
      </svg>
      <span class="text-sm text-red-400">{{ errorMessage }}</span>
    </div>
    <div v-if="infoMessage" class="flex items-center gap-2.5 bg-[#0e1d33] border border-[#1e3a5f] rounded-lg px-4 py-2.5">
      <svg class="w-4 h-4 text-[#7aadcc] shrink-0" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="2">
        <path stroke-linecap="round" stroke-linejoin="round" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
      </svg>
      <span class="text-sm text-[#7aadcc]">{{ infoMessage }}</span>
    </div>
  </div>
</template>
