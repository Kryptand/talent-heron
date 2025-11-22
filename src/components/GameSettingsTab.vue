<script setup lang="ts">
import StatusMessages from "./StatusMessages.vue";

defineProps<{
  wowPath: string;
  isScanning: boolean;
}>();

defineEmits<{
  (e: 'update:wowPath', value: string): void;
  (e: 'find:path'): void;
  (e: 'scan:characters'): void;
  (e: 'save:settings'): void;
}>();
</script>

<template>
  <div class="space-y-4">
    <!-- WoW Installation -->
    <div class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
      <div class="p-6">
        <h2 class="text-2xl font-bold text-white mb-4 drop-shadow-lg">WoW Installation Path</h2>
        <div class="flex gap-2 flex-wrap">
          <input
            :value="wowPath"
            @input="$emit('update:wowPath', ($event.target as HTMLInputElement).value)"
            type="text"
            placeholder="/Applications/World of Warcraft/_retail_"
            class="flex-1 px-4 py-3 bg-white/15 backdrop-blur-md border border-white/30 rounded-xl text-white placeholder-white/70 font-mono focus:outline-none focus:ring-2 focus:ring-ocean/50 transition-all"
          />
          <button @click="$emit('find:path')" class="px-5 py-3 text-white bg-ocean/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-ocean/40 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300" :disabled="isScanning">
            Auto-detect
          </button>
          <button @click="$emit('scan:characters')" class="px-5 py-3 text-white bg-ocean/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-ocean/40 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300" :disabled="isScanning">
            <span v-if="!isScanning">Scan Characters</span>
            <span v-else class="loading loading-spinner"></span>
            <span v-if="isScanning">Scanning...</span>
          </button>
        </div>
        <StatusMessages info-message="Set your WoW installation path and scan to discover characters. Characters will appear in the Character Library on the Active Characters tab.">
        </StatusMessages>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-2">
      <button @click="$emit('save:settings')" class="px-6 py-3 text-white bg-ocean/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-ocean/40 transition-all duration-300 font-semibold">Save Settings</button>
    </div>
  </div>
</template>
