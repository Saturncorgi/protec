[![Rust](https://github.com/Saturncorgi/protec/actions/workflows/rust.yml/badge.svg)](https://github.com/Saturncorgi/protec/actions/workflows/rust.yml)<br>
Notes on stuff you'll need to edit: <br>
Line 50 contains the password bytes as raw keycodes, you can look up the conversion<br>
Line 48 contains the path to your keyboard<br>
Line 111 may require some changes. Its a list of regexs that match the input devices to be disabled EG (pad\nKernel: *).\* matches the /dev/input/event my touchpad is connected too which changes each boot<br>
Mouse should just work but idk
<br>
<hr>
for arch linux makepkg -fis
