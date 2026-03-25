<script setup lang="ts">
defineProps<{
  wowPath: string;
  isScanning: boolean;
  addonInstalled: boolean | null;
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
    <div class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
      <div class="p-5">
        <h2 class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest mb-4">WoW Installation</h2>
        <div class="flex gap-2 flex-wrap">
          <input
            :value="wowPath"
            @input="$emit('update:wowPath', ($event.target as HTMLInputElement).value)"
            type="text"
            placeholder="/Applications/World of Warcraft/_retail_"
            class="flex-1 min-w-0 px-3 py-2.5 bg-[#07101e] border border-[#1e3a5f] rounded-lg text-sm text-[#e2eeff] placeholder-[#3a5870] font-mono focus:outline-none focus:border-[#2e5a9a] transition-colors"
          />
          <button
            @click="$emit('find:path')"
            :disabled="isScanning"
            class="px-4 py-2.5 border border-[#1e3a5f] hover:border-[#2e5a9a] text-[#7aadcc] hover:text-[#b0cce0] text-sm rounded-lg transition-colors disabled:opacity-40 whitespace-nowrap"
          >Auto-detect</button>
          <button
            @click="$emit('scan:characters')"
            :disabled="isScanning"
            class="px-4 py-2.5 bg-[#1d4ed8] hover:bg-[#2563eb] text-white text-sm font-medium rounded-lg transition-colors disabled:opacity-40 flex items-center gap-2 whitespace-nowrap"
          >
            <svg v-if="isScanning" class="animate-spin w-3.5 h-3.5 shrink-0" viewBox="0 0 24 24" fill="none">
              <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4" stroke-dashoffset="10"/>
            </svg>
            {{ isScanning ? 'Scanning...' : 'Scan Characters' }}
          </button>
        </div>

        <!-- Addon status -->
        <div v-if="addonInstalled === false" class="mt-4 flex items-start gap-3 rounded-lg border border-amber-900/30 bg-amber-900/10 px-4 py-3">
          <span class="text-amber-500/80 mt-0.5 shrink-0 text-sm">⚠</span>
          <span class="text-sm text-amber-400/80">
            <strong class="text-amber-400">TalentLoadoutsEx not found.</strong>
            Install from
            <a href="https://www.curseforge.com/wow/addons/talent-loadouts-ex" target="_blank" class="underline hover:text-amber-300 transition-colors">CurseForge</a>
            or
            <a href="https://wago.io/addons/TalentLoadoutsEx" target="_blank" class="underline hover:text-amber-300 transition-colors">Wago</a>.
          </span>
        </div>
        <div v-else-if="addonInstalled === true" class="mt-3 flex items-center gap-2 text-xs text-emerald-500/70">
          <span>✓</span> TalentLoadoutsEx detected
        </div>

        <p class="mt-3 text-xs text-[#5580a0]">Set your WoW path and scan to discover characters. They'll appear in the Character Library.</p>
      </div>
    </div>

    <div>
      <button
        @click="$emit('save:settings')"
        class="px-5 py-2.5 border border-[#1e3a5f] hover:border-[#2e5a9a] text-[#7aadcc] hover:text-[#b0cce0] text-sm font-medium rounded-lg transition-colors"
      >Save Settings</button>
    </div>
  </div>
</template>
