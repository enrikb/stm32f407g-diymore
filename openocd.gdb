target extended-remote :3333

# print demangled symbols
set print asm-demangle on

# set backtrace limit to not have infinite backtrace loops
set backtrace limit 32

# detect unhandled exceptions, hard faults and panics
break DefaultHandler
break HardFault
break rust_begin_unwind

# *try* to stop at the user entry point (it might be gone due to inlining)
break main

# send captured ITM to the file itm.txt
# if you have a monitor supporting SWO, connect it to PB3 to make this work
# 168000000 is the core clock frequency
# monitor tpiu config internal itm.txt uart off 168000000

# OR: make the microcontroller SWO (PB3) pin output compatible with UART (8N1)
# 4000000 is the frequency of the SWO pin
# the uart has to support a baudrate of 4Mbit/s
# it can be configured using `stty -F <tty> 4000000`
# higher rates typically need special setup, at least on Linux
monitor tpiu config external uart off 168000000 4000000

# enable ITM port 1
monitor itm port 1 on

load

# start the process but immediately halt the processor
stepi