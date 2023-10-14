# rite-emu - A CHIP-8 Emulator

Since the CHIP-8 is the rite of passage emulator. Get it? ...Get it?

## Building
This is a rust project, so do whatever you do with that on your system. Refer to [rustup.rs](rustup.rs) and the rust docs for platform specific instructions. 

## Controls
This project uses the standard for COSMAC VIP keypad integration for modern keyboard layouts.

As an example, we use the U.S. English QWERTY keyboard to correspond to COSMAC VIP keys below:

**U.S. English QWERTY:**
```
1 2 3 4
Q W E R
A S D F
Z X C V
```

**COSMAC VIP keypad:**
```
1 2 3 C
4 5 6 D
7 8 9 E
A 0 B F
```

## Things left to be done
- Beeping when sound timer is above 0, although I'm not a huge fan of that feature.
- Configurations for other implementation features, e.g. the functioning of instructions like `8XY6`

## Awesome people and resources
Everybody in the [emudev discord server](<https://discord.gg/dkmJAes>), [r/EmuDev](<https://www.reddit.com/r/EmuDev/>), and [Tobias' lovely blog](<https://tobiasvl.github.io/blog/write-a-chip-8-emulator/>).