use std::collections::HashMap;
use std::fs::{self, DirBuilder};
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Photo {
    pub file_name: String,
    pub has_raw: bool,
    pub has_jpg: bool,
}

#[derive(Debug)]
pub struct PhotoDir {
    pub path: PathBuf,
    pub filter: String,
    pub raw_ext: String,
    pub img_ext: String,
}

/// Checks that a path string is valid, and that the user has RW privileges in it
///
/// # Arguments:
/// - path: a String containing the path to a directory containing photos. An empty \
///   String can be passed to the function to indicate the path './'.
/// - verbose: enable extra debug information
///
/// # Returns:
/// - *on success*: a (PathBuf)[https://doc.rust-lang.org/std/path/struct.PathBuf.html] \
///   instance.
/// - *on failure*: a io::Result indicating the source of the error.
///
/// # Example
/// ```rust
/// let mypath = String::from("./");
/// let verbose = false;
/// let photo_dir = make_path(mypath, verbose);
/// ```
pub fn make_path(path: &String, verbose: bool) -> io::Result<PathBuf> {
    let mut new_path = PathBuf::new();

    //First detect if a path was given to the tool
    if path.is_empty() {
        new_path.push(fs::canonicalize("./").unwrap()); // This call can't fail
    } else {
        // Is the path relative or absolute?
        if &path[..1] == "." {
            new_path.push(fs::canonicalize(path)?);
        } else {
            new_path.push(path);
        }
    }
    if verbose {
        println!("\tUsing {} as the photo source directory.", path.as_str());
    }

    Ok(new_path)
}

/// Build a photography data base from the files included in a directorycar
///
/// # Details
/// This function lists all the files included in a directory, and makes a
/// database of those which correspond to photography files. Photos are detected
/// as RAW files (using the given RAW extension), or IMG files, i.e. developed images
/// from the RAW files (using the given IMG extension).
///
/// # Arguments
/// - path: a String containing the path to a directory containing photos. An empty \
///   String can be passed to the function to indicate the path './'.
/// - verbose: enable extra debug information
/// # Returns:
/// - *on success*: a `<https://doc.rust-lang.org/std/path/struct.PathBuf.html>` instance.
/// - *on failure*: a io::Result indicating the source of the error.
/// # Example
/// ```rust
/// let mypath = String::from("./");
/// let verbose = false;
/// let photo_dir = make_path(mypath, verbose);
/// ```
pub fn photo_database(
    photo_dir: &PhotoDir,
    verbose: bool,
) -> Result<HashMap<String, Photo>, io::Error> {
    // List the files in the directory
    let dir_list = fs::read_dir(&photo_dir.path)
        .unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<PathBuf>, io::Error>>()?;

    // HashMap for the photo data base.
    let mut photo_db: HashMap<String, Photo> = HashMap::new();

    // Iterate over the files in the directory
    for file in dir_list {
        // Omit folders - non recursive algorithm
        if file.is_file() {
            // Extract the file extension
            let extension = file.extension().unwrap().to_str().unwrap();
            // Detect if the file is photo file
            if extension == photo_dir.raw_ext || extension == photo_dir.img_ext {
                // Extract the file name, no path, no extension.
                let filename = String::from(file.file_stem().unwrap().to_str().unwrap());

                // Is the current file a RAW or a IMG file?
                let raw = extension == photo_dir.raw_ext.as_str();
                let jpg = extension == photo_dir.img_ext.as_str();

                // Now, let's build the key for the data base using the extension
                // marked by the filter.
                let mut file_path = file.clone();
                // Pop the file name from the complete file path
                file_path.pop();
                let mut file_path = String::from(file_path.to_str().unwrap());
                file_path += "/";
                file_path += filename.as_str();
                file_path += ".";

                // complete the file name using the filter extension
                if photo_dir.filter.as_str() == "RAW" {
                    file_path += photo_dir.raw_ext.as_str();
                } else {
                    file_path += photo_dir.img_ext.as_str();
                }

                // Detect whether the photo file was already present in the DB
                // If so, either the RAW or IMG file was listed previously.
                let was_in = photo_db.get(&file_path);

                match was_in {
                    Some(_) => {
                        // Have we found the pair file?
                        if (photo_db[&file_path].has_raw && jpg)
                            || (photo_db[&file_path].has_jpg && raw)
                        {
                            photo_db.insert(
                                file_path.clone(),
                                Photo {
                                    file_name: filename,
                                    has_raw: true,
                                    has_jpg: true,
                                },
                            );
                        }
                    }
                    None => {
                        photo_db.insert(
                            file_path.clone(),
                            Photo {
                                file_name: filename.clone(),
                                has_raw: raw,
                                has_jpg: jpg,
                            },
                        );
                    }
                }
            }
        }
    }

    if verbose {
        println!("\tFound {} photo files in the folder.", photo_db.len());
    }

    Ok(photo_db)
}

/// Move or delete the photography files marked
///
/// # Arguments:
/// - photo_dir: Struct 'PhotoDir'
///   String can be passed to the function to indicate the path './'.
/// - verbose: enable extra debug information
/// # Returns:
/// - *on success*: a `std::path::PathBuf' instance.
/// - *on failure*: a io::Result indicating the source of the error.
/// # Example
/// ```rust
/// let mypath = String::from("./");
/// let verbose = false;
/// let photo_dir = make_path(mypath, verbose);
/// ```
pub fn delete_photos(
    photo_dir: &PhotoDir,
    photo_db: &HashMap<String, Photo>,
    delete: bool,
    verbose: bool,
) -> io::Result<()> {
    const DELETE_DIR_NAME: &str = "to_delete/";

    // Create the directory for the discarded files
    let mut remove_dir = photo_dir.path.clone();
    remove_dir.push(DELETE_DIR_NAME);

    let delete_file_exists = match fs::metadata(&remove_dir) {
        Ok(some) => some.is_dir(),
        _ => false,
    };

    if !delete_file_exists {
        let _builder = DirBuilder::new()
            .recursive(false)
            .create(&remove_dir)
            .unwrap_or_else(|err| {
                println!("The directory could not be created");
                std::process::exit(err.raw_os_error().unwrap());
            });
    }

    if verbose && !delete {
        println!(
            "\tFiles to be deleted by the user are located at: {}/{}",
            photo_dir.path.as_path().to_str().unwrap(),
            DELETE_DIR_NAME,
        );
    }

    // Iterate over the photo DB and detect whether a file should be deleted or not
    let delete_path = String::from(remove_dir.as_os_str().to_str().unwrap());

    for (file, val) in photo_db {
        if (photo_dir.filter.as_str() == "RAW" && (val.has_raw && !val.has_jpg))
            || (photo_dir.filter == "IMG" && (val.has_jpg && !val.has_raw))
        {
            let mut delete_file = delete_path.clone();
            delete_file.push_str(val.file_name.as_str());
            delete_file.push_str(".");
            if photo_dir.filter.as_str() == "RAW" {
                delete_file.push_str(photo_dir.raw_ext.as_str());
            } else {
                delete_file.push_str(photo_dir.img_ext.as_str());
            }
            fs::copy(file.as_str(), delete_file.as_str())?;
            fs::remove_file(file.as_str())?;
            if verbose {
                println!("\tFile {} moved to to_delete folder", file.as_str());
            }
        }
    }

    if delete {
        fs::remove_dir_all(delete_path.as_str())?;
        if verbose {
            println!("Folder with discarded files deleted!");
        }
    } else {
        println!(
            "The folder {} contains the discarded files.",
            delete_path.as_str()
        );
    }
    Ok(())
}
