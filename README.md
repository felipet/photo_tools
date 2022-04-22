# photo_tools

A simple command line tool written in Rust to help managing my photo collection.

## Use case
The tool is meant to detect pictures that either don't have a developed file
from a raw file, or raw file corresponding to the developed image.
This helps my custom workflow when I analyse my photos taken from my Fuji
camera.

Usually, I copy all the files from a session to my computer (that makes faster
opening and working with images). Then, I start a quick visualization of the
developed files produced by the camera (I usually take my shots using
raw + developed image files). This way, I can easily delete blurry, unfocused
or ugly pictures. The main problem of this workflow is that I do that only
using the JPG files. That leads in a set of orphan raw files, i.e. the raw
files from the deleted JPG files.

This tool analyses a directory and finds those files. Then, it can directly
deleted them or move them to a particular subdirectory, which can be manually
deleted later. Though that is my main use-case, I sometimes open my collection
of new photos from Darktable, thus I mainly focus on raw files. This time,
my directory ends full of JPG files whose source raw file was deleted from
Darktable. The tool can help locating these files as well.

## Example of use
Let's consider a folder having these files:

```bash
  ➜  tree -L 1
  .
  ├── DSCF5341.JPG
  ├── DSCF5356.JPG
  ├── DSCF5356.RAF
  ├── DSCF5357.JPG
  ├── DSCF5357.RAF
  ├── DSCF5358.JPG
  ├── DSCF5358.RAF
  └── DSCF5359.RAF
  0 directories, 8 files
```

If our aim is to get rid of the developed images (JPGs) that doesn't have a
paired RAW file, we shall use the tool this way:

```bash
$ photo_tools IMG -p <path to the directory>
```

That will create a subdirectory named _to_delete_ inside the photo directory
containing all the developed files that have no raw pair.

By default, the tool doesn't perform any delete, but moves all the files to a
subdirectory instead. The argument **-d** shall be passed to the tool for a
direct delete of the files.

## Matching the tool config to a particular camera brand

I'm a fuji lover, so I made the default configuration for the file extensions
used by Fujifilm, i.e. _RAF_ for the raw files, and _JPG_ for the developed
files by the camera.

If you use a different camera manufacturer, you might need to adapt the
extension for the raw/developed files. In order to so, use arguments **-r**
and/or **-j**:

```bash
$ photo_tools RAW -r NEF
```

The previous example will run the tool in the current directory (path = ./) and
will consider a raw files if it's named this way: `ABCD.NEF`.

## How to install the program

Stable releases of the tool can be found in the _Releases_ section of the
(GitHub page)[https://github.com/felipet/photo_tools/releases]. If a
compiled version for your system is offered there, just download it and
place it in a system directory included in your **PATH** variable.

If not, you can easily compile the tool using **Cargo**:

```bash
$ cd photo_tools
$ cargo build --release
$ cp target/release/photo_tools   ~/.local/bin/
```
