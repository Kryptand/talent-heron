<script setup lang="ts">
interface SelectedCharacter {
  name: string;
  class: string;
  specializations: string[];
}

defineProps<{
  characters: SelectedCharacter[];
  allClasses: string[];
  classSpecs: Record<string, string[]>;
}>();

const emit = defineEmits<{
  (e: 'remove', index: number): void;
  (e: 'update:class', index: number, className: string): void;
  (e: 'toggle:spec', index: number, spec: string): void;
}>();

function updateClass(index: number, event: Event) {
  emit('update:class', index, (event.target as HTMLSelectElement).value);
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
</script>

<template>
  <div v-if="characters.length > 0" class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
    <div class="p-5">
      <h2 class="text-xs font-semibold text-[#7aadcc] uppercase tracking-widest mb-4">Active Characters</h2>
      <div class="space-y-3">
        <div
          v-for="(char, index) in characters"
          :key="index"
          class="rounded-xl border border-[#1e3a5f] overflow-hidden"
          :style="`border-left-color: ${classColor(char.class)}55`"
          style="border-left-width: 2px;"
        >
          <div class="bg-[#172e4a] p-4">
            <div class="flex items-center gap-3 mb-3">
              <span class="font-semibold text-base" :style="`color: ${classColor(char.class)}`">{{ char.name }}</span>
              <select
                :value="char.class"
                @change="updateClass(index, $event)"
                class="px-2.5 py-1.5 bg-[#0e1d33] border border-[#1e3a5f] rounded-lg text-sm text-[#b0cce0] focus:outline-none focus:border-[#2e5a9a] transition-colors cursor-pointer appearance-none"
                style="background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2210%22 height=%226%22 viewBox=%220 0 10 6%22%3E%3Cpath fill=%22%236b8fa8%22 d=%22M5 6L0 0h10z%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 0.6rem center; background-size: 0.55rem; padding-right: 1.8rem;"
              >
                <option v-for="className in allClasses" :key="className" :value="className" class="bg-[#0e1d33] text-[#e2eeff]">
                  {{ className }}
                </option>
              </select>
              <button
                @click="$emit('remove', index)"
                class="ml-auto text-xs text-[#7aadcc] hover:text-[#f87171] transition-colors px-2 py-1 rounded"
              >Remove</button>
            </div>
            <div class="flex flex-wrap gap-2">
              <button
                v-for="spec in classSpecs[char.class] || []"
                :key="spec"
                @click="$emit('toggle:spec', index, spec)"
                class="px-3 py-1.5 rounded-lg border text-sm font-medium transition-all capitalize"
                :style="char.specializations.includes(spec)
                  ? `border-color: ${classColor(char.class)}55; color: ${classColor(char.class)}; background: ${classColor(char.class)}12`
                  : ''"
                :class="char.specializations.includes(spec) ? '' : 'border-[#1e3a5f] text-[#7aadcc] hover:text-[#b0cce0] hover:border-[#2e5a9a]'"
              >{{ spec }}</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Empty state -->
  <div v-else class="bg-[#0e1d33] rounded-2xl border border-[#1e3a5f]">
    <div class="text-center py-14 px-6">
      <div class="text-4xl mb-3 opacity-20 select-none">⚔</div>
      <p class="text-sm text-[#7aadcc]">No characters added yet.</p>
      <p class="text-xs text-[#5580a0] mt-1">Add characters from the library below.</p>
    </div>
  </div>
</template>
