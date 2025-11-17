<script setup lang="ts">
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
            class="flex-1 px-4 py-3 bg-white/15 backdrop-blur-md border border-white/30 rounded-xl text-white placeholder-white/50 font-mono focus:outline-none focus:ring-2 focus:ring-white/50 transition-all"
          />
          <button @click="$emit('find:path')" class="px-5 py-3 text-white bg-purple-500/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-purple-500/40 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300" :disabled="isScanning">
            Auto-detect
          </button>
          <button @click="$emit('scan:characters')" class="px-5 py-3 text-white bg-purple-500/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-purple-500/40 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300" :disabled="isScanning">
            <span v-if="!isScanning">Scan Characters</span>
            <span v-else class="loading loading-spinner"></span>
            <span v-if="isScanning">Scanning...</span>
          </button>
        </div>
        <div class="flex items-start gap-3 bg-blue-500/20 backdrop-blur-lg border border-blue-300/30 rounded-xl p-4 shadow-lg mt-4">
          <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" class="stroke-white shrink-0 w-6 h-6 mt-0.5"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
          <span class="text-white/90 text-sm leading-relaxed">Set your WoW installation path and scan to discover characters. Characters will appear in the Character Library on the Active Characters tab.</span>
        </div>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-2">
      <button @click="$emit('save:settings')" class="px-6 py-3 text-white bg-purple-500/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-purple-500/40 transition-all duration-300 font-semibold">Save Settings</button>
    </div>
  </div>
</template>
