<script setup lang="ts">
import SelectedCharactersList from './SelectedCharactersList.vue';
import CharacterLibrary from './CharacterLibrary.vue';

interface DiscoveredCharacter {
  name: string;
  realm: string;
  class: string;
  accountId: string;
  lastPlayed: number;
}

interface SelectedCharacter {
  name: string;
  class: string;
  specializations: string[];
}

defineProps<{
  selectedCharacters: SelectedCharacter[];
  discoveredCharacters: DiscoveredCharacter[];
  allClasses: string[];
  classSpecs: Record<string, string[]>;
  libraryExpanded: boolean;
  isUpdating: boolean;
  hasValidSettings: boolean;
}>();

defineEmits<{
  (e: 'remove:character', index: number): void;
  (e: 'update:class', index: number, className: string): void;
  (e: 'toggle:spec', index: number, spec: string): void;
  (e: 'toggle:library'): void;
  (e: 'add:character', char: DiscoveredCharacter): void;
  (e: 'update:talents'): void;
}>();
</script>

<template>
  <div class="space-y-4">
    <!-- Selected Characters -->
    <SelectedCharactersList
      :characters="selectedCharacters"
      :all-classes="allClasses"
      :class-specs="classSpecs"
      @remove="(index) => $emit('remove:character', index)"
      @update:class="(index, className) => $emit('update:class', index, className)"
      @toggle:spec="(index, spec) => $emit('toggle:spec', index, spec)"
    />

    <!-- Character Library -->
    <CharacterLibrary
      :discovered-characters="discoveredCharacters"
      :expanded="libraryExpanded"
      @toggle="$emit('toggle:library')"
      @add="$emit('add:character', $event)"
    />

    <!-- Update CTA -->
    <div class="mt-2">
      <button
        @click="$emit('update:talents')"
        :disabled="!hasValidSettings || isUpdating"
        class="w-full px-6 py-4 text-base font-semibold text-white rounded-xl transition-all duration-200 disabled:opacity-40 disabled:cursor-not-allowed flex items-center justify-center gap-3"
        :class="hasValidSettings && !isUpdating
          ? 'bg-gradient-to-r from-[#1d4ed8] to-[#3b82f6] hover:from-[#1e40af] hover:to-[#2563eb] shadow-lg'
          : 'bg-[#172e4a] border border-[#1e3a5f]'"
        :style="hasValidSettings && !isUpdating ? 'box-shadow: 0 4px 24px rgba(59,130,246,0.25)' : ''"
      >
        <svg v-if="isUpdating" class="animate-spin w-5 h-5 shrink-0" viewBox="0 0 24 24" fill="none">
          <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4" stroke-dashoffset="10"/>
        </svg>
        {{ isUpdating ? 'Updating...' : 'Update Talents for All Characters' }}
      </button>
    </div>
  </div>
</template>
