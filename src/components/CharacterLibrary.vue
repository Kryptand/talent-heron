<script setup lang="ts">
import { ref, computed } from "vue";

interface DiscoveredCharacter {
  name: string;
  realm: string;
  class: string;
  accountId: string;
}

const props = defineProps<{
  discoveredCharacters: DiscoveredCharacter[];
  expanded: boolean;
}>();

defineEmits<{
  (e: 'toggle'): void;
  (e: 'add', char: DiscoveredCharacter): void;
}>();

const characterSearch = ref("");

// Group characters by account ID
const groupedCharacters = computed(() => {
  const filtered = props.discoveredCharacters.filter((char) => {
    if (!characterSearch.value) return true;
    const search = characterSearch.value.toLowerCase();
    return (
      char.name.toLowerCase().includes(search) ||
      char.realm.toLowerCase().includes(search) ||
      char.class.toLowerCase().includes(search) ||
      char.accountId.toLowerCase().includes(search)
    );
  });

  const groups: Record<string, DiscoveredCharacter[]> = {};
  filtered.forEach((char) => {
    if (!groups[char.accountId]) {
      groups[char.accountId] = [];
    }
    groups[char.accountId].push(char);
  });

  return groups;
});
</script>

<template>
  <div class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
    <div class="p-6">
      <h2 @click="$emit('toggle')" class="text-2xl font-bold text-white cursor-pointer hover:text-white/80 select-none transition-colors flex items-center gap-2 drop-shadow-lg">
        <span class="text-lg">{{ expanded ? '▼' : '▶' }}</span>
        Character Library
        <span class="text-sm font-normal text-white">({{ discoveredCharacters.length }} discovered)</span>
      </h2>
      <div v-if="expanded" class="mt-4 space-y-4">
        <!-- Search -->
        <input
          v-model="characterSearch"
          type="text"
          placeholder="Search characters, realms, classes, or accounts..."
          class="w-full px-4 py-3 bg-white/15 backdrop-blur-md border border-white/30 rounded-xl text-white placeholder-white/70 focus:outline-none focus:ring-2 focus:ring-ocean/50 transition-all"
        />

        <!-- Grouped Characters -->
        <div v-if="Object.keys(groupedCharacters).length > 0" class="space-y-4">
          <div
            v-for="(chars, accountId) in groupedCharacters"
            :key="accountId"
            class="bg-white/10 backdrop-blur-md rounded-xl border border-white/20 shadow-lg"
          >
            <div class="p-4">
              <div class="flex justify-between items-center mb-3 pb-2 border-b border-white/30">
                <span class="font-bold text-white drop-shadow">Account: {{ accountId }}</span>
                <span class="text-sm text-white font-medium">{{ chars.length }} character(s)</span>
              </div>
              <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-3">
                <div
                  v-for="char in chars"
                  :key="`${char.name}-${char.realm}-${char.accountId}`"
                  class="bg-white/10 backdrop-blur-sm hover:bg-white/20 cursor-pointer transition-all hover:shadow-lg rounded-lg border border-white/20"
                  @click="$emit('add', char)"
                >
                  <div class="p-3">
                    <div class="font-bold text-lg text-white drop-shadow">{{ char.name }}</div>
                    <div class="text-sm text-white/90 font-medium">{{ char.realm }} • {{ char.class }}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div v-else class="text-center py-8 text-white italic font-medium">
          No characters found. Go to Game Settings to scan for characters.
        </div>
      </div>
    </div>
  </div>
</template>
