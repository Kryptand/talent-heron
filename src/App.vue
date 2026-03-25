<script setup lang="ts">
import { ref, onMounted, computed, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openUrl as tauriOpenUrl } from "@tauri-apps/plugin-opener";
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
  lastPlayed: number;
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
const showSplash = ref(true);
const splashFadingOut = ref(false);

interface UpdateInfo {
  available: boolean;
  current_version: string;
  latest_version: string;
  release_url: string;
  release_notes: string;
  download_url: string | null;
}
const updateInfo = ref<UpdateInfo | null>(null);
const updateDismissed = ref(false);
const updateDownloadProgress = ref<number | null>(null);
const updateDownloadError = ref('');
const addonInstalled = ref<boolean | null>(null);

const isFirstRun = ref(false);
const setupStep = ref<'idle' | 'detecting' | 'scanning' | 'content' | 'done' | 'error'>('idle');
const setupStepsDone = ref<Record<string, boolean>>({});
const setupError = ref('');
const setupResults = ref({ chars: 0, bosses: 0, dungeons: 0 });

async function runQuickSetup() {
  setupStep.value = 'detecting';
  setupStepsDone.value = {};
  setupError.value = '';
  await nextTick();
  try {
    // Step 1: detect WoW path
    const path = await invoke<string>('find_wow_path');
    wowPath.value = path;
    setupStepsDone.value = { ...setupStepsDone.value, detecting: true };

    // Step 2: scan characters
    setupStep.value = 'scanning';
    await nextTick();
    const chars = await invoke<DiscoveredCharacter[]>('scan_characters', { wowPath: path });
    discoveredCharacters.value = chars;
    setupResults.value.chars = chars.length;
    setupStepsDone.value = { ...setupStepsDone.value, scanning: true };

    // Step 3: discover content
    setupStep.value = 'content';
    await nextTick();
    const content = await invoke<{ raid_bosses: string[], dungeons: string[] }>('discover_content');
    raidBosses.value = content.raid_bosses;
    dungeons.value = content.dungeons;
    raidDifficulties.value = ['normal', 'heroic'];
    setupResults.value.bosses = content.raid_bosses.length;
    setupResults.value.dungeons = content.dungeons.length;
    setupStepsDone.value = { ...setupStepsDone.value, content: true };

    await saveSettings(false);
    setupStep.value = 'done';
    setTimeout(() => { isFirstRun.value = false; }, 2000);
  } catch (e) {
    setupError.value = String(e);
    setupStep.value = 'error';
  }
}

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
  // Start splash fade out after 2 seconds
  setTimeout(() => {
    splashFadingOut.value = true;
    // Hide splash completely after animation finishes
    setTimeout(() => {
      showSplash.value = false;
    }, 1000);
  }, 2000);

  await loadSettings();

  if (!wowPath.value) {
    isFirstRun.value = true;
  } else {
    await scanForCharacters();
  }

  appInitialized.value = true;

  // Check for updates in the background
  try {
    const info = await invoke<UpdateInfo>("check_for_updates");
    updateInfo.value = info; // always set so we can show current version
  } catch (e) {
    console.warn("Update check failed:", e);
  }
});

async function loadSettings() {
  try {
    const store = await load("settings.json", { autoSave: false, defaults: {} });
    const settings = await store.get<Settings>("settings");

    if (settings) {
      wowPath.value = settings.wowPath || "";
      // discoveredCharacters are always rescanned fresh — never loaded from cache
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
      discoveredCharacters: [],
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

async function openUrl(url: string) {
  try { await tauriOpenUrl(url); } catch { /* ignore */ }
}

async function downloadUpdate(url: string) {
  updateDownloadProgress.value = 0;
  updateDownloadError.value = '';

  const unlisten = await listen<number>('update-progress', (event) => {
    updateDownloadProgress.value = event.payload;
  });

  try {
    await invoke('download_and_install_update', { url });
  } catch (e) {
    updateDownloadError.value = String(e);
    updateDownloadProgress.value = null;
    unlisten();
  }
}

async function checkAddon() {
  if (!wowPath.value) return;
  addonInstalled.value = await invoke<boolean>("check_addon_installed", { wowPath: wowPath.value });
}

async function scanForCharacters() {
  if (!wowPath.value) {
    errorMessage.value = "Please set WoW installation path first";
    return;
  }

  try {
    await checkAddon();
    isScanning.value = true;
    errorMessage.value = "";
    const chars = await invoke<DiscoveredCharacter[]>("scan_characters", {
      wowPath: wowPath.value,
    });

    // Always replace with fresh scan results — stale cache causes Unknown class issues
    discoveredCharacters.value = chars;
    if (appInitialized.value) {
      statusMessage.value = `Found ${chars.length} character(s)`;
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
  <div class="min-h-screen" style="background: #07101e;">
    <!-- Splash Screen -->
    <div
      v-if="showSplash"
      class="fixed inset-0 z-50 flex items-center justify-center transition-opacity duration-1000"
      :class="{ 'opacity-0': splashFadingOut, 'opacity-100': !splashFadingOut }"
    >
      <div class="absolute inset-0 midnight-bg">
        <div class="stars"></div>
        <div class="stars2"></div>
        <div class="moon-glow"></div>
      </div>
      <div class="relative z-10 text-center">
        <h1 class="text-7xl md:text-8xl font-bold tracking-widest midnight-title">
          TALENT HERON
        </h1>
        <div class="mt-3 h-px w-48 mx-auto bg-gradient-to-r from-transparent via-[#7dd3fc] to-transparent opacity-60"></div>
      </div>
    </div>

    <!-- First Run Setup -->
    <div
      v-if="isFirstRun && !showSplash"
      class="fixed inset-0 z-40 flex items-center justify-center bg-black/70 backdrop-blur-sm"
    >
      <div class="relative w-full max-w-sm mx-4 rounded-2xl border border-[#1e3a5f] bg-[#07101f] shadow-2xl overflow-hidden">
        <div class="h-px w-full bg-gradient-to-r from-transparent via-[#60a5fa] to-transparent"></div>
        <div class="p-8 text-center">
          <h2 class="text-xl font-semibold text-[#93c5fd] tracking-wide mb-1">Welcome to Talent Heron</h2>
          <p class="text-sm text-[#4a7fa8] mb-8">Set everything up in one click.</p>

          <!-- Steps -->
          <div class="space-y-4 mb-8 text-left">
            <div v-for="item in [
              { key: 'detecting', label: 'Detect WoW installation', result: setupStepsDone['detecting'] ? (wowPath || 'found') : null },
              { key: 'scanning',  label: 'Scan characters',         result: setupStepsDone['scanning']  ? `${setupResults.chars} found` : null },
              { key: 'content',   label: 'Fetch raids & dungeons',  result: setupStepsDone['content']   ? `${setupResults.bosses} bosses, ${setupResults.dungeons} dungeons` : null },
            ]" :key="item.key" class="flex items-start gap-3 text-sm">
              <span class="mt-0.5 w-5 h-5 flex items-center justify-center rounded-full text-xs shrink-0 transition-all"
                :class="setupStep === item.key
                  ? 'bg-[#1d4ed8] text-white'
                  : setupStepsDone[item.key]
                    ? 'bg-emerald-500/20 text-emerald-400'
                    : 'bg-[#1e3a5f]/40 text-[#3a5870]'">
                <svg v-if="setupStep === item.key" class="animate-spin w-3 h-3" viewBox="0 0 24 24" fill="none">
                  <circle cx="12" cy="12" r="10" stroke="currentColor" stroke-width="3" stroke-dasharray="31.4" stroke-dashoffset="10"/>
                </svg>
                <span v-else-if="setupStepsDone[item.key]">✓</span>
              </span>
              <div>
                <div :class="setupStep === item.key ? 'text-[#e2eeff]' : setupStepsDone[item.key] ? 'text-[#b0cce0]' : 'text-[#5580a0]'">
                  {{ item.label }}
                </div>
                <div v-if="item.result" class="text-xs text-emerald-400 mt-0.5">{{ item.result }}</div>
              </div>
            </div>
          </div>

          <p v-if="setupStep === 'error'" class="text-xs text-red-400 mb-4 text-left bg-red-900/20 rounded-lg p-3">{{ setupError }}</p>

          <button
            v-if="setupStep === 'idle' || setupStep === 'error'"
            @click="runQuickSetup"
            class="w-full py-3 rounded-xl bg-gradient-to-r from-[#1d4ed8] to-[#3b82f6] hover:from-[#1e40af] hover:to-[#2563eb] text-white font-semibold transition-all"
            style="box-shadow: 0 4px 20px rgba(59,130,246,0.25)"
          >
            {{ setupStep === 'error' ? 'Retry' : 'Get Started' }}
          </button>
          <div v-else-if="setupStep === 'done'" class="text-emerald-400 font-medium text-sm">All done ✓</div>
          <div v-else class="text-[#7aadcc] text-sm">Setting up...</div>
        </div>
      </div>
    </div>

    <!-- Update Available Modal -->
    <div
      v-if="updateInfo && updateInfo.available && !updateDismissed"
      class="fixed inset-0 z-50 flex items-center justify-center bg-black/60 backdrop-blur-sm"
    >
      <div class="relative w-full max-w-md mx-4 rounded-2xl border border-[#1e3a5f] bg-[#07101f] shadow-2xl overflow-hidden">
        <!-- Top accent -->
        <div class="h-px w-full bg-gradient-to-r from-transparent via-[#60a5fa] to-transparent"></div>
        <div class="p-6">
          <div class="flex items-start justify-between mb-4">
            <div>
              <h2 class="text-lg font-semibold text-[#93c5fd] tracking-wide">Update Available</h2>
              <p class="text-sm text-[#4a7fa8] mt-0.5">
                v{{ updateInfo.current_version }} → v{{ updateInfo.latest_version }}
              </p>
            </div>
            <button
              @click="updateDismissed = true"
              class="text-[#4a7fa8] hover:text-white transition-colors text-xl leading-none"
            >✕</button>
          </div>
          <div v-if="updateInfo.release_notes" class="mb-5 text-sm text-[#7aadcc] bg-[#0d1e33] rounded-lg p-3 max-h-40 overflow-y-auto whitespace-pre-wrap">
            {{ updateInfo.release_notes }}
          </div>
          <!-- Download progress -->
          <div v-if="updateDownloadProgress !== null" class="mb-4">
            <div class="flex justify-between text-xs text-[#7aadcc] mb-1.5">
              <span>{{ updateDownloadProgress >= 100 ? 'Installing...' : 'Downloading...' }}</span>
              <span>{{ updateDownloadProgress >= 100 ? '100%' : `${updateDownloadProgress}%` }}</span>
            </div>
            <div class="w-full h-2 bg-[#07101e] rounded-full overflow-hidden">
              <div
                class="h-full rounded-full transition-all duration-300"
                :class="updateDownloadProgress >= 100 ? 'bg-emerald-500' : 'bg-[#3b82f6]'"
                :style="`width: ${Math.min(updateDownloadProgress, 100)}%`"
              />
            </div>
            <p v-if="updateDownloadProgress >= 100" class="text-xs text-emerald-400 mt-2">
              Done! App is restarting...
            </p>
          </div>

          <div v-if="updateDownloadError" class="mb-3 text-xs text-red-400 bg-red-900/15 border border-red-900/30 rounded-lg px-3 py-2">
            {{ updateDownloadError }}
          </div>

          <div class="flex gap-2" v-if="updateDownloadProgress === null">
            <button
              v-if="updateInfo.download_url"
              @click="downloadUpdate(updateInfo.download_url)"
              class="flex-1 py-2.5 rounded-lg bg-gradient-to-r from-[#1d4ed8] to-[#3b82f6] hover:from-[#1e40af] hover:to-[#2563eb] text-white text-sm font-semibold transition-all"
              style="box-shadow: 0 2px 12px rgba(59,130,246,0.25)"
            >Download &amp; Install</button>
            <button
              v-else
              @click="openUrl(updateInfo.release_url); updateDismissed = true"
              class="flex-1 py-2.5 rounded-lg bg-[#1d4ed8] hover:bg-[#2563eb] text-white text-sm font-medium transition-colors"
            >Open Release Page</button>
            <button
              @click="updateDismissed = true"
              class="px-4 py-2.5 rounded-lg border border-[#1e3a5f] hover:border-[#2e5a9a] text-[#7aadcc] hover:text-[#b0cce0] text-sm transition-colors"
            >Later</button>
          </div>
        </div>
      </div>
    </div>

    <!-- Header -->
    <div class="relative overflow-hidden">
      <div class="absolute inset-0 midnight-bg">
        <div class="stars"></div>
        <div class="stars2"></div>
        <div class="moon-glow"></div>
      </div>
      <div class="absolute inset-0 bg-gradient-to-b from-transparent via-transparent to-[#07101e]"></div>
      <div class="relative z-10 container mx-auto px-4 md:px-8 py-10 md:py-14">
        <div class="text-center">
          <h1 class="text-4xl md:text-5xl font-bold tracking-widest midnight-title mb-2">
            TALENT HERON
          </h1>
          <div class="h-px w-32 mx-auto bg-gradient-to-r from-transparent via-[#7dd3fc] to-transparent opacity-40 mb-3"></div>
          <div class="flex items-center justify-center gap-3">
            <p class="text-[#7aadcc] text-sm tracking-widest uppercase">
              Auto-update WoW talents from Archon.gg
            </p>
            <span v-if="updateInfo" class="text-xs px-2 py-0.5 rounded-full border cursor-pointer transition-all"
              :class="updateInfo.available
                ? 'bg-[#1d4ed8]/20 border-[#3b82f6]/40 text-[#93c5fd] hover:bg-[#1d4ed8]/30'
                : 'bg-[#0e1d33] border-[#1e3a5f] text-[#5580a0]'"
              @click="updateDismissed = false"
            >
              {{ updateInfo.available ? `v${updateInfo.latest_version} available` : `v${updateInfo.current_version}` }}
            </span>
          </div>
        </div>
      </div>
    </div>

    <!-- Main Content -->
    <div class="container mx-auto max-w-4xl px-4 md:px-8 pb-10">
      <div class="bg-[#0e1d33]/95 rounded-2xl border border-[#1e3a5f] shadow-2xl">
        <div class="p-5 md:p-7">
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
            :addon-installed="addonInstalled"
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

<style scoped>
.midnight-bg {
  background: radial-gradient(ellipse at 50% 0%, #0f1f4a 0%, #060d1f 40%, #07101e 100%);
}

.moon-glow {
  position: absolute;
  top: -80px;
  left: 50%;
  transform: translateX(-50%);
  width: 400px;
  height: 400px;
  background: radial-gradient(circle, rgba(147, 197, 253, 0.08) 0%, rgba(96, 165, 250, 0.04) 40%, transparent 70%);
  border-radius: 50%;
  pointer-events: none;
}

.midnight-title {
  background: linear-gradient(180deg, #e2eeff 0%, #93c5fd 50%, #60a5fa 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  filter: drop-shadow(0 0 24px rgba(96, 165, 250, 0.4));
}

.stars {
  position: absolute;
  inset: 0;
  background-image:
    radial-gradient(1px 1px at 10% 15%, rgba(255,255,255,0.7) 0%, transparent 100%),
    radial-gradient(1px 1px at 25% 40%, rgba(255,255,255,0.5) 0%, transparent 100%),
    radial-gradient(1.5px 1.5px at 40% 8%, rgba(255,255,255,0.8) 0%, transparent 100%),
    radial-gradient(1px 1px at 55% 30%, rgba(255,255,255,0.4) 0%, transparent 100%),
    radial-gradient(1px 1px at 70% 12%, rgba(255,255,255,0.6) 0%, transparent 100%),
    radial-gradient(1.5px 1.5px at 82% 22%, rgba(255,255,255,0.7) 0%, transparent 100%),
    radial-gradient(1px 1px at 92% 5%, rgba(255,255,255,0.5) 0%, transparent 100%),
    radial-gradient(1px 1px at 15% 60%, rgba(255,255,255,0.3) 0%, transparent 100%),
    radial-gradient(1px 1px at 35% 75%, rgba(255,255,255,0.4) 0%, transparent 100%),
    radial-gradient(1px 1px at 60% 55%, rgba(255,255,255,0.3) 0%, transparent 100%),
    radial-gradient(1px 1px at 78% 68%, rgba(255,255,255,0.4) 0%, transparent 100%),
    radial-gradient(1px 1px at 90% 80%, rgba(255,255,255,0.3) 0%, transparent 100%);
  animation: twinkle 6s ease-in-out infinite alternate;
}

.stars2 {
  position: absolute;
  inset: 0;
  background-image:
    radial-gradient(1px 1px at 18% 25%, rgba(147,197,253,0.6) 0%, transparent 100%),
    radial-gradient(1px 1px at 45% 18%, rgba(147,197,253,0.4) 0%, transparent 100%),
    radial-gradient(1.5px 1.5px at 63% 42%, rgba(147,197,253,0.7) 0%, transparent 100%),
    radial-gradient(1px 1px at 85% 35%, rgba(147,197,253,0.5) 0%, transparent 100%),
    radial-gradient(1px 1px at 5% 50%, rgba(147,197,253,0.3) 0%, transparent 100%),
    radial-gradient(1px 1px at 30% 88%, rgba(147,197,253,0.3) 0%, transparent 100%),
    radial-gradient(1px 1px at 72% 82%, rgba(147,197,253,0.4) 0%, transparent 100%);
  animation: twinkle 8s ease-in-out infinite alternate-reverse;
}

@keyframes twinkle {
  0%   { opacity: 0.6; }
  100% { opacity: 1; }
}
</style>

