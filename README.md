# rusty-macros
My attempt at making windows cmd macros/aliases using rust... and also my intro to the rust language.

I could easily do this in C++ or C or python, but what's the fun in that? This way I get to jump on the hype train and see what this newfangled 'fireflower' is all about. Will I turn into fire mario? idk, but it's worth a shot.

## List of Macros:
* [X] ls
* [ ] rm
* [ ] mv
* [ ] cp
* [ ] touch
* [X] cat
* [ ] wget

## Usage:
Build the .rs programs with rustc as shown below. For me, the program is slow to run the first time, but is fast on subsequent runs.

If you do not want to build the macros from source, you can access [prebuilt ones hosted on my onedrive](https://kennesawedu-my.sharepoint.com/:u:/g/personal/agrave15_students_kennesaw_edu/Ed3Oll0jkglDp1T9QGB-HmABDtcJX1-9GfFMZN3bynBYBw?e=gsKUJ5)

Once you have the .exe's, move them all to one folder (typically named ```bin```), and add the folder to your environment variables. 
(in windows 10, 
1. search for 'edit environment variables for your account'
2. double click on Path in the user variables (top box)
3. click New and add the path to your ```bin``` folder
)

Then you can restart cmd, and you should be able to use the macros from any folder!

## Notes:
I could use cargo, it is a nice system with plenty of functionality, but my use case is pretty simple, so instead, I am just using rustc:
```
rustc -C opt-level=3 <prog_name.rs>
```
