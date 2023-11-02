This fixes xkb's SHIT.

Are you using ibus and xkb? Do you have your custom xkb layout? Do you wanna use it in ibus? Well, good luck, you are fucked.

There is NO way to have a user-specific xkb config. 95% of the tutorials [0] tell you to edit the `/usr/share/X11/xkb` system files, which is just lunatic [1]. [The rest 5%](https://who-t.blogspot.com/2020/09/user-specific-xkb-configuration-putting.html) that teach you to do it user-specifically DOES NOT WORK [2].

This repo should be run as a one-shot service as root. It parses the `/usr/share/X11/xkb/rules/evdev.xml` and inserts custom `<layout>` entry if it doesn't exist. It also copies custom symbols to `/usr/share/X11/xkb/symbols/` should it find it missing.



---



- [0] Including a tutorial from the official looking official Ubuntu docs website
- [1] Good luck adding it back when a system package update wipes your changes
- [2] It doesn't work on my Arch linux and didn't work for my other friend. `setxkbmap myuserlayout` simply pukes with "cannot load layout" or something, and ibus doesn't pick it up. What's funny is, if you managed to fuck up the system `evdev.xml`, ibus will actually pick up the `~/.config/xkb/rules/evdev.xml` (but it's still not useable).

