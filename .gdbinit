target remote :3333
load
monitor tpiu config internal /tmp/itm.fifo uart off 8000000
break main
continue
