[![Rust](https://github.com/Saturncorgi/protec/actions/workflows/rust.yml/badge.svg)](https://github.com/Saturncorgi/protec/actions/workflows/rust.yml)<br>
Yell at anyone who dares touch your computer when you're not there <br>
Notes on stuff you'll need to edit: <br>
Line 50 contains the password bytes as raw keycodes, you can look up the conversion<br>
Line 48 contains the path to your keyboard<br>
Line 111 may require some changes. It's a list of regex's that match the input devices to be disabled EG (pad\nKernel: *).\* matches the /dev/input/event my touchpad is connected too which changes each boot<br>
Mouse should just work but IDK
<br>
<hr>
For Arch Linux makepkg -fis<br>
Otherwise cargo build --release then copy the binary into bin