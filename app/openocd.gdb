target extended-remote :3333
monitor reset halt
monitor arm semihosting enable 
load
continue