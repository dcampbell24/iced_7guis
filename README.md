# 7 GUIs Iced

An implementation of [7 GUIs](https://eugenkiss.github.io/7guis/) using [iced](https://iced.rs/).

* [X] Counter
* [X] Temperature Converter
* [X] Flight Booker
* [X] Timer
* [X] CRUD
* [ ] Circle Drawer
* [ ] Cells

## Notes

### Flight Booker

I didn't know how to disable a widget or turn the background red. So instead, book turns red and
input is ignored.

### Timer

For some reason when the duration is brought to a minimum the elapsed time progress bar becomes
empty.

### CRUD

I once again have the issue with disabling widgets. The dynamic layout is also a bit funny. When
items are created they fill the space and enlarge it. I could not figure out how to have a fixed
layout. I also used radio buttons instead of normally selecting a name. 