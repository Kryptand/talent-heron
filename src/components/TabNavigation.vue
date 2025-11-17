<script setup lang="ts">
defineProps<{
  activeTab: string;
}>();

defineEmits<{
  (e: 'update:activeTab', value: string): void;
}>();
</script>

<template>
  <div role="tablist" class="flex gap-2 p-1 bg-white/10 backdrop-blur-md rounded-2xl border border-white/20 mt-6 mb-6">
    <button
      role="tab"
      @click="$emit('update:activeTab', 'characters')"
      :class="[
        'flex-1',
        'font-semibold',
        'py-3',
        'px-4',
        'rounded-xl',
        'transition-all',
        'duration-300',
        {
          'bg-white/30 backdrop-blur-lg shadow-lg text-white font-bold': activeTab === 'characters',
          'text-white/80 hover:text-white hover:bg-white/10': activeTab !== 'characters'
        }
      ]"
    >
      Active Characters
    </button>
    <button
      role="tab"
      @click="$emit('update:activeTab', 'game')"
      :class="[
        'flex-1',
        'font-semibold',
        'py-3',
        'px-4',
        'rounded-xl',
        'transition-all',
        'duration-300',
        {
          'bg-white/30 backdrop-blur-lg shadow-lg text-white font-bold': activeTab === 'game',
          'text-white/80 hover:text-white hover:bg-white/10': activeTab !== 'game'
        }
      ]"
    >
      Game Settings
    </button>
    <button
      role="tab"
      @click="$emit('update:activeTab', 'content')"
      :class="[
        'flex-1',
        'font-semibold',
        'py-3',
        'px-4',
        'rounded-xl',
        'transition-all',
        'duration-300',
        {
          'bg-white/30 backdrop-blur-lg shadow-lg text-white font-bold': activeTab === 'content',
          'text-white/80 hover:text-white hover:bg-white/10': activeTab !== 'content'
        }
      ]"
    >
      Content Settings
    </button>
  </div>
</template>
