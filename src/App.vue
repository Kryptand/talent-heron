<script setup lang="ts">
import { ref, onMounted, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { load } from "@tauri-apps/plugin-store";
import TabNavigation from "./components/TabNavigation.vue";
import ActiveCharactersTab from "./components/ActiveCharactersTab.vue";
import GameSettingsTab from "./components/GameSettingsTab.vue";
import ContentSettingsTab from "./components/ContentSettingsTab.vue";
import StatusMessages from "./components/StatusMessages.vue";

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
  discoveredCharacters: DiscoveredCharacter[];
  characters: SelectedCharacter[];
  raidDifficulties: string[];
  raidBosses: string[];
  dungeons: string[];
  clearPreviousBuilds: boolean;
}

interface UpdateSummary {
  total_talents_updated: number;
  raid_talents: number;
  mythic_plus_talents: number;
  characters_processed: number;
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
const isDiscovering = ref(false);
const statusMessage = ref("");
const errorMessage = ref("");
const updateSummary = ref<UpdateSummary | null>(null);

const activeTab = ref("characters");
const expandedSections = ref({
  characters: selectedCharacters.value.length > 0,
});
const appInitialized = ref(false);

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

const hasValidSettings = computed<boolean>(() => {
  return !!(wowPath.value && selectedCharacters.value.length > 0);
});

function toggleLibrary() {
  expandedSections.value.characters = !expandedSections.value.characters;
}

onMounted(async () => {
  await loadSettings();
  if (wowPath.value) {
    await scanForCharacters();
  } else {
    await findWowPath();
  }

  appInitialized.value = true;
});

async function loadSettings() {
  try {
    const store = await load("settings.json", { autoSave: false, defaults: {} });
    const settings = await store.get<Settings>("settings");

    if (settings) {
      wowPath.value = settings.wowPath || "";
      discoveredCharacters.value = settings.discoveredCharacters || [];
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

async function saveSettings(showMessage: boolean = true) {
  try {
    const store = await load("settings.json", { autoSave: false, defaults: {} });
    const settings: Settings = {
      wowPath: wowPath.value,
      discoveredCharacters: discoveredCharacters.value,
      characters: selectedCharacters.value,
      raidDifficulties: raidDifficulties.value,
      raidBosses: raidBosses.value,
      dungeons: dungeons.value,
      clearPreviousBuilds: clearPreviousBuilds.value,
    };

    await store.set("settings", settings);
    await store.save();
    if (showMessage) {
      statusMessage.value = "Settings saved";
      errorMessage.value = "";
    }
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

    // Merge with existing characters (avoid duplicates)
    const existingKeys = new Set(
      discoveredCharacters.value.map(c => `${c.name}-${c.realm}-${c.accountId}`)
    );

    const newChars = chars.filter(
      c => !existingKeys.has(`${c.name}-${c.realm}-${c.accountId}`)
    );

    discoveredCharacters.value = [...discoveredCharacters.value, ...newChars];

    if(appInitialized.value){
      statusMessage.value = `Found ${chars.length} character(s) (${newChars.length} new)`;
    }

    await saveSettings(false);
  } catch (error) {
    errorMessage.value = `Failed to scan characters: ${error}`;
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

  // Collapse the character library after adding a character
  expandedSections.value.characters = false;
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

async function discoverContent() {
  try {
    isDiscovering.value = true;
    errorMessage.value = "";
    statusMessage.value = "Discovering raids and dungeons from Warcraft Logs...";

    const content = await invoke<{ raid_bosses: string[], dungeons: string[] }>("discover_content");

    raidBosses.value = content.raid_bosses;
    dungeons.value = content.dungeons;

    statusMessage.value = `Discovered ${content.raid_bosses.length} raid bosses and ${content.dungeons.length} dungeons`;
    await saveSettings(false);
  } catch (error) {
    errorMessage.value = `Failed to discover content: ${error}`;
    statusMessage.value = "";
  } finally {
    isDiscovering.value = false;
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
    updateSummary.value = null;
    statusMessage.value = "Fetching talents from Archon.gg...";

    const result = await invoke<UpdateSummary>("update_talents_from_config", {
      config,
    });

    updateSummary.value = result;
    statusMessage.value = `Successfully updated ${result.total_talents_updated} talents (${result.raid_talents} raid, ${result.mythic_plus_talents} M+) for ${result.characters_processed} character(s)`;
    await saveSettings(false);
  } catch (error) {
    errorMessage.value = `Update failed: ${error}`;
    statusMessage.value = "";
    updateSummary.value = null;
  } finally {
    isUpdating.value = false;
  }
}
</script>

<template>
  <div class="min-h-screen p-4 md:p-8">
    <div class="container mx-auto max-w-6xl">
      <div class="bg-white/20 backdrop-blur-2xl rounded-3xl shadow-2xl border border-white/30">
        <div class="p-6 md:p-8">
          <!-- Header -->
          <div class="text-center mb-12">
            <img src="/ddd.png" alt="Talent Heron" class="mx-auto mb-4 h-24 w-auto" />
            <h1 class="text-5xl font-bold mb-3 bg-gradient-to-r from-white to-white/80 bg-clip-text text-transparent drop-shadow-lg">Talent Heron</h1>
            <p class="select-none text-white text-lg drop-shadow-lg">Auto-discover characters and update WoW talents from Archon.gg</p>
          </div>

          <StatusMessages :status-message="statusMessage" :error-message="errorMessage" />

          <!-- Tab Navigation -->
          <TabNavigation :active-tab="activeTab" @update:active-tab="activeTab = $event" />

          <!-- Active Characters Tab -->
          <ActiveCharactersTab
            v-if="activeTab === 'characters'"
            :selected-characters="selectedCharacters"
            :discovered-characters="discoveredCharacters"
            :all-classes="allClasses"
            :class-specs="classSpecs"
            :library-expanded="expandedSections.characters"
            :is-updating="isUpdating"
            :has-valid-settings="hasValidSettings"
            @remove:character="removeCharacter"
            @update:class="(index, className) => selectedCharacters[index].class = className"
            @toggle:spec="toggleSpec"
            @toggle:library="toggleLibrary"
            @add:character="addCharacter"
            @update:talents="updateTalents"
          />

          <!-- Game Settings Tab -->
          <GameSettingsTab
            v-if="activeTab === 'game'"
            :wow-path="wowPath"
            :is-scanning="isScanning"
            @update:wow-path="wowPath = $event"
            @find:path="findWowPath"
            @scan:characters="scanForCharacters"
            @save:settings="saveSettings"
          />

          <!-- Content Settings Tab -->
          <ContentSettingsTab
            v-if="activeTab === 'content'"
            :raid-difficulties="raidDifficulties"
            :raid-bosses="raidBosses"
            :dungeons="dungeons"
            :clear-previous-builds="clearPreviousBuilds"
            :is-discovering="isDiscovering"
            @toggle:difficulty="toggleDifficulty"
            @update:raid-bosses="raidBosses = $event"
            @update:dungeons="dungeons = $event"
            @update:clear-previous-builds="clearPreviousBuilds = $event"
            @discover:content="discoverContent"
            @save:settings="saveSettings"
          />
        </div>
      </div>
    </div>
  </div>
</template>

