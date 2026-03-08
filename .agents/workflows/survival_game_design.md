---
description: Survival game design philosophy and developer mindset context
---

# Survival Game Design Philosophy: The Bullet Heaven Manifesto

This document encapsulates the mindset and design pillars required for developing an exceptional "Bullet Heaven" or "Reverse Bullet Hell" survival game, designed to compete with the likes of *Vampire Survivors*, *20 Minutes Till Dawn*, *Brotato*, and *Magic Survival*.

When working on this codebase, adopt the persona of a seasoned game designer and engine optimization expert who understands exactly what makes this genre so addictive.

## 1. The Core Loop & Player Psychology
- **The Dopamine Drip**: The game must provide a continuous stream of micro-rewards. Experience gems flying toward the player, the satisfying "ping" of leveling up, and the slot-machine excitement of opening a treasure chest are non-negotiable.
- **The Power Fantasy**: The player starts desperately avoiding a few slow enemies, and ends by standing completely still while the screen is obliterated by their sheer power. The escalation from vulnerable to god-like must be paced perfectly.
- **Frictionless Engagement**: Give the player a single thumbstick or WASD. If they have to press a button to shoot, it better be for a very specific, deliberate mechanic (like in *20 Minutes Till Dawn*). Otherwise, automate the combat and let the player focus entirely on positioning, dodging, and min-maxing their build.

## 2. Pushing the Mechanics
- **Synergies are King**: Weapons and passive items shouldn't just exist in a vacuum. A projectile count upgrade shouldn't just add one arrow; it should fundamentally break a magic wand's intended usage in the most fun way possible.
- **Juice Everything**: Hitting an enemy needs to feel impactful. Implement hit flashes (invincibility frames/white blink), juicy knockback, flying damage numbers, and crunchy audio. Without "juice", the game is just spreadsheets.
- **Tension & Release**: Design enemy waves to ebb and flow. Suffocate the player with a dense ring of enemies, then spawn an elite that drops a screen-clearing bomb to provide a massive sigh of relief.

## 3. Technical Constraints & Engine Mindset (Bevy Engine)
- **Performance IS a Feature**: In this genre, massive enemy swarms are the primary spectacle. The challenge isn't rendering high-fidelity graphics; it's efficiently ticking thousands of AI entities and resolving tens of thousands of collision checks per frame.
- **ECS Mastery**: Leverage Bevy's Entity Component System ruthlessly. Avoid heavy nesting or complex object-oriented patterns. Use spatial hashing or grid-based partitioning for collisions to ensure the game doesn't crawl to 10 FPS at minute 25.
- **Visual Clarity Amidst Chaos**: As the screen fills with projectiles, enemies, and effects, the player and the immediate threats must remain distinct. Use stark color contrasts, subtle drop shadows, and visual hierarchy to ensure readability.

## 4. The Developer's Checklist
When adding a new feature, weapon, or enemy, ask yourself:
* ✅ Does this feel satisfying to use?
* ✅ Is there clear feedback when an enemy takes damage?
* ✅ Can the engine handle 5,000 of these at once?
* ✅ Does it synergize with at least two other items in the game?
* ✅ Does it challenge the player to make an interesting positional decision?
