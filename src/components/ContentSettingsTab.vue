<script setup lang="ts">
defineProps<{
  raidDifficulties: string[];
  raidBosses: string[];
  dungeons: string[];
  clearPreviousBuilds: boolean;
  isDiscovering: boolean;
}>();

defineEmits<{
  (e: 'toggle:difficulty', difficulty: string): void;
  (e: 'update:raidBosses', value: string[]): void;
  (e: 'update:dungeons', value: string[]): void;
  (e: 'update:clearPreviousBuilds', value: boolean): void;
  (e: 'discover:content'): void;
  (e: 'save:settings'): void;
}>();
</script>

<template>
  <div class="space-y-4">
    <!-- Auto-Discovery -->
    <div class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
      <div class="p-5">
        <h2 class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest mb-1">Auto-Discovery</h2>
        <p class="text-sm text-[#7aadcc] mb-4">Fetch current season raids and M+ dungeons from Archon.gg.</p>
        <button
          @click="$emit('discover:content')"
          :disabled="isDiscovering"
          class="px-5 py-2.5 bg-[#1d4ed8] hover:bg-[#2563eb] disabled:opacity-40 disabled:cursor-not-allowed text-white text-sm font-medium rounded-lg transition-colors flex items-center gap-2"
        >
          <svg v-if="isDiscovering" class="animate-spin w-4 h-4 shrink-0" viewBox="0 0 24 24" fill="none">
            <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4" stroke-dashoffset="10"/>
          </svg>
          {{ isDiscovering ? 'Discovering...' : 'Auto-Discover Current Season' }}
        </button>
      </div>
    </div>

    <!-- Raid Settings -->
    <div class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
      <div class="p-5">
        <h2 class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest mb-4">Raid</h2>

        <div class="mb-5">
          <label class="text-xs text-[#5580a0] uppercase tracking-wide mb-2 block">Difficulty</label>
          <div class="flex gap-2">
            <button
              v-for="diff in ['normal', 'heroic', 'mythic']"
              :key="diff"
              @click="$emit('toggle:difficulty', diff)"
              class="px-4 py-1.5 rounded-lg border text-sm font-medium transition-all capitalize"
              :class="raidDifficulties.includes(diff)
                ? 'bg-[#1d4ed8]/20 border-[#3b82f6]/40 text-[#93c5fd]'
                : 'border-[#1e3a5f] text-[#7aadcc] hover:text-[#b0cce0] hover:border-[#2e5a9a]'"
            >{{ diff }}</button>
          </div>
        </div>

        <div>
          <div class="flex items-center justify-between mb-2">
            <label class="text-xs text-[#5580a0] uppercase tracking-wide">Bosses</label>
            <span class="text-xs text-[#7aadcc]">{{ raidBosses.length }} configured</span>
          </div>
          <textarea
            :value="raidBosses.join(', ')"
            @input="$emit('update:raidBosses', ($event.target as HTMLTextAreaElement).value.split(',').map(s => s.trim()).filter(s => s))"
            placeholder="broodtwister, sikran, queen-ansurek"
            rows="2"
            class="w-full px-3 py-2.5 bg-[#07101e] border border-[#1e3a5f] rounded-lg text-sm text-[#b0cce0] placeholder-[#1e3a5f] focus:outline-none focus:border-[#2e5a9a] transition-colors resize-none font-mono"
          />
          <div v-if="raidBosses.length > 0" class="flex flex-wrap gap-1.5 mt-2">
            <span
              v-for="boss in raidBosses"
              :key="boss"
              class="text-xs px-2 py-0.5 rounded-md bg-[#172e4a] border border-[#1e3a5f] text-[#7aadcc] font-mono"
            >{{ boss }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- M+ Settings -->
    <div class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
      <div class="p-5">
        <h2 class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest mb-4">Mythic+</h2>
        <div>
          <div class="flex items-center justify-between mb-2">
            <label class="text-xs text-[#5580a0] uppercase tracking-wide">Dungeons</label>
            <span class="text-xs text-[#7aadcc]">{{ dungeons.length }} configured</span>
          </div>
          <textarea
            :value="dungeons.join(', ')"
            @input="$emit('update:dungeons', ($event.target as HTMLTextAreaElement).value.split(',').map(s => s.trim()).filter(s => s))"
            placeholder="ara-kara, city-of-threads, mists-of-tirna-scithe"
            rows="2"
            class="w-full px-3 py-2.5 bg-[#07101e] border border-[#1e3a5f] rounded-lg text-sm text-[#b0cce0] placeholder-[#1e3a5f] focus:outline-none focus:border-[#2e5a9a] transition-colors resize-none font-mono"
          />
          <div v-if="dungeons.length > 0" class="flex flex-wrap gap-1.5 mt-2">
            <span
              v-for="dungeon in dungeons"
              :key="dungeon"
              class="text-xs px-2 py-0.5 rounded-md bg-[#172e4a] border border-[#1e3a5f] text-[#7aadcc] font-mono"
            >{{ dungeon }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Options -->
    <div class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
      <div class="p-5">
        <h2 class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest mb-3">Options</h2>
        <label class="flex items-center gap-3 cursor-pointer group" @click.prevent="$emit('update:clearPreviousBuilds', !clearPreviousBuilds)">
          <div
            class="w-9 h-5 rounded-full border transition-all relative flex-shrink-0"
            :class="clearPreviousBuilds ? 'bg-[#1d4ed8]/30 border-[#3b82f6]/40' : 'bg-[#07101e] border-[#1e3a5f]'"
          >
            <div
              class="absolute top-[2px] w-4 h-4 rounded-full transition-all"
              :class="clearPreviousBuilds ? 'left-[1.125rem] bg-[#93c5fd]' : 'left-[2px] bg-[#3a5870]'"
            />
          </div>
          <span class="text-sm text-[#7aadcc] group-hover:text-[#b0cce0] transition-colors">
            Clear previous auto-generated builds before updating
          </span>
        </label>
      </div>
    </div>

    <!-- Save -->
    <div>
      <button
        @click="$emit('save:settings')"
        class="px-5 py-2.5 border border-[#1e3a5f] hover:border-[#2e5a9a] text-[#7aadcc] hover:text-[#b0cce0] text-sm font-medium rounded-lg transition-colors"
      >Save Settings</button>
    </div>
  </div>
</template>
