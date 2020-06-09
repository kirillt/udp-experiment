Use `collect` utility for listening a UDP port and reporting amount of received datagrams with selected interval.\
Use `stress` utility to send as much datagrams as you want into selected UDP port.\
(Both binaries have `--help` information about parameters.)

You can specify sending interval either directly or via DPS ("datagrams per second").\
In case you specify too short interval, datagrams will be batched and sent in "burst" mode.
