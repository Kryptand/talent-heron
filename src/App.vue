<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { load, save } from "@tauri-apps/plugin-store";

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

interface Settings {
  wowPath: string;
  characters: SelectedCharacter[];
  raidDifficulties: string[];
  raidBosses: string[];
  dungeons: string[];
  clearPreviousBuilds: boolean;
}

const wowPath = ref("");
const discoveredCharacters = ref<DiscoveredCharacter[]>([]);
const selectedCharacters = ref<SelectedCharacter[]>([]);
const raidDifficulties = ref<string[]>(["heroic"]);
const raidBosses = ref<string[]>([]);
const dungeons = ref<string[]>([]);
const clearPreviousBuilds = ref(false);

const isScanning = ref(false);
const isUpdating = ref(false);
const statusMessage = ref("");
const errorMessage = ref("");

const allClasses = [
  "Warrior", "Paladin", "Hunter", "Rogue", "Priest",
  "DeathKnight", "Shaman", "Mage", "Warlock", "Monk",
  "Druid", "DemonHunter", "Evoker"
];

const classSpecs: Record<string, string[]> = {
  Warrior: ["arms", "fury", "protection"],
  Paladin: ["holy", "protection", "retribution"],
  Hunter: ["beast-mastery", "marksmanship", "survival"],
  Rogue: ["assassination", "outlaw", "subtlety"],
  Priest: ["discipline", "holy", "shadow"],
  DeathKnight: ["blood", "frost", "unholy"],
  Shaman: ["elemental", "enhancement", "restoration"],
  Mage: ["arcane", "fire", "frost"],
  Warlock: ["affliction", "demonology", "destruction"],
  Monk: ["brewmaster", "mistweaver", "windwalker"],
  Druid: ["balance", "feral", "guardian", "restoration"],
  DemonHunter: ["havoc", "vengeance"],
  Evoker: ["devastation", "preservation", "augmentation"]
};

const hasValidSettings = computed(() => {
  return wowPath.value && selectedCharacters.value.length > 0;
});

onMounted(async () => {
  await loadSettings();
  if (wowPath.value) {
    await scanForCharacters();
  } else {
    await findWowPath();
  }
});

async function loadSettings() {
  try {
    const store = await load("settings.json", { autoSave: false });
    const settings = await store.get<Settings>("settings");

    if (settings) {
      wowPath.value = settings.wowPath || "";
      selectedCharacters.value = settings.characters || [];
      raidDifficulties.value = settings.raidDifficulties || ["heroic"];
      raidBosses.value = settings.raidBosses || [];
      dungeons.value = settings.dungeons || [];
      clearPreviousBuilds.value = settings.clearPreviousBuilds || false;
    }
  } catch (error) {
    console.log("No saved settings found, using defaults");
  }
}

async function saveSettings() {
  try {
    const store = await load("settings.json", { autoSave: false });
    const settings: Settings = {
      wowPath: wowPath.value,
      characters: selectedCharacters.value,
      raidDifficulties: raidDifficulties.value,
      raidBosses: raidBosses.value,
      dungeons: dungeons.value,
      clearPreviousBuilds: clearPreviousBuilds.value,
    };

    await store.set("settings", settings);
    await store.save();
    statusMessage.value = "Settings saved";
    errorMessage.value = "";
  } catch (error) {
    errorMessage.value = `Failed to save settings: ${error}`;
  }
}

async function findWowPath() {
  try {
    const path = await invoke<string>("find_wow_path");
    wowPath.value = path;
    await scanForCharacters();
  } catch (error) {
    errorMessage.value = `Could not find WoW installation automatically. Please set the path manually.`;
  }
}

async function scanForCharacters() {
  if (!wowPath.value) {
    errorMessage.value = "Please set WoW installation path first";
    return;
  }

  try {
    isScanning.value = true;
    errorMessage.value = "";
    const chars = await invoke<DiscoveredCharacter[]>("scan_characters", {
      wowPath: wowPath.value,
    });
    discoveredCharacters.value = chars;
    statusMessage.value = `Found ${chars.length} character(s)`;
  } catch (error) {
    errorMessage.value = `Failed to scan characters: ${error}`;
    discoveredCharacters.value = [];
  } finally {
    isScanning.value = false;
  }
}

function addCharacter(char: DiscoveredCharacter) {
  const existingIndex = selectedCharacters.value.findIndex(
    (c) => c.name === char.name
  );

  if (existingIndex >= 0) {
    statusMessage.value = `${char.name} is already added`;
    return;
  }

  const classSpecs = getClassSpecs(char.class);

  selectedCharacters.value.push({
    name: char.name,
    class: char.class === "Unknown" ? "Warrior" : char.class,
    specializations: classSpecs.length > 0 ? [classSpecs[0]] : [],
  });
}

function removeCharacter(index: number) {
  selectedCharacters.value.splice(index, 1);
}

function getClassSpecs(className: string): string[] {
  return classSpecs[className] || [];
}

function toggleSpec(charIndex: number, spec: string) {
  const char = selectedCharacters.value[charIndex];
  const specIndex = char.specializations.indexOf(spec);

  if (specIndex >= 0) {
    char.specializations.splice(specIndex, 1);
  } else {
    char.specializations.push(spec);
  }
}

function toggleDifficulty(difficulty: string) {
  const index = raidDifficulties.value.indexOf(difficulty);
  if (index >= 0) {
    raidDifficulties.value.splice(index, 1);
  } else {
    raidDifficulties.value.push(difficulty);
  }
}

async function updateTalents() {
  if (!hasValidSettings.value) {
    errorMessage.value = "Please configure settings before updating";
    return;
  }

  // Build output path
  const firstChar = selectedCharacters.value[0];
  const accountId = discoveredCharacters.value.find(
    (c) => c.name === firstChar.name
  )?.accountId || "Unknown";

  const outputPath = `${wowPath.value}/WTF/Account/${accountId}/SavedVariables/TalentLoadoutsEx.lua`;

  const config = {
    characters: selectedCharacters.value,
    raidDifficulties: raidDifficulties.value,
    raidBosses: raidBosses.value,
    dungeons: dungeons.value,
    clearPreviousBuilds: clearPreviousBuilds.value,
    outputPath,
  };

  try {
    isUpdating.value = true;
    errorMessage.value = "";
    statusMessage.value = "Fetching talents from Archon.gg...";

    const result = await invoke<string>("update_talents_from_config", {
      config,
    });

    statusMessage.value = result;
    await saveSettings();
  } catch (error) {
    errorMessage.value = `Update failed: ${error}`;
    statusMessage.value = "";
  } finally {
    isUpdating.value = false;
  }
}
</script>

<template>
  <main class="container">
    <header>
      <h1>🎮 Archon Talent Updater</h1>
      <p class="subtitle">Auto-discover characters and update WoW talents from Archon.gg</p>
    </header>

    <!-- WoW Installation -->
    <section class="settings-section">
      <h2>📂 WoW Installation</h2>
      <div class="input-group">
        <input
          v-model="wowPath"
          type="text"
          placeholder="/Applications/World of Warcraft/_retail_"
          class="path-input"
        />
        <button @click="findWowPath" class="btn-secondary" :disabled="isScanning">
          🔍 Auto-detect
        </button>
        <button @click="scanForCharacters" class="btn-secondary" :disabled="isScanning">
          <span v-if="!isScanning">🔄 Scan</span>
          <span v-else>⏳ Scanning...</span>
        </button>
      </div>
    </section>

    <!-- Discovered Characters -->
    <section class="settings-section" v-if="discoveredCharacters.length > 0">
      <h2>👥 Discovered Characters</h2>
      <div class="character-grid">
        <div
          v-for="char in discoveredCharacters"
          :key="`${char.name}-${char.realm}`"
          class="character-card"
          @click="addCharacter(char)"
        >
          <div class="char-name">{{ char.name }}</div>
          <div class="char-details">{{ char.realm }} • {{ char.class }}</div>
        </div>
      </div>
    </section>

    <!-- Selected Characters -->
    <section class="settings-section" v-if="selectedCharacters.length > 0">
      <h2>✅ Selected Characters & Specs</h2>
      <div class="selected-characters">
        <div v-for="(char, index) in selectedCharacters" :key="index" class="selected-char">
          <div class="char-header">
            <span class="char-name-large">{{ char.name }}</span>
            <select v-model="char.class" class="class-select">
              <option v-for="className in allClasses" :key="className" :value="className">
                {{ className }}
              </option>
            </select>
            <button @click="removeCharacter(index)" class="btn-remove">✕</button>
          </div>
          <div class="spec-selection">
            <label
              v-for="spec in getClassSpecs(char.class)"
              :key="spec"
              class="spec-checkbox"
            >
              <input
                type="checkbox"
                :checked="char.specializations.includes(spec)"
                @change="toggleSpec(index, spec)"
              />
              {{ spec }}
            </label>
          </div>
        </div>
      </div>
    </section>

    <!-- Raid Settings -->
    <section class="settings-section">
      <h2>🏆 Raid Settings</h2>
      <div class="form-group">
        <label>Difficulties:</label>
        <div class="checkbox-group">
          <label v-for="diff in ['normal', 'heroic', 'mythic']" :key="diff">
            <input
              type="checkbox"
              :checked="raidDifficulties.includes(diff)"
              @change="toggleDifficulty(diff)"
            />
            {{ diff }}
          </label>
        </div>
      </div>
      <div class="form-group">
        <label>Bosses (comma-separated, lowercase-hyphenated):</label>
        <input
          v-model="raidBosses"
          type="text"
          placeholder="broodtwister, sikran, queen-ansurek"
          @input="raidBosses = ($event.target as HTMLInputElement).value.split(',').map(s => s.trim())"
          :value="raidBosses.join(', ')"
        />
      </div>
    </section>

    <!-- M+ Settings -->
    <section class="settings-section">
      <h2>⚔️ Mythic+ Settings</h2>
      <div class="form-group">
        <label>Dungeons (comma-separated, lowercase-hyphenated):</label>
        <input
          v-model="dungeons"
          type="text"
          placeholder="ara-kara, city-of-threads, mists-of-tirna-scithe"
          @input="dungeons = ($event.target as HTMLInputElement).value.split(',').map(s => s.trim())"
          :value="dungeons.join(', ')"
        />
      </div>
    </section>

    <!-- Other Settings -->
    <section class="settings-section">
      <h2>⚙️ Other Settings</h2>
      <label class="checkbox-label">
        <input type="checkbox" v-model="clearPreviousBuilds" />
        Clear all previous auto-generated builds before updating
      </label>
    </section>

    <!-- Actions -->
    <section class="action-section">
      <button @click="saveSettings" class="btn-secondary">💾 Save Settings</button>
      <button
        @click="updateTalents"
        :disabled="!hasValidSettings || isUpdating"
        class="btn-update"
      >
        <span v-if="!isUpdating">🚀 Update Talents</span>
        <span v-else>⏳ Updating...</span>
      </button>
    </section>

    <!-- Status -->
    <section v-if="statusMessage || errorMessage" class="status-section">
      <div v-if="statusMessage" class="status-success">✓ {{ statusMessage }}</div>
      <div v-if="errorMessage" class="status-error">✗ {{ errorMessage }}</div>
    </section>
  </main>
</template>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 2rem;
}

header {
  text-align: center;
  margin-bottom: 2rem;
}

h1 {
  font-size: 2.5rem;
  margin-bottom: 0.5rem;
  color: #24c8db;
}

.subtitle {
  color: #666;
  font-size: 1.1rem;
}

.settings-section {
  background: #f9f9f9;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1.5rem;
  margin-bottom: 1.5rem;
}

h2 {
  font-size: 1.3rem;
  margin-bottom: 1rem;
  color: #333;
}

.input-group {
  display: flex;
  gap: 0.5rem;
}

.path-input {
  flex: 1;
  padding: 0.6rem 1rem;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-family: monospace;
}

.btn-secondary,
.btn-update {
  padding: 0.6rem 1.2rem;
  font-size: 0.95rem;
  font-weight: 600;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary {
  background-color: #24c8db;
  color: white;
}

.btn-secondary:hover:not(:disabled) {
  background-color: #1da1b5;
}

.btn-secondary:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.character-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 1rem;
}

.character-card {
  background: white;
  border: 2px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.character-card:hover {
  border-color: #24c8db;
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(36, 200, 219, 0.2);
}

.char-name {
  font-weight: 600;
  font-size: 1.1rem;
  color: #333;
}

.char-details {
  font-size: 0.9rem;
  color: #666;
  margin-top: 0.25rem;
}

.selected-characters {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.selected-char {
  background: white;
  border: 1px solid #ddd;
  border-radius: 8px;
  padding: 1rem;
}

.char-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  margin-bottom: 0.75rem;
}

.char-name-large {
  font-weight: 600;
  font-size: 1.1rem;
  color: #333;
}

.class-select {
  padding: 0.4rem 0.8rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 0.9rem;
}

.btn-remove {
  margin-left: auto;
  padding: 0.4rem 0.8rem;
  background: #f44336;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
}

.btn-remove:hover {
  background: #d32f2f;
}

.spec-selection {
  display: flex;
  flex-wrap: wrap;
  gap: 0.75rem;
}

.spec-checkbox {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  padding: 0.4rem 0.8rem;
  background: #f5f5f5;
  border-radius: 4px;
  cursor: pointer;
  font-size: 0.9rem;
}

.spec-checkbox input {
  cursor: pointer;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  font-weight: 600;
  margin-bottom: 0.5rem;
  color: #333;
}

.form-group input[type="text"] {
  width: 100%;
  padding: 0.6rem 1rem;
  border: 1px solid #ddd;
  border-radius: 6px;
  font-size: 0.95rem;
}

.checkbox-group {
  display: flex;
  gap: 1rem;
}

.checkbox-group label {
  display: flex;
  align-items: center;
  gap: 0.4rem;
  cursor: pointer;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-size: 0.95rem;
}

.action-section {
  display: flex;
  gap: 1rem;
  margin-bottom: 1.5rem;
}

.btn-update {
  flex: 1;
  background-color: #4caf50;
  color: white;
  font-size: 1.1rem;
}

.btn-update:hover:not(:disabled) {
  background-color: #45a049;
}

.btn-update:disabled {
  background-color: #ccc;
  cursor: not-allowed;
}

.status-section {
  padding: 1rem;
  border-radius: 8px;
}

.status-success {
  background: #e8f5e9;
  color: #2e7d32;
  padding: 1rem;
  border-radius: 8px;
  border-left: 4px solid #4caf50;
}

.status-error {
  background: #ffebee;
  color: #c62828;
  padding: 1rem;
  border-radius: 8px;
  border-left: 4px solid #f44336;
}

@media (prefers-color-scheme: dark) {
  .settings-section {
    background: #1a1a1a;
    border-color: #444;
  }

  h2 {
    color: #f6f6f6;
  }

  .path-input,
  .class-select,
  .form-group input[type="text"] {
    background: #2a2a2a;
    color: #f6f6f6;
    border-color: #555;
  }

  .character-card,
  .selected-char {
    background: #2a2a2a;
    border-color: #555;
  }

  .char-name,
  .char-name-large,
  .form-group label {
    color: #f6f6f6;
  }

  .char-details {
    color: #aaa;
  }

  .spec-checkbox {
    background: #333;
    color: #f6f6f6;
  }
}
</style>
