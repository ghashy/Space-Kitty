<div align = "center">

# Space-Kitty

</div>

<img width="1920" alt="Space Kitty Presentation" src="https://github.com/ghashy/Space-Kitty/assets/128966780/b06f223c-996c-4d91-8275-8b30d18124a9">

## Description

![git text](https://github.com/ghashy/Space-Kitty/assets/128966780/c86e455c-4a38-4f62-a34f-aa79f3f8c766)

![Space Kitty - Trailer](https://github.com/ghashy/Space-Kitty/assets/109857267/17c39889-de77-4e68-aac7-ec74ba57f382)


### Specifications

Made with [Bevy game engine](https://github.com/bevyengine/bevy). Enspired by [Learn Bevy Engine 0.10 series](https://www.youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd).

### Used crates:

* [bevy_hanabi](https://github.com/djeedai/bevy_hanabi) - very convenient to work with particles system.
* [bevy_tweening](https://github.com/djeedai/bevy_tweening) - big help for small tweens!
* [kira](https://github.com/tesselode/kira) - the most powerful rust audio library for creating games.
* [rapier2d](https://github.com/dimforge/rapier) - one and only :).
* [egui](https://github.com/emilk/egui) - can't debug without it!

### To-do:

#### Design:

- [x] Fish-boss
- [x] Audio design
- [x] Design border, 12 dog’s skins, new cat’s skin
- [x] Add fish score icon
- [x] Design new game icon
- [x] Design health of the ship
- [x] Design Game Over screen
- [x] Design game presentation image
- [x] Design animation for Big Buddy
- [x] Design Message box
- [x] Add last collision sound for Kitty

#### Coding:

- [x] Implement audio
- [x] Implement highscore system
- [x] Dog’s message when rotating wildly
- [x] Status bar
- [x] Dog’s spawn outside the screen
- [x] Entity’s message when collecting every first and 10th cracker
- [x] Dog’s message when picking cracker randomly
- [x] Message when new dog arriving
- [x] Crackers particles when kitty colliding with dog
- [x] Drop 25% of Kitty's crackers on colliding with dog.
- [x] Implement gameover screen
- [ ] Document code
- [x] Decrease Magic Wand's scale
- [x] Rework collision system
- [x] Resizing screen fix
- [x] Doggy theme particles
- [x] Dogs loots more fish
- [x] Restrict max amount of spawned crackers at the same time
- [x] Update avatar images
- [x] Add Big dog animation
- [x] Fix aduio clipping on Master
- [x] Add health regerenation (a glass of milk)
- [x] Add controls sheet at the start of the game
- [x] Despawn notes on exit game state
- [x] Sort high scores
- [x] Despawn milk when exit from game state
- [x] Add sounds: picking milk, gameover screen music, pressing buttons
- [x] Improve sprites loading in the entire project
- [x] Fix alignement on the gamover screen
- [x] Big Dog doesn't show up on the high scores
- [x] Reset high scores after each game
- [x] Add spacesuits for the dogs on the game over screen
- [x] Add phrases "The milk'd been drinked!" when collecting the milk and "The milk escaped!" when not collecting the milk
- [x] Add splash screen
- [x] Try add cpu-based particles

##### Future features:

- [ ] Update bevy to 0.11.0
- [ ] Speed
- [ ] Health
- [ ] Shield
- [ ] Water gun
- [ ] Dog locator
- [ ] Implement logic "Smart dog"
- [ ] Rotating antenna
- [ ] Add antenna, emitting particles on milk detection
- [ ] Fish boss
