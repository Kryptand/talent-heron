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
    <!-- Auto-Discovery Section -->
    <div class="bg-gradient-to-br from-ocean/20 to-ocean-pink/20 backdrop-blur-lg rounded-2xl border border-white/30 shadow-xl">
      <div class="p-6">
        <h2 class="text-2xl font-bold text-white mb-3 drop-shadow-lg">Auto-Discovery</h2>
        <p class="text-white text-sm mb-4">
          Automatically fetch current raid bosses and mythic+ dungeons from Warcraft Logs.
          This will populate the lists below with the latest content.
        </p>
        <button @click="$emit('discover:content')" class="w-full px-6 py-4 text-white bg-ocean/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-ocean/40 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300 font-semibold" :disabled="isDiscovering">
          <span v-if="!isDiscovering">Auto-Discover Current Content</span>
          <span v-else class="loading loading-spinner"></span>
          <span v-if="isDiscovering">Discovering...</span>
        </button>
      </div>
    </div>

    <!-- Raid Settings -->
    <div class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
      <div class="p-6">
        <h2 class="text-2xl font-bold text-white mb-4 drop-shadow-lg">Raid Settings</h2>
        <div class="mb-4">
          <label class="text-white font-semibold mb-2 block">Difficulties:</label>
          <div class="flex gap-4">
            <label v-for="diff in ['normal', 'heroic', 'mythic']" :key="diff" class="flex items-center gap-2 cursor-pointer bg-white/10 backdrop-blur-sm px-4 py-2 rounded-lg border border-white/20 hover:bg-white/20 transition-all">
              <input
                type="checkbox"
                :checked="raidDifficulties.includes(diff)"
                @change="$emit('toggle:difficulty', diff)"
                class="checkbox checkbox-sm border-white/50"
              />
              <span class="text-white capitalize font-medium">{{ diff }}</span>
            </label>
          </div>
        </div>
        <div class="mt-4">
          <label class="text-white font-semibold mb-2 block">Bosses (comma-separated, lowercase-hyphenated):</label>
          <textarea
            :value="raidBosses.join(', ')"
            @input="$emit('update:raidBosses', ($event.target as HTMLTextAreaElement).value.split(',').map(s => s.trim()).filter(s => s))"
            placeholder="broodtwister, sikran, queen-ansurek"
            rows="3"
            class="w-full px-4 py-3 bg-white/15 backdrop-blur-md border border-white/30 rounded-xl text-white placeholder-white/70 focus:outline-none focus:ring-2 focus:ring-ocean/50 transition-all resize-none"
          />
          <span class="text-white text-sm mt-1 block font-medium">{{ raidBosses.length > 0 ? `${raidBosses.length} bosses configured` : 'No bosses configured' }}</span>
        </div>
      </div>
    </div>

    <!-- M+ Settings -->
    <div class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
      <div class="p-6">
        <h2 class="text-2xl font-bold text-white mb-4 drop-shadow-lg">Mythic+ Settings</h2>
        <div>
          <label class="text-white font-semibold mb-2 block">Dungeons (comma-separated, lowercase-hyphenated):</label>
          <textarea
            :value="dungeons.join(', ')"
            @input="$emit('update:dungeons', ($event.target as HTMLTextAreaElement).value.split(',').map(s => s.trim()).filter(s => s))"
            placeholder="ara-kara, city-of-threads, mists-of-tirna-scithe"
            rows="3"
            class="w-full px-4 py-3 bg-white/15 backdrop-blur-md border border-white/30 rounded-xl text-white placeholder-white/70 focus:outline-none focus:ring-2 focus:ring-ocean/50 transition-all resize-none"
          />
          <span class="text-white text-sm mt-1 block font-medium">{{ dungeons.length > 0 ? `${dungeons.length} dungeons configured` : 'No dungeons configured' }}</span>
        </div>
      </div>
    </div>

    <!-- Other Settings -->
    <div class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
      <div class="p-6">
        <h2 class="text-2xl font-bold text-white mb-4 drop-shadow-lg">Other Settings</h2>
        <label class="flex items-center gap-3 cursor-pointer bg-white/10 backdrop-blur-sm px-4 py-3 rounded-lg border border-white/20 hover:bg-white/15 transition-all">
          <input
            type="checkbox"
            :checked="clearPreviousBuilds"
            @change="$emit('update:clearPreviousBuilds', ($event.target as HTMLInputElement).checked)"
            class="checkbox checkbox-sm border-white/50"
          />
          <span class="text-white font-medium">Clear all previous auto-generated builds before updating</span>
        </label>
      </div>
    </div>

    <!-- Actions -->
    <div class="flex gap-2">
      <button @click="$emit('save:settings')" class="px-6 py-3 text-white bg-ocean/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-ocean/40 transition-all duration-300 font-semibold">Save Settings</button>
    </div>
  </div>
</template>
