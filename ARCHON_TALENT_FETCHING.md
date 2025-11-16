# Archon Talent Fetching Documentation

This document outlines how the ArchonConfigUpdater program fetches talent builds from Archon.gg for World of Warcraft characters.

## Base URL Structure

The base URL pattern for Archon talent builds is:
```
https://www.archon.gg/wow/builds/{spec}/{class}/{contentType}/{tier}/{difficulty}/{encounter}
```

### URL Parameters

| Parameter | Description | Example Values |
|-----------|-------------|----------------|
| `{spec}` | Character specialization (lowercase) | `frost`, `unholy`, `protection`, `demonology` |
| `{class}` | Character class (lowercase, with hyphens) | `mage`, `warrior`, `death-knight`, `demon-hunter` |
| `{contentType}` | Type of content | `raid`, `mythic-plus` |
| `{tier}` | Tier/overview type | `overview` (raids), `overview/10` (M+) |
| `{difficulty}` | Raid difficulty (empty for M+) | `heroic`, `normal`, `mythic` |
| `{encounter}` | Boss or dungeon name (lowercase) | `broodtwister`, `ara-kara` |

### Class Name Mapping

Most classes use their name in lowercase, but these require special handling:

| Internal Name | URL Format |
|---------------|------------|
| `DeathKnight` | `death-knight` |
| `DemonHunter` | `demon-hunter` |
| All others | Lowercase version (e.g., `Warrior` → `warrior`) |

## Specialization Index Mapping

Each specialization has a numeric index used internally:

### Warrior
- Arms: 1
- Fury: 2
- Protection: 3

### Paladin
- Holy: 1
- Protection: 2
- Retribution: 3

### Hunter
- Beast Mastery: 1
- Marksmanship: 2
- Survival: 3

### Rogue
- Assassination: 1
- Combat: 2
- Subtlety: 3

### Priest
- Discipline: 1
- Holy: 2
- Shadow: 3

### Death Knight
- Blood: 1
- Frost: 2
- Unholy: 3

### Shaman
- Elemental: 1
- Enhancement: 2
- Restoration: 3

### Mage
- Arcane: 1
- Fire: 2
- Frost: 3

### Warlock
- Affliction: 1
- Demonology: 2
- Destruction: 3

### Monk
- Brewmaster: 1
- Mistweaver: 2
- Windwalker: 3

### Druid
- Balance: 1
- Feral: 2
- Guardian: 3
- Restoration: 4

### Demon Hunter
- Havoc: 1
- Vengeance: 2

### Evoker
- Devastation: 1
- Preservation: 2

## URL Construction Examples

### Raid Boss Talent Build
```
https://www.archon.gg/wow/builds/frost/mage/raid/overview/heroic/broodtwister
```

**Breakdown:**
- Spec: `frost`
- Class: `mage`
- Content Type: `raid`
- Tier: `overview`
- Difficulty: `heroic`
- Encounter: `broodtwister`

### Mythic+ Dungeon Build (Current Week)
```
https://www.archon.gg/wow/builds/unholy/death-knight/mythic-plus/overview/10//ara-kara/this-week
```

**Breakdown:**
- Spec: `unholy`
- Class: `death-knight`
- Content Type: `mythic-plus`
- Tier: `overview/10`
- Difficulty: (empty, note the double slash `//`)
- Encounter: `ara-kara`
- Time Period: `this-week`

### Mythic+ Dungeon Build (Last Week)
```
https://www.archon.gg/wow/builds/protection/warrior/mythic-plus/overview/10//mists-of-tirna-scithe/last-week
```

**Breakdown:**
- Same structure as above, but with `last-week` suffix

## Fetching Process

### 1. Raid Builds

For raid encounters, the process is straightforward:

```
GET https://www.archon.gg/wow/builds/{spec}/{class}/raid/overview/{difficulty}/{encounter}
```

Example:
```
GET https://www.archon.gg/wow/builds/shadow/priest/raid/overview/heroic/sikran
```

### 2. Mythic+ Builds

For Mythic+ dungeons, the program uses a fallback strategy:

1. **Determine Time Period**:
   - If today is Wednesday (reset day): Try `last-week` first
   - Otherwise: Try `this-week` first

2. **Primary Request**:
   ```
   GET https://www.archon.gg/wow/builds/{spec}/{class}/mythic-plus/overview/10//{dungeon}/{timespan}
   ```

3. **Fallback Request**:
   - If primary fails or returns empty, try the opposite timespan
   - This ensures you get data even if the current week doesn't have enough statistics yet

Example for Wednesday (reset day):
```
Primary:  GET .../mythic-plus/overview/10//ara-kara/last-week
Fallback: GET .../mythic-plus/overview/10//ara-kara/this-week
```

Example for other days:
```
Primary:  GET .../mythic-plus/overview/10//ara-kara/this-week
Fallback: GET .../mythic-plus/overview/10//ara-kara/last-week
```

## Extracting Talent String from Response

The HTML response contains a link to Wowhead's talent calculator. To extract the talent string:

1. Parse the HTML response
2. Find the anchor tag: `<a href="...wowhead.com/talent-calc/blizzard/...">`
   - XPath: `//a[contains(@href, 'wowhead.com/talent-calc/blizzard/')]`
3. Extract the `href` attribute value
4. Remove the prefix `https://www.wowhead.com/talent-calc/blizzard/`
5. The remaining string is the talent loadout string

**Example:**
```
Full URL: https://www.wowhead.com/talent-calc/blizzard/mage/frost/DABCabc123XYZ
Talent String: mage/frost/DABCabc123XYZ
```

## Error Handling

### HTTP 500 (Internal Server Error)
- Indicates insufficient data for the requested build
- This is expected for new bosses or unpopular spec/encounter combinations
- Handle gracefully by returning empty/null

### Empty Results
- If no talent link is found in the HTML, return empty/null
- Move to fallback timespan for M+ builds if applicable

## Configuration Structure

### Config File Format (JSON)

```json
{
  "characters": [
    {
      "name": "CharacterName",
      "class": "Warlock",
      "specializations": ["demonology", "destruction"]
    }
  ],
  "raidDifficulties": ["heroic", "normal"],
  "raidBosses": ["broodtwister", "sikran", "queen-ansurek"],
  "dungeons": ["ara-kara", "city-of-threads", "mists-of-tirna-scithe"],
  "clearPreviousBuilds": false
}
```

### Config Properties

| Property | Type | Description |
|----------|------|-------------|
| `characters` | Array | List of characters with their class and specializations |
| `raidDifficulties` | Array | Raid difficulties to fetch (e.g., `["heroic", "normal", "mythic"]`) |
| `raidBosses` | Array | List of boss names (lowercase, hyphenated) |
| `dungeons` | Array | List of dungeon names (lowercase, hyphenated) |
| `clearPreviousBuilds` | Boolean | Whether to clear previous builds before updating |

### Character Object

| Property | Type | Description |
|----------|------|-------------|
| `name` | String | Character name (for identification only) |
| `class` | String | Class name (PascalCase, e.g., `DeathKnight`, `DemonHunter`) |
| `specializations` | Array | List of spec names (lowercase, e.g., `["frost", "unholy"]`) |

## Talent Identifier Generation

The program generates unique identifiers for each talent build:

### Raid Builds
Format: `R-{difficulty}-{boss}`

Examples:
- `R-heroic-broodtwister`
- `R-normal-sikran`
- `R-mythic-queen-ansurek`

### Mythic+ Builds
Format: `M+-{dungeon}`

Examples:
- `M+-ara-kara`
- `M+-city-of-threads`
- `M+-mists-of-tirna-scithe`

## Rate Limiting

The program implements:
- **Semaphore**: Maximum 5 concurrent requests to Archon
- **Connection Pooling**: Up to 10 connections per server
- **Timeout**: 3 minutes per request
- **Keep-Alive**: Connections reused when possible
- **No-Cache**: Forces fresh data retrieval

## WoW Integration: Reading and Writing Talent Data

This program integrates with World of Warcraft through the **TalentLoadoutsEx** addon, which stores talent loadouts in a Lua saved variables file.

### File Path Structure

The addon's saved variables file is located at:
```
<WoW Installation>/WTF/Account/<Account ID>/SavedVariables/TalentLoadoutsEx.lua
```

**Platform-specific examples:**

**Windows:**
```
C:\Program Files (x86)\World of Warcraft\_retail_\WTF\Account\400793633#1\SavedVariables\TalentLoadoutsEx.lua
```

**macOS:**
```
/Applications/World of Warcraft/_retail_/WTF/Account/400793633#1/SavedVariables/TalentLoadoutsEx.lua
```

**Linux:**
```
~/.wine/drive_c/Program Files (x86)/World of Warcraft/_retail_/WTF/Account/400793633#1/SavedVariables/TalentLoadoutsEx.lua
```

**Important notes:**
- The `Account` folder contains your Battle.net account ID (not your character name)
- For multiple WoW accounts, there will be multiple `Account` folders (e.g., `Account1`, `Account2`)
- The file is in Lua table format, not JSON

### TalentLoadoutsEx.lua File Format

The file contains a Lua table structure organized by class and specialization:

```lua
TalentLoadoutEx = {
  ["WARRIOR"] = {
    [1] = {  -- Arms (spec index 1)
      { ["icon"] = 132355, ["name"] = "My Arms Build", ["text"] = "warrior/arms/ABC123..." },
      { ["icon"] = 0, ["name"] = "R-heroic-sikran_ARCT", ["text"] = "warrior/arms/XYZ789..." },
    },
    [2] = {  -- Fury (spec index 2)
      { ["icon"] = 132347, ["name"] = "My Fury Build", ["text"] = "warrior/fury/DEF456..." },
    },
    [3] = {  -- Protection (spec index 3)
      { ["icon"] = 132341, ["name"] = "Tank Build", ["text"] = "warrior/protection/GHI789..." },
    },
  },
  ["MAGE"] = {
    [1] = {  -- Arcane
      { ["icon"] = 135932, ["name"] = "M+-ara-kara_ARCT", ["text"] = "mage/arcane/PQR123..." },
    },
    [3] = {  -- Frost
      { ["icon"] = 135846, ["name"] = "R-mythic-broodtwister_ARCT", ["text"] = "mage/frost/STU456..." },
    },
  },
  ["OPTION"] = { ["IsEnabledPvp"] = false },
}
```

### TalentLoadoutExTalent Object Structure

Each talent entry in the file has these properties:

| Property | Type | Required | Description |
|----------|------|----------|-------------|
| `icon` | Number | Yes | Spell icon ID (use 0 if unknown) |
| `name` | String | Yes | Display name for the talent loadout |
| `text` | String | Yes | The talent string from Wowhead (e.g., `mage/frost/ABC123...`) |
| `isExpanded` | Boolean | No | UI state: whether the loadout is expanded in the addon |
| `isInGroup` | Boolean | No | UI state: whether the loadout is in a group |

### How the Program Reads Existing Talents

1. **Read the TalentLoadoutsEx.lua file**:
   ```
   FilePath: <outputPath from settings.json>
   ```

2. **Parse the Lua table structure**:
   - Extract class keys (e.g., `["WARRIOR"]`, `["MAGE"]`)
   - Extract spec indices for each class (e.g., `[1]`, `[2]`, `[3]`)
   - Parse talent objects within each spec array

3. **Preserve existing talents**:
   - The program reads ALL existing talents in the file
   - It only modifies/removes talents with the `_ARCT` suffix (auto-generated)
   - User-created talents are preserved

### How the Program Writes Updated Talents

1. **Identify auto-generated talents**:
   - All talents fetched from Archon are suffixed with `_ARCT`
   - Example: `R-heroic-sikran_ARCT`, `M+-ara-kara_ARCT`

2. **Upsert process** (for each class/spec combination):
   - Remove all existing talents ending with `_ARCT`
   - Add new talents fetched from Archon (with `_ARCT` suffix)
   - Keep all other talents unchanged

3. **Write back to file**:
   - Convert the entire structure back to Lua table format
   - Overwrite the `TalentLoadoutsEx.lua` file
   - WoW will load the updated talents next time the addon loads

### Generated Talent Naming

The program uses a specific naming convention for auto-generated talents:

**Format:** `{identifier}_ARCT`

Where `_ARCT` = **Arc**hon **T**alent (auto-generated marker)

**Examples:**
- `R-heroic-sikran_ARCT` (Raid: Heroic Sikran)
- `M+-ara-kara_ARCT` (Mythic+: Ara-Kara dungeon)
- `R-mythic-queen-ansurek_ARCT` (Raid: Mythic Queen Ansurek)

### How Characters and Classes are Determined

The program **does NOT** read character data from WoW. Instead, it uses the `settings.json` configuration file to determine:

- Which classes to fetch talents for
- Which specializations within those classes
- Which encounters (bosses/dungeons) to fetch

**Why this approach?**
- WoW doesn't store a central list of "your characters" in an easily accessible format
- Different characters may be on different realms/servers
- The SavedVariables files are account-wide, not character-specific
- This allows you to pre-fetch talents for classes you don't have characters for yet

### Alternative: Custom File Format

If you want to persist talents in your own format instead of using the TalentLoadoutsEx addon:

#### Option 1: JSON Format
```json
{
  "Warrior": {
    "1": [
      {
        "name": "R-heroic-sikran",
        "talentString": "warrior/arms/ABC123...",
        "icon": 0
      }
    ],
    "2": [
      {
        "name": "M+-ara-kara",
        "talentString": "warrior/fury/XYZ789...",
        "icon": 0
      }
    ]
  },
  "Mage": {
    "3": [
      {
        "name": "R-mythic-broodtwister",
        "talentString": "mage/frost/DEF456...",
        "icon": 0
      }
    ]
  }
}
```

#### Option 2: Simple Text Format
```
# Class: Warrior, Spec: 1 (Arms)
R-heroic-sikran=warrior/arms/ABC123...
M+-ara-kara=warrior/arms/XYZ789...

# Class: Warrior, Spec: 2 (Fury)
R-heroic-sikran=warrior/fury/DEF456...

# Class: Mage, Spec: 3 (Frost)
R-mythic-broodtwister=mage/frost/GHI789...
```

#### Option 3: CSV Format
```csv
class,spec_index,spec_name,identifier,talent_string
Warrior,1,Arms,R-heroic-sikran,warrior/arms/ABC123...
Warrior,1,Arms,M+-ara-kara,warrior/arms/XYZ789...
Warrior,2,Fury,R-heroic-sikran,warrior/fury/DEF456...
Mage,3,Frost,R-mythic-broodtwister,mage/frost/GHI789...
```

### clearPreviousBuilds Setting

The `clearPreviousBuilds` config option controls how auto-generated talents are handled:

**When `false` (default):**
- Only removes and replaces talents with `_ARCT` suffix for the specific class/spec being updated
- Preserves auto-generated talents for classes/specs NOT in your config

**When `true`:**
- Removes ALL talents ending with `_ARCT` across ALL classes and specs
- Useful for completely refreshing all auto-generated builds
- Still preserves user-created talents (those without `_ARCT`)

## Complete Request Flow

1. **Parse Configuration**: Load settings.json with character details
2. **Read Existing Talents**: Parse TalentLoadoutsEx.lua to get current saved talents
3. **For Each Character/Spec Combination**:
   - For each raid boss + difficulty: Fetch raid build from Archon
   - For each dungeon: Fetch M+ build (with this-week/last-week fallback)
4. **Fetch URL**: Make HTTP GET request to constructed Archon URL
5. **Parse HTML**: Extract Wowhead talent calculator link
6. **Extract Talent String**: Remove URL prefix to get talent loadout code
7. **Build Talent Object**: Create talent with name, text (talent string), and icon
8. **Upsert Talents**: Remove old auto-generated talents, add new ones
9. **Write Back to File**: Convert to Lua format and save to TalentLoadoutsEx.lua

## Implementation Checklist

When porting to another language, ensure you:

**Archon API Integration:**
- [ ] Implement class name mapping (DeathKnight → death-knight, DemonHunter → demon-hunter)
- [ ] Convert all URL components to lowercase
- [ ] Handle empty difficulty parameter for M+ (results in `//` in URL)
- [ ] Implement Wednesday detection for M+ timespan selection
- [ ] Add fallback logic for M+ (try both this-week and last-week)
- [ ] Parse HTML to extract Wowhead talent calculator links
- [ ] Strip `https://www.wowhead.com/talent-calc/blizzard/` prefix from links
- [ ] Handle HTTP 500 errors as "no data available" (not fatal errors)
- [ ] Implement rate limiting (max 5 concurrent requests recommended)
- [ ] Set appropriate HTTP timeout (3 minutes recommended)
- [ ] Include proper HTTP headers (keep-alive, no-cache)

**Talent Data Management:**
- [ ] Map specializations to their numeric indices (1-4 depending on class)
- [ ] Generate talent identifiers (R-{diff}-{boss} or M+-{dungeon})
- [ ] Add `_ARCT` suffix to all auto-generated talents
- [ ] Implement Lua table parser for reading TalentLoadoutsEx.lua
- [ ] Implement Lua table formatter for writing TalentLoadoutsEx.lua
- [ ] Preserve user-created talents (those without `_ARCT` suffix)
- [ ] Handle class keys as uppercase strings (e.g., "WARRIOR", "MAGE")
- [ ] Handle spec indices as numeric keys (1-4)
- [ ] Support clearPreviousBuilds option for full refresh
- [ ] Create outputPath config for TalentLoadoutsEx.lua location
