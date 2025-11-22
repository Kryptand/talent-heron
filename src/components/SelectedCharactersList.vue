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
  const target = event.target as HTMLSelectElement;
  emit('update:class', index, target.value);
}
</script>

<template>
  <div v-if="characters.length > 0" class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
    <div class="p-6">
      <h2 class="text-2xl font-bold text-white mb-4 drop-shadow-lg">Your Active Characters</h2>
      <div class="space-y-3">
        <div v-for="(char, index) in characters" :key="index" class="bg-white/15 backdrop-blur-md rounded-xl border border-white/20 shadow-lg">
          <div class="p-4">
            <div class="flex items-center gap-3 mb-3">
              <span class="font-bold text-lg text-white drop-shadow">{{ char.name }}</span>
              <select :value="char.class" @change="updateClass(index, $event)" class="px-3 py-2 pr-8 bg-white/25 backdrop-blur-xl border border-white/40 rounded-lg text-white font-semibold focus:outline-none focus:ring-2 focus:ring-ocean/50 transition-all shadow-lg appearance-none cursor-pointer" style="-webkit-appearance: none; -moz-appearance: none; background-image: url('data:image/svg+xml;charset=UTF-8,%3Csvg xmlns=%22http://www.w3.org/2000/svg%22 width=%2212%22 height=%228%22 viewBox=%220 0 12 8%22%3E%3Cpath fill=%22%23ffffff%22 d=%22M6 8L0 0h12z%22/%3E%3C/svg%3E'); background-repeat: no-repeat; background-position: right 0.75rem center; background-size: 0.65rem;">
                <option v-for="className in allClasses" :key="className" :value="className" class="bg-ocean-dark text-white">
                  {{ className }}
                </option>
              </select>
              <button @click="$emit('remove', index)" class="px-4 py-2 text-sm text-white bg-ocean-pink/30 backdrop-blur-lg border border-white/20 rounded-lg shadow-lg hover:shadow-xl hover:bg-ocean-pink/40 ml-auto transition-all duration-300">Remove</button>
            </div>
            <div class="flex flex-wrap gap-2">
              <label
                v-for="spec in classSpecs[char.class] || []"
                :key="spec"
                class="flex items-center gap-2 cursor-pointer bg-white/15 backdrop-blur-sm px-3 py-2 rounded-lg border border-white/20 hover:bg-white/25 transition-all"
              >
                <input
                  type="checkbox"
                  :checked="char.specializations.includes(spec)"
                  @change="$emit('toggle:spec', index, spec)"
                  class="checkbox checkbox-sm border-white/50"
                />
                <span class="text-white text-sm font-semibold">{{ spec }}</span>
              </label>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- Empty state -->
  <div v-else class="bg-white/10 backdrop-blur-lg rounded-2xl border border-white/20 shadow-xl">
    <div class="text-center py-12 px-6">
      <h3 class="text-2xl font-bold text-white mb-2 drop-shadow-lg">No active characters yet</h3>
      <p class="text-white text-lg font-medium">Add characters from the Character Library below to get started</p>
    </div>
  </div>
</template>
