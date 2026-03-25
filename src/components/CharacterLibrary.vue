<script setup lang="ts">
import { ref, computed } from "vue";

interface DiscoveredCharacter {
  name: string;
  realm: string;
  class: string;
  accountId: string;
  lastPlayed: number;
}

const WEEK_SECS = 7 * 24 * 60 * 60;
const now = Math.floor(Date.now() / 1000);

function recentLabel(lastPlayed: number): string | null {
  if (!lastPlayed) return null;
  const age = now - lastPlayed;
  if (age < WEEK_SECS) return "this week";
  if (age < WEEK_SECS * 4) return "this month";
  return null;
}

const classColors: Record<string, string> = {
  Warrior:    '#C79C6E',
  Paladin:    '#F58CBA',
  Hunter:     '#ABD473',
  Rogue:      '#FFF569',
  Priest:     '#d4d4d4',
  DeathKnight:'#C41F3B',
  Shaman:     '#0070DE',
  Mage:       '#69CCF0',
  Warlock:    '#9482C9',
  Monk:       '#00FF96',
  Druid:      '#FF7D0A',
  DemonHunter:'#A330C9',
  Evoker:     '#33937F',
};

function classColor(className: string): string {
  return classColors[className] || '#7aadcc';
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
    if (!groups[char.accountId]) groups[char.accountId] = [];
    groups[char.accountId].push(char);
  });
  return groups;
});
</script>

<template>
  <div class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
    <div class="p-5">
      <button @click="$emit('toggle')" class="w-full flex items-center gap-2 text-left group select-none">
        <svg
          class="w-3 h-3 text-[#7aadcc] group-hover:text-[#2e5a9a] transition-all duration-200"
          :class="expanded ? 'rotate-90' : ''"
          viewBox="0 0 12 12" fill="currentColor"
        ><path d="M4 2l4 4-4 4" stroke="currentColor" stroke-width="1.5" fill="none" stroke-linecap="round" stroke-linejoin="round"/></svg>
        <span class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest group-hover:text-[#7aadcc] transition-colors">
          Character Library
        </span>
        <span class="text-xs text-[#5580a0]">({{ discoveredCharacters.length }})</span>
      </button>

      <div v-if="expanded" class="mt-4 space-y-4">
        <!-- Search -->
        <input
          v-model="characterSearch"
          type="text"
          placeholder="Search characters, classes, realms..."
          class="w-full px-3 py-2 bg-[#07101e] border border-[#1e3a5f] rounded-lg text-sm text-[#e2eeff] placeholder-[#3a5870] focus:outline-none focus:border-[#2e5a9a] transition-colors"
        />

        <!-- Grouped Characters -->
        <div v-if="Object.keys(groupedCharacters).length > 0" class="space-y-4">
          <div v-for="(chars, accountId) in groupedCharacters" :key="accountId">
            <div class="text-xs text-[#5580a0] mb-2 px-0.5 font-mono">{{ accountId }}</div>
            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-2">
              <button
                v-for="char in chars"
                :key="`${char.name}-${char.realm}-${char.accountId}`"
                @click="$emit('add', char)"
                class="text-left bg-[#172e4a] hover:bg-[#172e4a] border border-[#1e3a5f] hover:border-[#2e5a9a] rounded-lg p-3 transition-all"
                :style="`border-left: 2px solid ${classColor(char.class)}44`"
              >
                <div class="flex items-center gap-1.5">
                  <span class="font-semibold text-sm" :style="`color: ${classColor(char.class)}`">{{ char.name }}</span>
                  <span
                    v-if="recentLabel(char.lastPlayed)"
                    class="text-xs px-1.5 py-0.5 rounded-full border"
                    style="background: rgba(52,211,153,0.08); color: #34d399; border-color: rgba(52,211,153,0.2);"
                  >{{ recentLabel(char.lastPlayed) }}</span>
                </div>
                <div class="text-xs text-[#7aadcc] mt-0.5">{{ char.realm }} · {{ char.class }}</div>
              </button>
            </div>
          </div>
        </div>
        <div v-else class="text-center py-6 text-xs text-[#5580a0]">
          No characters found. Go to Game Settings to scan.
        </div>
      </div>
    </div>
  </div>
</template>
