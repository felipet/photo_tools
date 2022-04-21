//! photo_tools command-line tool
//!
//! This tool helps managing a big collection of photographies, having both: developed images and
//! raw files.
use argparse::{ArgumentParser, Print, Store, StoreTrue};
use chrono::prelude::*;
use photo_tools::{delete_photos, make_path, photo_database, PhotoDir};

struct Options {
    verbose: bool,
    photo_dir: String,
    img_ext: String,
    raw_ext: String,
    photo_del: bool,
    filter: String,
}

fn main() {
    // Variables to store the commandline arguments/options
    let mut options = Options {
        verbose: false,
        photo_dir: String::new(),
        photo_del: false,
        raw_ext: String::from("RAF"),
        img_ext: String::from("JPG"),
        filter: String::new(),
    };

    // Argument parsing environment
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(
            "Photo management tool: This tool finds orphan files in a folder and deletes them.",
        );
        // Filter argument: If JPG is used, the program will delete RAW files having no JPG
        // equivalent in the same directory, viceversa when using RAF as filter.
        ap.refer(&mut options.filter)
            .required()
            .add_argument("FILTER", Store, "IMG or RAW");
        ap.add_option(
            &["-V", "--version"],
            Print(env!("CARGO_PKG_VERSION").to_string()),
            "Show version",
        );
        // Option to set the extension of the camera raw files. Fujifilm uses RAF (default value).
        ap.refer(&mut options.raw_ext).add_option(
            &["-r", "--rawext"],
            Store,
            "Extension of the raw files (RAF by default)",
        );
        // Option to set the extension of the camera image files. Fujifilm uses JPG (default value).
        ap.refer(&mut options.img_ext).add_option(
            &["-j", "--photoext"],
            Store,
            "Extension of the image files (JPG by default)",
        );
        // Path of the directory containing the photography files.
        ap.refer(&mut options.photo_dir)
            .add_option(&["-p", "--path"], Store, "Path of the folder");
        // Option to enable directly deleting the selected files. Moved to a new folder instead.
        ap.refer(&mut options.photo_del).add_option(
            &["-d", "--delete"],
            StoreTrue,
            "Delete filtered photos",
        );
        // Extra info to the console.
        ap.refer(&mut options.verbose).add_option(
            &["-v", "--verbose"],
            StoreTrue,
            "Enable verbose mode",
        );

        ap.parse_args_or_exit();
    }

    if options.verbose {
        let dt = Local::now();
        println!(
            "photo_tools - log - {}\n",
            dt.format("%Y-%m-%d %H:%M:%S").to_string()
        );
        println!("\tFiltering orphan files by {}!", options.filter);
    }

    // Extract the path of the photo directory
    let photo_dir = make_path(&options.photo_dir, options.verbose).unwrap_or_else(|err| {
        if err.raw_os_error() == Some(2) {
            println!("Error: Not a valid path: {}", options.photo_dir);
        } else {
            println!("{:?}", err);
        }
        std::process::exit(err.raw_os_error().unwrap());
    });

    let photo = PhotoDir {
        path: photo_dir,
        filter: options.filter,
        raw_ext: options.raw_ext,
        img_ext: options.img_ext,
    };

    let photo_db = photo_database(&photo, options.verbose).unwrap();

    match delete_photos(&photo, &photo_db, options.photo_del, options.verbose) {
        Ok(_) => println!("All done!"),
        Err(error) => println!("{:?}", error),
    };
}
