# Connect to the openocd GDB server
target extended-remote :3333

# Prevent infinite loop on panic!()
set backtrace limit 32

# Reset the target device before running (using openocd)
monitor reset halt

# Load the firmware onto the device
load

# enable the tui
tui enable
layout split
