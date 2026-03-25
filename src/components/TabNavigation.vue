<script setup lang="ts">
defineProps<{
  activeTab: string;
}>();

defineEmits<{
  (e: 'update:activeTab', value: string): void;
}>();

const tabs = [
  { key: 'characters', label: 'Characters' },
  { key: 'game', label: 'Game Settings' },
  { key: 'content', label: 'Content' },
];
</script>

<template>
  <div class="flex gap-0 mt-6 mb-6 border-b border-[#1e3a5f]">
    <button
      v-for="tab in tabs"
      :key="tab.key"
      role="tab"
      @click="$emit('update:activeTab', tab.key)"
      class="relative px-6 py-3 text-sm font-medium transition-colors duration-200 select-none"
      :class="activeTab === tab.key ? 'text-[#93c5fd]' : 'text-[#7aadcc] hover:text-[#7aadcc]'"
    >
      {{ tab.label }}
      <span
        v-if="activeTab === tab.key"
        class="absolute bottom-0 left-0 right-0 h-px bg-gradient-to-r from-transparent via-[#60a5fa] to-transparent"
      />
    </button>
  </div>
</template>
