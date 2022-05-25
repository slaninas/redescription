# Redescription
[The Binding of Isaac: Rebirth](https://store.steampowered.com/app/250900/The_Binding_of_Isaac_Rebirth/) tool that displays informations about items on the screen, written in Rust.

Unlike following DLCs the Rebirth does not support mods, this tool uses [OpenCV's](https://opencv.org/)
[template matching](https://docs.opencv.org/4.x/df/dfb/group__imgproc__object.html#ga586ebfb0a7fb604b35a23d85391329be) instead and displays the info to the 
terminal, see [Limitations](https://github.com/slaninas/rebirth-descriptions/new/master?readme=1#limitations).

The descriptions are scraped from the [Platinum God](https://platinumgod.co.uk/rebirth).

This is an alternative for those who want to play Rebirth without DLCs.
If you play Afterbirth+ or Repentance, I highly recommend you to use [[AB+|Rep] External item descriptions](https://steamcommunity.com/sharedfiles/filedetails/?id=836319872).

<img src="https://github.com/slaninas/redescription/blob/master/screenshots/deck.png">
<img src="https://github.com/slaninas/redescription/blob/master/screenshots/shop.png">

## Limitations
Since it works by taking screenshots and running [match_template](https://docs.rs/opencv/latest/opencv/imgproc/fn.match_template.html)
for every item in the game, this tool in not as accurate as tools that use Isaac's modding API in the Afterbirth+ or Repentance DLCs.

The inaccuracies come from the fact that the items on the pedestals are animated (squeezing animation for pedestal items)
and because the template matching is done in grayscale
(if an item is detected and there is simillar one that differs only in color scheme it will be shown as well).

Occasionally there are false matches.

Two monitor setup is needed and the game must run on primary screen in fullscreen mode, descriptions are printed to the terminal on the secondary monitor.

## How to Build&Run
- [Get OpenCV](https://github.com/twistedfall/opencv-rust#getting-opencv) 4.5.4
- [Unpack game's resources](https://www.reddit.com/r/themoddingofisaac/comments/3rw1a7/modding_tutorial_part_1/)

### Linux
```
git clone https://github.com/slaninas/redescription
cd redescription
cargo run --release path_to_unpacked_resources
```

First time you run the exacutable the items descriptions are downloaded into `descriptions.json`. If you want to redownload, remove this file and run the executable again.

You can get the executable [here](https://github.com/slaninas/redescription/releases) section but you still need to install the OpenCV library (dynamic linking).


### Windows
Method similar to the Linux one should work.

