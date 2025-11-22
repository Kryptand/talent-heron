<script setup lang="ts">
import SelectedCharactersList from './SelectedCharactersList.vue';
import CharacterLibrary from './CharacterLibrary.vue';

interface DiscoveredCharacter {
  name: string;
  realm: string;
  class: string;
  accountId: string;
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

    <!-- Update Action -->
    <div class="mt-6">
      <button
        @click="$emit('update:talents')"
        :disabled="!hasValidSettings || isUpdating"
        class="w-full px-6 py-4 text-lg font-semibold text-white bg-ocean/30 backdrop-blur-lg border border-white/20 rounded-xl shadow-lg hover:shadow-xl hover:bg-ocean/40 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300"
      >
        <span v-if="!isUpdating">Update Talents for All Characters</span>
        <span v-else class="loading loading-spinner"></span>
        <span v-if="isUpdating">Updating...</span>
      </button>
    </div>
  </div>
</template>
