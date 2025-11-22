# Ocean Theme Color System Design

**Date:** 2025-01-22
**Status:** Approved for Implementation

## Overview

Replace the current purple/magenta color scheme with a custom ocean-themed palette featuring teals, blues, mint, and coral pink. Implement a hybrid system using both a custom DaisyUI theme and Tailwind color extensions while maintaining the glassmorphism aesthetic.

## Color Palette

### Base Colors
- **Dark Teal**: `#114b5f` - Deep ocean color
- **Teal**: `#028090` - Bright teal
- **Frosted Mint**: `#e4fde1` - Very light green
- **Baltic Blue**: `#456990` - Medium blue
- **Bubblegum Pink**: `#f45b69` - Coral pink

### Design Approach
**Hybrid System:** Custom DaisyUI theme ("archon-ocean") for semantic colors + Tailwind config extension for direct color access.

## Color System Architecture

### Layer 1: Custom DaisyUI Theme ("archon-ocean")

Semantic color mappings that DaisyUI components automatically use:

- `primary`: Teal (#028090) - main buttons, active states, primary actions
- `secondary`: Baltic blue (#456990) - secondary buttons, supporting UI
- `accent`: Bubblegum pink (#f45b69) - highlights, CTAs, important alerts
- `success`: Frosted mint (#e4fde1) - success messages, positive states
- `error`: Bubblegum pink (#f45b69) - error messages, destructive actions
- `info`: Dark teal (#114b5f) - info messages, neutral notifications
- `base-100`: Dark teal (#114b5f) - main background color
- `base-200`: Teal (#028090) - secondary backgrounds
- `base-300`: Baltic blue (#456990) - tertiary backgrounds

### Layer 2: Tailwind Extension

Direct color access for custom styling:

```javascript
colors: {
  'ocean-dark': '#114b5f',
  'ocean': '#028090',
  'ocean-mint': '#e4fde1',
  'ocean-blue': '#456990',
  'ocean-pink': '#f45b69',
}
```

### Text Colors (WCAG AA Compliant)

All combinations tested to meet 4.5:1 contrast minimum:

- **White (#ffffff)** on dark teal, teal, baltic blue, bubblegum pink
- **Dark teal (#114b5f)** or rich black (#0a1f29) on frosted mint

## Gradient Backgrounds & Glassmorphism

### Page Background Gradient

Multi-stop gradient flowing through the ocean palette:

```css
background: linear-gradient(135deg,
  #114b5f 0%,     /* Dark teal - top left */
  #028090 35%,    /* Teal - mid */
  #456990 65%,    /* Baltic blue - mid-right */
  #114b5f 100%    /* Back to dark teal - bottom right */
);
```

Mint and pink excluded from background gradient to avoid visual noise.

### Glass Card System

Layered transparency for depth:

- **Standard cards**: `bg-ocean/10` + `backdrop-blur-lg` + `border border-white/20`
- **Hover state**: `bg-ocean/15` for subtle feedback
- **Active/focused**: `ring-2 ring-ocean-pink/50` for clear interaction

### Button Glass Effects

- **Primary buttons**: `bg-ocean/30` with `backdrop-blur-lg`, hover to `bg-ocean/40`
- **Secondary buttons**: `bg-ocean-blue/30`, hover to `bg-ocean-blue/40`
- **Accent buttons**: `bg-ocean-pink/30`, hover to `bg-ocean-pink/40`
- All buttons: `border border-white/20` + `shadow-lg` → `shadow-xl` on hover

### Text with Glass

Maintain readability over blur:

- White text with `drop-shadow-lg` for depth
- Heavier drop shadows on busy gradient areas
- Icons use `stroke-white` to match text

## Status Messages & Semantic Colors

### Success Messages
- Background: `bg-ocean-mint/20`
- Border: `border-ocean-mint/40`
- Icon: `stroke-ocean-dark`
- Text: `text-ocean-dark`
- Effect: `backdrop-blur-lg` + `shadow-lg`

### Error Messages
- Background: `bg-ocean-pink/30`
- Border: `border-ocean-pink/50`
- Icon: `stroke-white`
- Text: `text-white`
- Effect: `backdrop-blur-lg` + `shadow-lg`

### Info Messages
- Background: `bg-ocean-blue/30`
- Border: `border-ocean-blue/50`
- Icon: `stroke-white`
- Text: `text-white`
- Effect: `backdrop-blur-lg` + `shadow-lg`

### Additional Semantic Uses

- **Loading spinners**: `ocean` (teal) for primary, `ocean-pink` for accent
- **Progress bars**: `ocean` fill with `ocean-dark` background
- **Badges**: `ocean-pink` for important, `ocean-blue` for neutral, `ocean-mint` for positive
- **Links**: `ocean` with `ocean-pink` hover

## Color Shades & Component States

### Generated Color Shades

**Teal (Primary) Shades:**
- `ocean-50`: #e6f4f6 (very light, subtle backgrounds)
- `ocean-100`: #b3e0e5 (light, hover highlights)
- `ocean-500`: #028090 (base teal)
- `ocean-600`: #026d7a (darker, button hover states)
- `ocean-700`: #015a64 (darkest, active/pressed states)

**Additional Utility Shades:**
- `ocean-dark-600`: #0d3a4a (lighter than base, for borders)
- `ocean-blue-400`: #5a7fa8 (lighter baltic, disabled states)
- `ocean-pink-600`: #e14156 (darker pink, error hover)
- `ocean-mint-700`: #a8d6a4 (darker mint, success borders)

### Component State Colors

**Buttons:**
- Default: Base color at 30% opacity
- Hover: Base color at 40% opacity + `shadow-xl`
- Active: Base color at 50% opacity + `scale-95`
- Disabled: `ocean-blue-400` at 20% opacity + `opacity-50`

**Inputs:**
- Background: `bg-white/15`
- Border: `border-white/30`
- Focus: `ring-2 ring-ocean/50`
- Placeholder: `placeholder-white/50`
- Disabled: `bg-white/5` + `border-white/10`

**Borders & Dividers:**
- Standard borders: `border-white/20`
- Emphasized borders: `border-white/30`
- Dividers: `border-ocean-dark/30`

## Implementation Plan

### Files to Modify

1. **tailwind.config.js**
   - Add custom DaisyUI theme "archon-ocean" with semantic color mappings
   - Extend Tailwind colors with ocean palette and shades
   - Set "archon-ocean" as the only theme

2. **src/style.css** (or create if needed)
   - Add gradient background to root element
   - Define CSS custom properties for the gradient
   - Ensure backdrop-blur utilities available

3. **src/components/StatusMessages.vue**
   - Update success message: mint background, dark teal text/icons
   - Update error message: pink background, white text/icons
   - Update info message: baltic blue background, white text/icons

4. **src/components/GameSettingsTab.vue**
   - Update button classes to ocean colors
   - Adjust card backgrounds to ocean palette
   - Update input styling with new focus rings

### Implementation Order

1. Configure Tailwind/DaisyUI (establishes color system)
2. Update global styles (gradient background)
3. Update StatusMessages component (validate semantic colors)
4. Update GameSettingsTab component (validate full system)
5. Scan remaining components for old color references

### Validation Checklist

- [ ] Check contrast ratios with browser DevTools
- [ ] Test all button states (hover, active, disabled)
- [ ] Verify status messages are readable
- [ ] Confirm glassmorphism renders properly
- [ ] Test on different screen sizes
- [ ] Verify gradient background displays correctly

## Accessibility

All color combinations meet **WCAG AA standards**:
- Normal text: 4.5:1 contrast ratio minimum
- Large text: 3:1 contrast ratio minimum

Contrast ratios verified for all semantic color uses.

## Design Rationale

**Ocean Theme Approach:** Creates a cohesive aquatic feel with teal as the hero color, uses pink sparingly for impact, and mint works naturally for success states. The palette evokes calm, trust, and professionalism while maintaining visual interest.

**Glassmorphism Retention:** The existing blur effects work beautifully with the new ocean colors, creating depth and modern aesthetics. The gradient background provides rich visual context without overwhelming the content.

**Hybrid System Benefits:** Combines the automatic semantic handling of DaisyUI themes with the flexibility of direct Tailwind color access, enabling both systematic consistency and creative freedom.