# Examples of Essences

## Essences

### [Dark Essence]

Name: Dark Essence
Flavor: "Manifested essence of darkness"
Rank: Unranked
Rarity: Uncommon
Description:
Requirements: Less than 4 absorbed essences.
Effect: Imbues 1 awakened dark essence ability and 4 unawakened dark essence abilities.

Item, Consumable, Essence

## Using Essences

### Essence Template

You have absorbed [{essence}]. You have absorbed {n} of 4 essences.
Progress to iron rank: {n \* 25}% ({n}/4 essences).

[{essence}] has bonded to your [{attribute: Power | Recovery | Spirit | Speed} ] attribute, changing your [{attribute}] from normal to [Iron 0]. Master all {lowercase_essence} abilities to increase your [{attribute}] attribute.
You have awakened the {lowercase_essence} ability [{ability_name}]. You have awakened 1 of 5 {lowercase_essence} abilities.

Developer notes:

- When using an essence a player will always be normal rank, as 4 essences are required to advance to iron rank.
- Each {essence} will bind to one {attribute}. Only one essence may be bound to each attribute, so if Essence Foo is bound to the Power attribute, only the Recovery, Spirit and Speed attributes will remain avaiable for the next essence.
- When awakening an essence, one essence ability is always awakened from that essence as well.

### Dark Essence Example

You have absorbed [Dark Essence]. You have absorbed 1 of 4 essences.
Progress to iron rank: 25% (1/4 essences).

[Dark Essence] has bonded to your [Speed] attribute, changing your [Speed] from normal to [Iron 0]. Master all dark essence abilities to increase your [Speed] attribute.

You have awakened the dark essence ability [Midnight Eyes]. You have awakened 1 of 5 dark essence abilities.

#### Ability: [Midnight Eyes] (Dark)

Special ability (perception)
Base cost: None.
Cooldown: None.
Current rank: Iron 0 (00%)
Effect (Iron): See through darkness.

You have 4 unawakened essence abilities.

You are able to absorb [Awakening Stone of the Stars]. Absorb Y/N? (Y)

You have awakened the dark essence ability [Cloak of Night]. You have awakened 2 of 5 dark essence abilities.

#### Ability: [Cloak of Night] (Dark)

Conjuration (darkness, light, dimension).
Base cost: Moderate mana to conjure.
Current rank: Iron 0 (00%).
Cooldown: None.
Effect (Iron): Conjures a magical cloak that can alter the wearer. Offers limited physical protection. Can generate light, or blend into shadows. Cloak can reduce the weight of the wearer for a low mana-per-second cost, allowing reduced falling speed and water walking. Cannot be given or taken away, although effects can be extended to others in very close proximity.
