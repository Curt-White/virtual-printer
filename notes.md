#### Commands
* Single, Double, Triple Identifying commands
* Commands can have numbers representing bit options
* Hex 10 04 n [a] - represents an optional value a depending on the value of a
* Sub commands where function number is value directly after ident (DLE DC4 fn m t)
* Command has standard arg for subset of functions where fn number is not directly following the original command (ESC ( A pL pH fn n c t)

#### Modes
##### Standard Mode
* Prints whenever it receives printing or feed
instructions

##### Page Mode
* All of the data is stored in memory until the printer
receives an FF commands where the data is then printed. Upon
executing this FF command the printer returns to the standard
mode.
